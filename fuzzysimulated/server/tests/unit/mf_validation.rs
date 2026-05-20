fn validate_trimf(params: &[f64]) -> Result<(), String> {
    if params.len() != 3 {
        return Err("trimf requer 3 parâmetros".into());
    }
    if params[0] > params[1] || params[1] > params[2] {
        return Err("trimf: a ≤ b ≤ c".into());
    }
    Ok(())
}

fn validate_trapmf(params: &[f64]) -> Result<(), String> {
    if params.len() != 4 {
        return Err("trapmf requer 4 parâmetros".into());
    }
    if params[0] > params[1] || params[1] > params[2] || params[2] > params[3] {
        return Err("trapmf: a ≤ b ≤ c ≤ d".into());
    }
    Ok(())
}

fn validate_gaussmf(params: &[f64]) -> Result<(), String> {
    if params.len() != 2 {
        return Err("gaussmf requer 2 parâmetros".into());
    }
    if params[1] <= 0.0 {
        return Err("gaussmf: sigma > 0".into());
    }
    Ok(())
}

#[test]
fn test_validate_trimf_ok() {
    let result = validate_trimf(&[0.0, 10.0, 22.0]);
    assert!(result.is_ok(), "Esperava Ok mas obteve: {:?}", result);
}

#[test]
fn test_validate_trimf_shoulder() {
    let result = validate_trimf(&[0.0, 0.0, 25.0]);
    assert!(result.is_ok(), "Esperava Ok para open left mas obteve: {:?}", result);
    let result = validate_trimf(&[25.0, 50.0, 50.0]);
    assert!(result.is_ok(), "Esperava Ok para open right mas obteve: {:?}", result);
}

#[test]
fn test_validate_trimf_incoherent() {
    let result = validate_trimf(&[22.0, 10.0, 0.0]);
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}

#[test]
fn test_validate_trimf_wrong_params() {
    let result = validate_trimf(&[1.0, 2.0]);
    assert!(result.is_err(), "Esperava Err para 2 params mas obteve: {:?}", result);
    let result = validate_trimf(&[1.0, 2.0, 3.0, 4.0]);
    assert!(result.is_err(), "Esperava Err para 4 params mas obteve: {:?}", result);
}

#[test]
fn test_validate_trapmf_ok() {
    let result = validate_trapmf(&[0.0, 0.0, 20.0, 40.0]);
    assert!(result.is_ok(), "Esperava Ok mas obteve: {:?}", result);
    let result = validate_trapmf(&[60.0, 80.0, 100.0, 100.0]);
    assert!(result.is_ok(), "Esperava Ok para shoulder mas obteve: {:?}", result);
}

#[test]
fn test_validate_trapmf_incoherent() {
    let result = validate_trapmf(&[40.0, 20.0, 0.0, 0.0]);
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}

#[test]
fn test_validate_gaussmf_ok() {
    let result = validate_gaussmf(&[50.0, 15.0]);
    assert!(result.is_ok(), "Esperava Ok mas obteve: {:?}", result);
}

#[test]
fn test_validate_gaussmf_zero_sigma() {
    let result = validate_gaussmf(&[50.0, 0.0]);
    assert!(result.is_err(), "Esperava Err para sigma=0 mas obteve: {:?}", result);
}

#[test]
fn test_validate_gaussmf_negative_sigma() {
    let result = validate_gaussmf(&[50.0, -1.0]);
    assert!(result.is_err(), "Esperava Err para sigma negativo mas obteve: {:?}", result);
}

#[test]
fn test_validate_gaussmf_wrong_params() {
    let result = validate_gaussmf(&[50.0]);
    assert!(result.is_err(), "Esperava Err para 1 param mas obteve: {:?}", result);
    let result = validate_gaussmf(&[50.0, 15.0, 10.0]);
    assert!(result.is_err(), "Esperava Err para 3 params mas obteve: {:?}", result);
}
