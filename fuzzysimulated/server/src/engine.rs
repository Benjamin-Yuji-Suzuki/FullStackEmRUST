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

pub fn membership(value: f64, mf_type: &str, params: &[f64]) -> f64 {
    match mf_type {
        "trimf" if params.len() >= 3 => {
            let (a, b, c) = (params[0], params[1], params[2]);
            if value <= a || value >= c {
                0.0
            } else if (value - b).abs() < f64::EPSILON {
                1.0
            } else if value < b {
                (value - a) / (b - a)
            } else {
                (c - value) / (c - b)
            }
        }
        "trapmf" if params.len() >= 4 => {
            let (a, b, c, d) = (params[0], params[1], params[2], params[3]);
            if value <= a || value >= d {
                0.0
            } else if value >= b && value <= c {
                1.0
            } else if value < b {
                (value - a) / (b - a)
            } else {
                (d - value) / (d - c)
            }
        }
        "gaussmf" if params.len() >= 2 => {
            let (mean, sigma) = (params[0], params[1]);
            (-0.5 * ((value - mean) / sigma).powi(2)).exp()
        }
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
            if after.starts_with(sep) {
                let rest = after[sep.len()..].trim();
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

pub fn evaluate_mamdani(
    variables: &[VarInfo],
    rules: &[RuleInfo],
    inputs: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let var_names: Vec<String> = variables.iter().map(|v| v.name.clone()).collect();
    let mut outputs = HashMap::new();
    let consequent_vars: Vec<&VarInfo> = variables
        .iter()
        .filter(|v| v.role == "consequent")
        .collect();

    for cons_var in &consequent_vars {
        let resolution = cons_var.resolution.max(2);
        let step = (cons_var.universe_max - cons_var.universe_min) / (resolution - 1) as f64;
        let mut aggregated = vec![0.0_f64; resolution];
        let mut has_active_rule = false;

        for rule in rules {
            let conditions = parse_rule_conditions(&rule.rule_text, &var_names);
            if conditions.is_empty() {
                continue;
            }
            let mut alpha = 1.0_f64;
            let mut cons_term_label = String::new();
            for (var_name, term_label) in &conditions {
                let term = variables
                    .iter()
                    .find(|v| v.name.eq_ignore_ascii_case(var_name))
                    .and_then(|v| v.terms.iter().find(|t| t.label.eq_ignore_ascii_case(term_label)));
                match term {
                    Some(t) => {
                        if t.label == *term_label {
                            if cons_var.name.eq_ignore_ascii_case(var_name) {
                                cons_term_label = t.label.clone();
                            } else if let Some(input) = inputs.get(var_name) {
                                let mu = membership(*input, &t.mf_type, &t.params);
                                alpha = alpha.min(mu);
                            } else {
                                alpha = 0.0;
                            }
                        }
                    }
                    None => {
                        if variables.iter().any(|v| v.name.eq_ignore_ascii_case(var_name)) {
                            alpha = 0.0;
                        }
                    }
                }
            }
            alpha *= rule.weight;
            if alpha <= 0.0 || cons_term_label.is_empty() {
                continue;
            }
            has_active_rule = true;
            if let Some(cons_term) = cons_var.terms.iter().find(|t| t.label == cons_term_label) {
                for i in 0..resolution {
                    let y = cons_var.universe_min + i as f64 * step;
                    let mu_y = membership(y, &cons_term.mf_type, &cons_term.params);
                    let clipped = mu_y.min(alpha);
                    if clipped > aggregated[i] {
                        aggregated[i] = clipped;
                    }
                }
            }
        }
        let result = if !has_active_rule {
            (cons_var.universe_min + cons_var.universe_max) / 2.0
        } else {
            let mut num = 0.0_f64;
            let mut den = 0.0_f64;
            for i in 0..resolution {
                let y = cons_var.universe_min + i as f64 * step;
                num += y * aggregated[i];
                den += aggregated[i];
            }
            if den.abs() < f64::EPSILON {
                (cons_var.universe_min + cons_var.universe_max) / 2.0
            } else {
                num / den
            }
        };
        outputs.insert(cons_var.name.clone(), result);
    }
    outputs
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
