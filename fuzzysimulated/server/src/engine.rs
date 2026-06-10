use logicfuzzy_academic::{
    gaussmf, trapmf, trimf,
    rule::{Antecedent, Connector, RuleBuilder},
    tsk::{TskConsequent, TskEngine, TskRule},
    pso::{PsoConfig, PsoOptimizer},
    var_svg,
    ExplainReport, FuzzyVariable, MamdaniEngine, MembershipFn, Term, Universe,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TermInfo {
    pub term_id: Uuid,
    pub label: String,
    pub mf_type: String,
    pub params: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct VarInfo {
    pub var_id: Uuid,
    pub name: String,
    pub role: String,
    pub universe_min: f64,
    pub universe_max: f64,
    pub resolution: usize,
    pub terms: Vec<TermInfo>,
}

#[derive(Debug, Clone)]
pub struct RuleInfo {
    pub rule_text: String,
    pub weight: f64,
}

fn mf_from_params(mf_type: &str, params: &[f64]) -> Option<MembershipFn> {
    match mf_type {
        "trimf" if params.len() >= 3 => {
            Some(MembershipFn::Trimf([params[0], params[1], params[2]]))
        }
        "trapmf" if params.len() >= 4 => {
            Some(MembershipFn::Trapmf([params[0], params[1], params[2], params[3]]))
        }
        "gaussmf" if params.len() >= 2 => {
            Some(MembershipFn::Gaussmf { mean: params[0], sigma: params[1] })
        }
        _ => None,
    }
}

pub fn membership(value: f64, mf_type: &str, params: &[f64]) -> f64 {
    match mf_type {
        "trimf" if params.len() >= 3 => trimf(value, params[0], params[1], params[2]),
        "trapmf" if params.len() >= 4 => trapmf(value, params[0], params[1], params[2], params[3]),
        "gaussmf" if params.len() >= 2 => gaussmf(value, params[0], params[1]),
        _ => 0.0,
    }
}

fn find_in_text(text: &str, candidates: &[String]) -> Option<String> {
    let lower = text.to_lowercase();
    let mut best: Option<(String, usize)> = None;
    for c in candidates {
        let cl = c.to_lowercase();
        if let Some(pos) = lower.find(&cl) {
            let is_better = best.as_ref().is_none_or(|(_, p)| pos < *p);
            if is_better {
                best = Some((c.clone(), pos));
            }
        }
    }
    best.map(|(s, _)| s)
}

fn extract_term(text: &str, var_name: &str) -> String {
    let separators = [" = ", " é ", " e ", "= ", "= ", " é", " =", "= "];
    let lower = text.to_lowercase();
    let vn_lower = var_name.to_lowercase();
    if let Some(pos) = lower.find(&vn_lower) {
        let after = &text[pos + vn_lower.len()..];
        for sep in &separators {
            if let Some(stripped) = after.strip_prefix(sep) {
                let rest = stripped.trim();
                let word_end = rest
                    .find([' ', ',', '.'])
                    .unwrap_or(rest.len());
                return rest[..word_end].to_string();
            }
        }
    }
    String::new()
}

pub fn parse_rule_conditions(
    text: &str,
    var_names: &[String],
) -> Vec<(String, String)> {
    let normalized = text
        .replace("ENTÃO", "ENTAO")
        .replace("THEN", "ENTAO");
    let parts: Vec<&str> = normalized.splitn(2, "ENTAO").collect();
    if parts.len() < 2 {
        return Vec::new();
    }
    let mut conditions = Vec::new();
    let ant_text = parts[0].strip_prefix("SE").unwrap_or(parts[0]).trim();
    let ant_parts: Vec<&str> = ant_text
        .split(" E ")
        .flat_map(|s| s.split(" AND "))
        .collect();
    for part in &ant_parts {
        let trimmed = part.trim();
        if let Some(var_name) = find_in_text(trimmed, var_names) {
            let term = extract_term(trimmed, &var_name);
            if !term.is_empty() {
                conditions.push((var_name, term));
            }
        }
    }
    let cons_text = parts[1].trim();
    if let Some(var_name) = find_in_text(cons_text, var_names) {
        let term = extract_term(cons_text, &var_name);
        if !term.is_empty() {
            conditions.push((var_name, term));
        }
    }
    conditions
}

pub fn compute_rule_activation(
    rule_text: &str,
    variables: &HashMap<String, &VarInfo>,
    inputs: &HashMap<String, f64>,
) -> f64 {
    let var_names: Vec<String> = variables.keys().cloned().collect();
    let conditions = parse_rule_conditions(rule_text, &var_names);
    if conditions.is_empty() {
        return 0.0;
    }
    let mut alpha = 1.0_f64;
    for (var_name, term_label) in &conditions {
        if let Some(var_info) = variables.get(var_name) {
            if let Some(term) = var_info.terms.iter().find(|t| t.label.eq_ignore_ascii_case(term_label)) {
                if let Some(input) = inputs.get(var_name) {
                    let mu = membership(*input, &term.mf_type, &term.params);
                    alpha = alpha.min(mu);
                } else {
                    alpha = 0.0;
                }
            } else {
                alpha = 0.0;
            }
        } else {
            alpha = 0.0;
        }
        if alpha <= 0.0 {
            break;
        }
    }
    alpha
}

fn build_engine(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    inputs: &HashMap<String, f64>,
) -> MamdaniEngine {
    let var_names: Vec<String> = variables.iter().map(|v| v.name.clone()).collect();
    let mut engine = MamdaniEngine::new();

    for var in variables {
        let resolution = var.resolution.max(2);
        let uni = Universe::new(var.universe_min, var.universe_max, resolution);
        let mut fv = FuzzyVariable::new(&var.name, uni);
        for term in &var.terms {
            if let Some(mf) = mf_from_params(&term.mf_type, &term.params) {
                fv.add_term(Term::new(&term.label, mf));
            }
        }
        match var.role.as_str() {
            "antecedent" | "input" => engine.add_antecedent(fv),
            "consequent" | "output" => engine.add_consequent(fv),
            _ => {}
        }
    }

    for rule in rules {
        let conditions = parse_rule_conditions(&rule.rule_text, &var_names);
        if conditions.len() < 2 {
            continue;
        }
        let ante = &conditions[..conditions.len() - 1];
        let conseq = &conditions[conditions.len() - 1];

        let mut builder = RuleBuilder::new();
        builder = builder.when(&ante[0].0, &ante[0].1);
        for (var_name, term_label) in &ante[1..] {
            builder = builder.and(var_name, term_label);
        }
        builder = builder.then(&conseq.0, &conseq.1);
        if (rule.weight - 1.0).abs() > f64::EPSILON {
            builder = builder.weight(rule.weight);
        }
        engine.add_rule(builder.build());
    }

    for (name, value) in inputs {
        let _ = engine.set_input(name, *value);
    }

    engine
}

pub fn evaluate_mamdani(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    inputs: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let engine = build_engine(variables, rules, inputs);
    match engine.compute() {
        Ok(outputs) => outputs,
        Err(_) => variables
            .iter()
            .filter(|v| v.role == "consequent" || v.role == "output")
            .map(|v| (v.name.clone(), (v.universe_min + v.universe_max) / 2.0))
            .collect(),
    }
}

/// Avalia um sistema usando inferência TSK (Takagi-Sugeno-Kang) (UC18).
pub fn evaluate_tsk(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    inputs: &HashMap<String, f64>,
    coeffs: &HashMap<String, Vec<f64>>,
) -> HashMap<String, f64> {
    let var_names: Vec<String> = variables.iter().map(|v| v.name.clone()).collect();
    let ant_vars: Vec<&VarInfo> = variables.iter().filter(|v| v.role == "antecedent" || v.role == "input").collect();

    let mut engine = TskEngine::new();
    for var in &ant_vars {
        let resolution = var.resolution.max(2);
        let uni = Universe::new(var.universe_min, var.universe_max, resolution);
        let mut fv = FuzzyVariable::new(&var.name, uni);
        for term in &var.terms {
            if let Some(mf) = mf_from_params(&term.mf_type, &term.params) {
                fv.add_term(Term::new(&term.label, mf));
            }
        }
        engine.add_antecedent(fv);
    }

    for var in variables {
        if var.role == "consequent" || var.role == "output" {
            engine.add_output(&var.name, Universe::new(var.universe_min, var.universe_max, 101));
        }
    }

    let sorted_ant_names: Vec<String> = {
        let mut names: Vec<String> = ant_vars.iter().map(|v| v.name.clone()).collect();
        names.sort();
        names
    };

    for rule in rules {
        let conditions = parse_rule_conditions(&rule.rule_text, &var_names);
        if conditions.len() < 2 { continue; }
        let ante = &conditions[..conditions.len() - 1];
        let conseq = &conditions[conditions.len() - 1];

        let mut rule_antecedents = Vec::new();
        for (vname, tlabel) in ante {
            rule_antecedents.push(Antecedent::new(vname, tlabel));
        }
        if rule_antecedents.is_empty() { continue; }

        let coeff_key = format!("{}_{}", conseq.0, conseq.1);
        let coeffs_for_rule = coeffs.get(&coeff_key)
            .cloned()
            .unwrap_or_else(|| {
                let mut c = vec![0.0; sorted_ant_names.len() + 1];
                if let Some(first) = c.first_mut() { *first = 50.0; }
                c
            });

        engine.add_rule(TskRule::new(
            rule_antecedents,
            Connector::And,
            vec![TskConsequent::new(&conseq.0, coeffs_for_rule)],
        ));
    }

    for (name, value) in inputs {
        let _ = engine.set_input(name, *value);
    }

    match engine.compute() {
        Ok(outputs) => outputs,
        Err(_) => variables
            .iter()
            .filter(|v| v.role == "consequent" || v.role == "output")
            .map(|v| (v.name.clone(), (v.universe_min + v.universe_max) / 2.0))
            .collect(),
    }
}

/// Gera relatório de diagnóstico explicativo de uma simulação Mamdani (UC20).
pub fn generate_diagnostic(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    inputs: &HashMap<String, f64>,
) -> Result<serde_json::Value, String> {
    let engine = build_engine(variables, rules, inputs);
    let _ = engine.compute();
    let report = engine.explain().unwrap_or(ExplainReport {
        fuzzification: Vec::new(),
        rule_firings: Vec::new(),
        outputs: HashMap::new(),
        rules_fired: 0,
        rules_skipped: 0,
    });

    Ok(serde_json::json!({
        "fuzzification": report.fuzzification.iter().map(|fv| serde_json::json!({
            "variable": fv.variable,
            "crisp_input": fv.crisp_input,
            "term_degrees": fv.term_degrees.iter().map(|(t, v)| serde_json::json!({"term": t, "mu": v})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "rule_firings": report.rule_firings.iter().map(|rf| serde_json::json!({
            "rule_text": rf.rule_text,
            "firing_degree": rf.firing_degree,
            "fired": rf.fired,
        })).collect::<Vec<_>>(),
        "outputs": serde_json::json!(report.outputs),
        "rules_fired": report.rules_fired,
        "rules_skipped": report.rules_skipped,
    }))
}

/// Gera um gráfico 3D SVG de superfície de controle para sistemas com exatamente 2 entradas.
/// X e Y varrem os universos das duas variáveis antecedentes, Z é o valor defuzzificado.
/// Usa projeção isométrica com gradiente de cor mapeado em Z.
pub fn generate_surface_svg_3d(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    x_name: &str,
    y_name: &str,
    out_name: &str,
    resolution: usize,
) -> String {
    let x_res = resolution.max(5).min(50);
    let y_res = resolution.max(5).min(50);

    let x_var = match variables.iter().find(|v| v.name == x_name) {
        Some(v) => v,
        None => return r#"<svg width="400" height="200" viewBox="0 0 400 200"><text x="200" y="100" text-anchor="middle" fill="red" font-size="12">Variável X não encontrada</text></svg>"#.to_string(),
    };
    let y_var = match variables.iter().find(|v| v.name == y_name) {
        Some(v) => v,
        None => return r#"<svg width="400" height="200" viewBox="0 0 400 200"><text x="200" y="100" text-anchor="middle" fill="red" font-size="12">Variável Y não encontrada</text></svg>"#.to_string(),
    };

    let x_lo = x_var.universe_min;
    let x_hi = x_var.universe_max;
    let y_lo = y_var.universe_min;
    let y_hi = y_var.universe_max;

    let mut grid: Vec<Vec<[f64; 3]>> = Vec::with_capacity(x_res);
    let mut z_min = f64::MAX;
    let mut z_max = f64::MIN;

    for xi in 0..x_res {
        let mut row = Vec::with_capacity(y_res);
        for yi in 0..y_res {
            let xv = x_lo + (xi as f64 / (x_res - 1) as f64) * (x_hi - x_lo);
            let yv = y_lo + (yi as f64 / (y_res - 1) as f64) * (y_hi - y_lo);
            let mut inputs = HashMap::new();
            inputs.insert(x_name.to_string(), xv);
            inputs.insert(y_name.to_string(), yv);
            let outputs = evaluate_mamdani(variables, rules, &inputs);
            let zv = outputs.get(out_name).copied().unwrap_or_else(|| {
                outputs.values().copied().sum::<f64>() / outputs.len().max(1) as f64
            });
            z_min = z_min.min(zv);
            z_max = z_max.max(zv);
            row.push([xv, yv, zv]);
        }
        grid.push(row);
    }

    let z_range = (z_max - z_min).max(1.0);

    let w = 620.0;
    let h = 480.0;
    let pad = 70.0;
    let plot_w = w - 2.0 * pad;
    let plot_h = h - 2.0 * pad;

    let scale = f64::min(plot_w, plot_h) / 2.5;

    let cx = w / 2.0;
    let cy = h / 2.0 + 30.0;

    let angle = std::f64::consts::PI / 6.0;
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let norm_x = |v: f64| -> f64 { (v - x_lo) / (x_hi - x_lo).max(1.0) };
    let norm_y = |v: f64| -> f64 { (v - y_lo) / (y_hi - y_lo).max(1.0) };
    let norm_z = |v: f64| -> f64 { (v - z_min) / z_range };

    let project = |xv: f64, yv: f64, zv: f64| -> (f64, f64) {
        let nx = norm_x(xv) * 2.0 - 1.0;
        let ny = norm_y(yv) * 2.0 - 1.0;
        let nz = norm_z(zv);
        let sx = (nx - ny) * cos_a * scale;
        let sy = (nx + ny) * sin_a * scale - nz * scale * 0.8;
        (cx + sx, cy - sy)
    };

    let heat_color = |t: f64| -> String {
        let t = t.clamp(0.0, 1.0);
        let (r, g, b) = if t < 0.25 {
            let v = (t * 4.0 * 255.0) as u8;
            (0, v, 255)
        } else if t < 0.5 {
            let v = ((t - 0.25) * 4.0 * 255.0) as u8;
            (0, 255, (255 - v))
        } else if t < 0.75 {
            let v = ((t - 0.5) * 4.0 * 255.0) as u8;
            (v, 255, 0)
        } else {
            let v = ((t - 0.75) * 4.0 * 255.0) as u8;
            (255, (255 - v), 0)
        };
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    };

    let mut faces: Vec<(f64, String)> = Vec::new();

    for xi in 0..x_res - 1 {
        for yi in 0..y_res - 1 {
            let p00 = grid[xi][yi];
            let p10 = grid[xi + 1][yi];
            let p01 = grid[xi][yi + 1];
            let p11 = grid[xi + 1][yi + 1];

            let pts = [
                project(p00[0], p00[1], p00[2]),
                project(p10[0], p10[1], p10[2]),
                project(p11[0], p11[1], p11[2]),
                project(p01[0], p01[1], p01[2]),
            ];

            let avg_z = (p00[2] + p10[2] + p01[2] + p11[2]) / 4.0;
            let color_t = (avg_z - z_min) / z_range;
            let fill = heat_color(color_t);

            let poly = format!(
                r#"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="{}" stroke="rgba(0,0,0,0.08)" stroke-width="0.3"/>"#,
                pts[0].0, pts[0].1,
                pts[1].0, pts[1].1,
                pts[2].0, pts[2].1,
                pts[3].0, pts[3].1,
                fill
            );
            faces.push((avg_z, poly));
        }
    }

    faces.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let faces_svg: String = faces.iter().map(|(_, s)| s.as_str()).collect();

    let mut axis_paths = String::new();
    let mut axis_labels = String::new();

    let origin = project(x_lo, y_lo, z_min);
    let x_end = project(x_hi, y_lo, z_min);
    let y_end = project(x_lo, y_hi, z_min);
    let z_top = project(x_lo, y_lo, z_max);

    axis_paths.push_str(&format!(
        r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="var(--amber)" stroke-width="1.5"/>"#,
        origin.0, origin.1, x_end.0, x_end.1
    ));
    axis_paths.push_str(&format!(
        r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="var(--amber)" stroke-width="1.5"/>"#,
        origin.0, origin.1, y_end.0, y_end.1
    ));
    axis_paths.push_str(&format!(
        r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="var(--amber)" stroke-width="1.5"/>"#,
        origin.0, origin.1, z_top.0, z_top.1
    ));

    let x_mid = project((x_lo + x_hi) / 2.0, y_lo, z_min);
    let y_mid = project(x_lo, (y_lo + y_hi) / 2.0, z_min);
    let _z_mid = project(x_lo, y_lo, (z_min + z_max) / 2.0);

    axis_labels.push_str(&format!(
        r#"<text x="{:.1}" y="{:.1}" fill="var(--amber)" font-size="9" text-anchor="middle" font-weight="600">{}</text>"#,
        x_mid.0, x_mid.1 + 16.0, x_name
    ));
    axis_labels.push_str(&format!(
        r#"<text x="{:.1}" y="{:.1}" fill="var(--amber)" font-size="9" text-anchor="middle" font-weight="600">{}</text>"#,
        y_mid.0 + 16.0, y_mid.1, y_name
    ));
    axis_labels.push_str(&format!(
        r#"<text x="{:.1}" y="{:.1}" fill="var(--amber)" font-size="9" text-anchor="middle" font-weight="600">{}</text>"#,
        z_top.0, z_top.1 + 16.0, out_name
    ));

    let mut tick_labels = String::new();
    for i in 0..=4 {
        let t = i as f64 / 4.0;
        let xv = x_lo + t * (x_hi - x_lo);
        let yv = y_lo + t * (y_hi - y_lo);
        let zv = z_min + t * z_range;

        let xp = project(xv, y_lo, z_min);
        let yp = project(x_lo, yv, z_min);
        let zp = project(x_lo, y_lo, zv);

        tick_labels.push_str(&format!(
            r#"<text x="{:.1}" y="{:.1}" fill="var(--text3)" font-size="7" text-anchor="end">{}</text>"#,
            xp.0 - 6.0, xp.1 + 3.0, format_value(xv)
        ));
        tick_labels.push_str(&format!(
            r#"<text x="{:.1}" y="{:.1}" fill="var(--text3)" font-size="7" text-anchor="start">{}</text>"#,
            yp.0 + 4.0, yp.1 + 3.0, format_value(yv)
        ));
        tick_labels.push_str(&format!(
            r#"<text x="{:.1}" y="{:.1}" fill="var(--text3)" font-size="7" text-anchor="end">{}</text>"#,
            zp.0 - 6.0, zp.1 + 3.0, format_value(zv)
        ));

        let xt = project(xv, y_lo, z_min);
        let yt = project(x_lo, yv, z_min);
        let zt = project(x_lo, y_lo, zv);
        tick_labels.push_str(&format!(
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="var(--border)" stroke-width="0.3"/>"#,
            xt.0, xt.1, origin.0, origin.1
        ));
        tick_labels.push_str(&format!(
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="var(--border)" stroke-width="0.3"/>"#,
            yt.0, yt.1, origin.0, origin.1
        ));
        tick_labels.push_str(&format!(
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="var(--border)" stroke-width="0.3"/>"#,
            zt.0, zt.1, origin.0, origin.1
        ));
    }

    let color_bar_w = 12.0;
    let color_bar_h = 180.0;
    let cb_x = w - pad - color_bar_w - 10.0;
    let cb_y = pad + 20.0;

    let mut color_bar = String::new();
    let n_steps = 40;
    for i in 0..n_steps {
        let t = i as f64 / (n_steps - 1) as f64;
        let y_pos = cb_y + (1.0 - t) * color_bar_h;
        let h_step = color_bar_h / n_steps as f64;
        color_bar.push_str(&format!(
            r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="{}"/>"#,
            cb_x, y_pos, color_bar_w, h_step + 0.5, heat_color(t)
        ));
    }
    color_bar.push_str(&format!(
        r#"<text x="{:.1}" y="{:.1}" fill="var(--text3)" font-size="7" text-anchor="middle">{}</text>"#,
        cb_x + color_bar_w / 2.0, cb_y - 5.0, format_value(z_max)
    ));
    color_bar.push_str(&format!(
        r#"<text x="{:.1}" y="{:.1}" fill="var(--text3)" font-size="7" text-anchor="middle">{}</text>"#,
        cb_x + color_bar_w / 2.0, cb_y + color_bar_h + 10.0, format_value(z_min)
    ));
    color_bar.push_str(&format!(
        r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="none" stroke="var(--border)" stroke-width="0.5"/>"#,
        cb_x, cb_y, color_bar_w, color_bar_h
    ));

    format!(
        r#"<svg width="{w}" height="{h}" viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg" style="background:var(--surface1);border-radius:4px;width:100%;max-width:{w}px">
            <rect width="100%" height="100%" fill="transparent"/>
            {tick_labels}
            {faces_svg}
            {axis_paths}
            {axis_labels}
            {color_bar}
            <text x="{title_x:.1}" y="{title_y:.1}" fill="var(--text3)" font-size="8" text-anchor="middle">Superfície 3D — {x_name} × {y_name} → {out_name}</text>
        </svg>"#,
        w = w, h = h,
        title_x = w / 2.0, title_y = pad - 10.0,
        tick_labels = tick_labels, faces_svg = faces_svg,
        axis_paths = axis_paths, axis_labels = axis_labels,
        color_bar = color_bar,
        x_name = x_name, y_name = y_name, out_name = out_name,
    )
}

fn format_value(v: f64) -> String {
    if v.abs() >= 1_000_000.0 {
        format!("{:.1}M", v / 1_000_000.0)
    } else if v.abs() >= 1_000.0 {
        format!("{:.1}k", v / 1_000.0)
    } else if (v - v.round()).abs() < 0.01 {
        format!("{:.0}", v)
    } else {
        format!("{:.1}", v)
    }
}

/// Gera SVG das funções de pertinência de um sistema (UC19).
pub fn generate_svg(variables: &[VarInfo]) -> Vec<(String, String)> {
    let mut svgs = Vec::new();
    for var in variables {
        let resolution = var.resolution.max(2);
        let uni = Universe::new(var.universe_min, var.universe_max, resolution);
        let mut fv = FuzzyVariable::new(&var.name, uni);
        for term in &var.terms {
            if let Some(mf) = mf_from_params(&term.mf_type, &term.params) {
                fv.add_term(Term::new(&term.label, mf));
            }
        }
        let svg_str = var_svg!(fv);
        svgs.push((var.name.clone(), svg_str));
    }
    svgs
}

/// Miniatura de SplitMix64 PRNG — algoritmo idêntico ao usado no logicfuzzy_academic.
struct PsoRng(u64);

impl PsoRng {
    fn new(seed: u64) -> Self { Self(seed) }

    fn next_f64(&mut self) -> f64 {
        let mut z = self.0.wrapping_add(0x9e3779b97f4a7c15);
        self.0 = z;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        let u = z ^ (z >> 31);
        (u >> 11) as f64 * (1.0 / 9007199254740992.0)
    }

    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }
}

/// Parâmetros de configuração do PSO.
#[derive(Debug, Clone)]
pub struct PsoConfigParams {
    pub seed: u64,
    pub w: f64,
    pub c1: f64,
    pub c2: f64,
}

impl Default for PsoConfigParams {
    fn default() -> Self {
        Self { seed: 42, w: 0.729, c1: 1.494, c2: 1.494 }
    }
}

/// Otimiza parâmetros de MF usando PSO (UC17).
/// Retorna (best_position, best_fitness, history[(iter, fitness)]).
pub fn optimize_with_pso(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    target_inputs: &[HashMap<String, f64>],
    target_outputs: &[HashMap<String, f64>],
    population_size: usize,
    max_iterations: usize,
    config: &PsoConfigParams,
) -> Result<(Vec<f64>, f64, Vec<(f64, f64)>), String> {
    if target_inputs.is_empty() || target_outputs.is_empty() {
        return Err("Dados de referência vazios".into());
    }
    if target_inputs.len() != target_outputs.len() {
        return Err("Quantidade de inputs e outputs difere".into());
    }

    let all_params: Vec<(f64, f64)> = variables.iter().flat_map(|v| {
        v.terms.iter().flat_map(|t| match t.mf_type.as_str() {
            "trimf" => vec![
                (v.universe_min, v.universe_max),
                (v.universe_min, v.universe_max),
                (v.universe_min, v.universe_max),
            ],
            "trapmf" => vec![
                (v.universe_min, v.universe_max),
                (v.universe_min, v.universe_max),
                (v.universe_min, v.universe_max),
                (v.universe_min, v.universe_max),
            ],
            "gaussmf" => vec![
                (v.universe_min, v.universe_max),
                (0.1, (v.universe_max - v.universe_min) / 2.0),
            ],
            _ => vec![],
        })
    }).collect();

    if all_params.is_empty() {
        return Err("Nenhum parâmetro para otimizar".into());
    }

    let fitness_fn = |params: &[f64]| {
        let mut param_idx = 0;
        let mut mse = 0.0_f64;

        for (i, input_row) in target_inputs.iter().enumerate() {
            let mut temp_vars = variables.to_vec();
            for var in &mut temp_vars {
                for term in &mut var.terms {
                    let n_params = match term.mf_type.as_str() {
                        "trimf" => 3,
                        "trapmf" => 4,
                        "gaussmf" => 2,
                        _ => 0,
                    };
                    if n_params > 0 && param_idx + n_params <= params.len() {
                        let mut new_params = params[param_idx..param_idx + n_params].to_vec();
                        match term.mf_type.as_str() {
                            "trimf" if new_params.len() >= 3 => {
                                new_params.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                            }
                            "trapmf" if new_params.len() >= 4 => {
                                new_params.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                            }
                            _ => {}
                        }
                        term.params = new_params;
                        param_idx += n_params;
                    }
                }
            }
            param_idx = 0;

            let outputs = evaluate_mamdani(&temp_vars, rules, input_row);
            if let Some(expected) = target_outputs.get(i) {
                for (k, v) in &outputs {
                    if let Some(ev) = expected.get(k) {
                        let diff = v - ev;
                        mse += diff * diff;
                    }
                }
            }
        }
        mse / target_inputs.len() as f64
    };

    // ── PSO loop manual com captura de histórico ──
    let dim = all_params.len();
    let mut rng = PsoRng::new(config.seed);
    let w = config.w;
    let c1 = config.c1;
    let c2 = config.c2;

    // Inicializa partículas
    let mut pos: Vec<Vec<f64>> = (0..population_size)
        .map(|_| all_params.iter().map(|&(lo, hi)| rng.range(lo, hi)).collect())
        .collect();
    let mut vel: Vec<Vec<f64>> = (0..population_size)
        .map(|_| all_params.iter().map(|&(lo, hi)| rng.range(-(hi - lo) * 0.1, (hi - lo) * 0.1)).collect())
        .collect();
    let mut pbest_pos = pos.clone();
    let mut pbest_fit: Vec<f64> = pos.iter().map(|p| fitness_fn(p)).collect();

    // Melhor global inicial
    let mut gbest_idx = 0;
    for i in 1..population_size {
        if pbest_fit[i] < pbest_fit[gbest_idx] { gbest_idx = i; }
    }
    let mut gbest_pos = pbest_pos[gbest_idx].clone();
    let mut gbest_fit = pbest_fit[gbest_idx];

    let mut history = vec![(0.0_f64, gbest_fit)];

    for iter in 0..max_iterations {
        for i in 0..population_size {
            for j in 0..dim {
                let r1 = rng.next_f64();
                let r2 = rng.next_f64();
                vel[i][j] = w * vel[i][j]
                    + c1 * r1 * (pbest_pos[i][j] - pos[i][j])
                    + c2 * r2 * (gbest_pos[j] - pos[i][j]);
                pos[i][j] = (pos[i][j] + vel[i][j])
                    .clamp(all_params[j].0, all_params[j].1);
            }

            let fit = fitness_fn(&pos[i]);
            if fit < pbest_fit[i] {
                pbest_fit[i] = fit;
                pbest_pos[i] = pos[i].clone();
            }
            if fit < gbest_fit {
                gbest_fit = fit;
                gbest_pos = pos[i].clone();
            }
        }
        history.push(((iter + 1) as f64, gbest_fit));
    }

    Ok((gbest_pos, gbest_fit, history))
}

/// Executa PSO múltiplas vezes com sementes diferentes e retorna estatísticas agregadas.
pub fn multi_run_pso(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    target_inputs: &[HashMap<String, f64>],
    target_outputs: &[HashMap<String, f64>],
    population_size: usize,
    max_iterations: usize,
    num_runs: usize,
    base_config: &PsoConfigParams,
) -> Result<(Vec<f64>, f64, Vec<[f64; 2]>, Vec<serde_json::Value>), String> {
    let mut all_fitness: Vec<f64> = Vec::with_capacity(num_runs);
    let mut all_histories: Vec<Vec<(f64, f64)>> = Vec::with_capacity(num_runs);
    let mut best_overall_pos = Vec::new();
    let mut best_overall_fit = f64::MAX;

    for run in 0..num_runs {
        let mut cfg = base_config.clone();
        cfg.seed = base_config.seed.wrapping_add(run as u64);
        let (pos, fit, history) = optimize_with_pso(
            variables, rules, target_inputs, target_outputs,
            population_size, max_iterations, &cfg,
        )?;
        all_fitness.push(fit);
        all_histories.push(history);
        if fit < best_overall_fit {
            best_overall_fit = fit;
            best_overall_pos = pos;
        }
    }

    let mean = all_fitness.iter().sum::<f64>() / all_fitness.len() as f64;
    let variance = all_fitness.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / all_fitness.len() as f64;
    let _std_dev = variance.sqrt();
    let _min_fit = all_fitness.iter().cloned().fold(f64::MAX, f64::min);
    let _max_fit = all_fitness.iter().cloned().fold(f64::MIN, f64::max);

    let runs_json: Vec<serde_json::Value> = (0..num_runs).map(|i| {
        let h: Vec<[f64; 2]> = all_histories[i].iter().map(|&(a, b)| [a, b]).collect();
        serde_json::json!({
            "run": i,
            "seed": base_config.seed.wrapping_add(i as u64),
            "best_fitness": all_fitness[i],
            "history": h,
        })
    }).collect();

    let best_history = all_histories.into_iter()
        .min_by(|a, b| {
            let a_last = a.last().map(|&(_, f)| f).unwrap_or(f64::MAX);
            let b_last = b.last().map(|&(_, f)| f).unwrap_or(f64::MAX);
            a_last.partial_cmp(&b_last).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or_default()
        .into_iter()
        .map(|(a, b)| [a, b])
        .collect();

    Ok((best_overall_pos, best_overall_fit, best_history, runs_json))
}

/// Explora a superfície de saída de um sistema fuzzy usando PSO.
/// Encontra mínimo, máximo e classifica a superfície (mínimo/máximo/sela/monotônica).
pub fn explore_output_surface(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    x_var: &str,
    y_var: &str,
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
) -> Result<serde_json::Value, String> {
    let n_ant = variables.iter().filter(|v| v.role == "antecedent" || v.role == "input").count();
    if n_ant < 2 {
        return Err("São necessárias ao menos 2 variáveis antecedentes".into());
    }

    let pso_bounds = vec![
        (x_bounds.0.min(x_bounds.1), x_bounds.0.max(x_bounds.1)),
        (y_bounds.0.min(y_bounds.1), y_bounds.0.max(y_bounds.1)),
    ];
    let pop_size = 20;
    let max_iter = 50;

    // ── 1. PSO — encontrar MÍNIMO ──
    let config_min = PsoConfig {
        population_size: pop_size,
        max_iterations: max_iter,
        bounds: pso_bounds.clone(),
        seed: Some(42),
        ..Default::default()
    };
    let mut opt_min = PsoOptimizer::new(config_min);
    let fitness_min = |params: &[f64]| {
        let mut inputs = HashMap::new();
        inputs.insert(x_var.to_string(), params[0]);
        inputs.insert(y_var.to_string(), params[1]);
        let outputs = evaluate_mamdani(variables, rules, &inputs);
        outputs.values().copied().sum::<f64>() / outputs.len().max(1) as f64
    };
    let (min_pos, min_val, _) = opt_min.optimize(fitness_min);

    // ── 2. PSO — encontrar MÁXIMO (nega fitness) ──
    let config_max = PsoConfig {
        population_size: pop_size,
        max_iterations: max_iter,
        bounds: pso_bounds.clone(),
        seed: Some(43),
        ..Default::default()
    };
    let mut opt_max = PsoOptimizer::new(config_max);
    let fitness_max = |params: &[f64]| {
        let mut inputs = HashMap::new();
        inputs.insert(x_var.to_string(), params[0]);
        inputs.insert(y_var.to_string(), params[1]);
        let outputs = evaluate_mamdani(variables, rules, &inputs);
        -(outputs.values().copied().sum::<f64>() / outputs.len().max(1) as f64)
    };
    let (max_pos, max_neg, _) = opt_max.optimize(fitness_max);
    let max_val = -max_neg;

    // ── 3. PSO exploratório (seed diferente) para detectar sela ──
    let config_seed2 = PsoConfig {
        population_size: pop_size,
        max_iterations: max_iter / 2,
        bounds: pso_bounds.clone(),
        seed: Some(99),
        ..Default::default()
    };
    let mut opt_seed2 = PsoOptimizer::new(config_seed2);
    let fitness_seed2 = |params: &[f64]| {
        let mut inputs = HashMap::new();
        inputs.insert(x_var.to_string(), params[0]);
        inputs.insert(y_var.to_string(), params[1]);
        let outputs = evaluate_mamdani(variables, rules, &inputs);
        outputs.values().copied().sum::<f64>() / outputs.len().max(1) as f64
    };
    let (alt_pos, alt_val, _) = opt_seed2.optimize(fitness_seed2);

    // ── 4. Amostragem para rugosidade ──
    let sample_res = 8usize;
    let mut values = Vec::new();
    for xi in 0..sample_res {
        for yi in 0..sample_res {
            let xv = x_bounds.0 + (xi as f64 / (sample_res - 1) as f64) * (x_bounds.1 - x_bounds.0);
            let yv = y_bounds.0 + (yi as f64 / (sample_res - 1) as f64) * (y_bounds.1 - y_bounds.0);
            let mut inputs = HashMap::new();
            inputs.insert(x_var.to_string(), xv);
            inputs.insert(y_var.to_string(), yv);
            let outputs = evaluate_mamdani(variables, rules, &inputs);
            let z = outputs.values().copied().sum::<f64>() / outputs.len().max(1) as f64;
            values.push(z);
        }
    }
    let roughness = if values.len() > 1 {
        let diff_sum: f64 = values.windows(2).map(|w| (w[1] - w[0]).abs()).sum();
        diff_sum / (values.len() - 1) as f64
    } else {
        0.0
    };

    // ── 5. Classificação ──
    let range = max_val - min_val;
    let spread = if range.abs() < 1e-9 { 0.0 } else { range };
    let alt_diff = ((alt_pos[0] - min_pos[0]).powi(2) + (alt_pos[1] - min_pos[1]).powi(2)).sqrt();

    let classification = if spread.abs() < 1e-3 {
        "monotonica".to_string()
    } else if alt_diff > (x_bounds.1 - x_bounds.0).abs() * 0.3
        || alt_diff > (y_bounds.1 - y_bounds.0).abs() * 0.3
    {
        if (alt_val - min_val).abs() < spread * 0.1 || (alt_val - max_val).abs() < spread * 0.1 {
            if min_val.abs() < max_val.abs() {
                "minimo".to_string()
            } else {
                "maximo".to_string()
            }
        } else {
            "sela".to_string()
        }
    } else if spread > 0.0 {
        if min_val.abs() < max_val.abs() && (max_val - min_val).abs() > spread * 0.5 {
            "minimo_maximo".to_string()
        } else if min_val.abs() < max_val.abs() {
            "minimo".to_string()
        } else {
            "maximo".to_string()
        }
    } else {
        "indefinido".to_string()
    };

    Ok(serde_json::json!({
        "x_var": x_var,
        "y_var": y_var,
        "min_point": { "x": min_pos[0], "y": min_pos[1], "z": min_val },
        "max_point": { "x": max_pos[0], "y": max_pos[1], "z": max_val },
        "classification": classification,
        "roughness": (roughness * 1000.0).round() / 1000.0,
        "spread": (spread * 1000.0).round() / 1000.0,
        "min_val": (min_val * 1000.0).round() / 1000.0,
        "max_val": (max_val * 1000.0).round() / 1000.0,
        "alt_point": { "x": alt_pos[0], "y": alt_pos[1], "z": alt_val },
        "converged": true,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimf_peak() {
        let mu = membership(25.0, "trimf", &[0.0, 25.0, 50.0]);
        assert!((mu - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_trimf_left_edge() {
        let mu = membership(0.0, "trimf", &[0.0, 25.0, 50.0]);
        assert!((mu - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_trimf_right_edge() {
        let mu = membership(50.0, "trimf", &[0.0, 25.0, 50.0]);
        assert!((mu - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_trimf_linear_rise() {
        let mu = membership(12.5, "trimf", &[0.0, 25.0, 50.0]);
        assert!((mu - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_trimf_linear_fall() {
        let mu = membership(37.5, "trimf", &[0.0, 25.0, 50.0]);
        assert!((mu - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_trapmf_plateau() {
        let mu = membership(30.0, "trapmf", &[0.0, 20.0, 40.0, 60.0]);
        assert!((mu - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_trapmf_left_ramp() {
        let mu = membership(10.0, "trapmf", &[0.0, 20.0, 40.0, 60.0]);
        assert!((mu - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_trapmf_right_ramp() {
        let mu = membership(50.0, "trapmf", &[0.0, 20.0, 40.0, 60.0]);
        assert!((mu - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_trapmf_outside() {
        let mu = membership(-1.0, "trapmf", &[0.0, 20.0, 40.0, 60.0]);
        assert!((mu - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_gaussmf_peak() {
        let mu = membership(50.0, "gaussmf", &[50.0, 10.0]);
        assert!((mu - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_gaussmf_one_sigma() {
        let mu = membership(60.0, "gaussmf", &[50.0, 10.0]);
        assert!((mu - (-0.5_f64).exp()).abs() < 1e-10);
    }

    #[test]
    fn test_parse_simple_rule() {
        let conds = parse_rule_conditions(
            "SE Temperatura = Alta ENTAO Risco = Alto",
            &["Temperatura".to_string(), "Risco".to_string()],
        );
        assert_eq!(conds.len(), 2);
        assert_eq!(conds[0], ("Temperatura".to_string(), "Alta".to_string()));
        assert_eq!(conds[1], ("Risco".to_string(), "Alto".to_string()));
    }

    #[test]
    fn test_parse_portuguese_rule() {
        let conds = parse_rule_conditions(
            "SE temperatura é frio E umidade é seco ENTÃO conforto é desconfortavel",
            &["temperatura".to_string(), "umidade".to_string(), "conforto".to_string()],
        );
        assert_eq!(conds.len(), 3);
        assert_eq!(conds[0], ("temperatura".to_string(), "frio".to_string()));
        assert_eq!(conds[1], ("umidade".to_string(), "seco".to_string()));
        assert_eq!(conds[2], ("conforto".to_string(), "desconfortavel".to_string()));
    }

    #[test]
    fn test_mamdani_basic_inference() {
        let var_id = Uuid::new_v4();
        let term_alta = TermInfo {
            term_id: Uuid::new_v4(),
            label: "Alta".into(),
            mf_type: "trimf".into(),
            params: vec![50.0, 75.0, 100.0],
        };
        let term_alto = TermInfo {
            term_id: Uuid::new_v4(),
            label: "Alto".into(),
            mf_type: "trimf".into(),
            params: vec![0.0, 0.5, 1.0],
        };
        let variables = vec![
            VarInfo {
                var_id,
                name: "Temperatura".into(),
                role: "antecedent".into(),
                universe_min: 0.0,
                universe_max: 100.0,
                resolution: 101,
                terms: vec![term_alta],
            },
            VarInfo {
                var_id: Uuid::new_v4(),
                name: "Risco".into(),
                role: "consequent".into(),
                universe_min: 0.0,
                universe_max: 1.0,
                resolution: 101,
                terms: vec![term_alto],
            },
        ];
        let rules = vec![
            RuleInfo {
                rule_text: "SE Temperatura = Alta ENTAO Risco = Alto".into(),
                weight: 1.0,
            },
        ];
        let inputs = [("Temperatura".to_string(), 80.0)].into();
        let outputs = evaluate_mamdani(&variables, &rules, &inputs);
        let risco = outputs.get("Risco").copied().unwrap_or(0.5);
        assert!(risco > 0.0, "Risco should be positive, got {risco}");
        assert!(risco < 1.0, "Risco should be < 1.0, got {risco}");
    }

    #[test]
    fn test_membership_nan_input_doesnt_panic() {
        let r = membership(f64::NAN, "trimf", &[0.0, 25.0, 50.0]);
        assert!(r.is_nan(), "membership com x=NaN deve retornar NaN, não panic");
        let r = membership(f64::NAN, "gaussmf", &[50.0, 10.0]);
        assert!(r.is_nan(), "gaussmf com x=NaN deve retornar NaN, não panic");
    }
}
