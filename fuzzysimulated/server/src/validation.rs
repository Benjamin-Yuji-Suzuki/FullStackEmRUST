pub fn validate_trimf(params: &[f64]) -> Result<(), String> {
    if params.len() != 3 {
        return Err("trimf requer 3 parâmetros".into());
    }
    if params[0] > params[1] || params[1] > params[2] {
        return Err("trimf: a ≤ b ≤ c".into());
    }
    Ok(())
}

pub fn validate_trapmf(params: &[f64]) -> Result<(), String> {
    if params.len() != 4 {
        return Err("trapmf requer 4 parâmetros".into());
    }
    if params[0] > params[1] || params[1] > params[2] || params[2] > params[3] {
        return Err("trapmf: a ≤ b ≤ c ≤ d".into());
    }
    Ok(())
}

pub fn validate_gaussmf(params: &[f64]) -> Result<(), String> {
    if params.len() != 2 {
        return Err("gaussmf requer 2 parâmetros".into());
    }
    if params[1] <= 0.0 {
        return Err("gaussmf: sigma > 0".into());
    }
    Ok(())
}

pub fn validate_system_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Nome obrigatório".into());
    }
    if trimmed.len() > 255 {
        return Err("Máximo 255 caracteres".into());
    }
    Ok(())
}

pub fn validate_defuzz_method(method: &str) -> Result<(), String> {
    let valid = ["centroid", "bisector", "mom", "lom", "som"];
    if !valid.contains(&method) {
        return Err(format!("Método inválido: {method}"));
    }
    Ok(())
}
