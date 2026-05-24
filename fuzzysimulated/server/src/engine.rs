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

/// Otimiza parâmetros de MF usando PSO (UC17).
pub fn optimize_with_pso(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    target_inputs: &[HashMap<String, f64>],
    target_outputs: &[HashMap<String, f64>],
    population_size: usize,
    max_iterations: usize,
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

    let config = PsoConfig {
        population_size,
        max_iterations,
        bounds: all_params,
        seed: Some(42),
        ..Default::default()
    };

    let mut optimizer = PsoOptimizer::new(config);

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

    let (best_pos, best_fit, _state) = optimizer.optimize(fitness_fn);
    Ok((best_pos, best_fit, vec![]))
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
}
