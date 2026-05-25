use server::validation::{validate_gaussmf, validate_trapmf, validate_trimf};

#[test]
fn test_validate_trimf_ok() {
    validate_trimf(&[0.0, 10.0, 12.0]).expect("trimf [0,10,12] deve ser Ok");
}

#[test]
fn test_validate_trimf_non_finite() {
    let result = validate_trimf(&[0.0, f64::NAN, 10.0]);
    assert!(result.is_err(), "NaN deve ser rejeitado");
    let result = validate_trimf(&[0.0, f64::INFINITY, 10.0]);
    assert!(result.is_err(), "Inf deve ser rejeitado");
}

#[test]
fn test_validate_trimf_shoulder() {
    validate_trimf(&[0.0, 0.0, 25.0]).expect("shoulder esquerdo deve ser Ok");
    validate_trimf(&[25.0, 50.0, 50.0]).expect("shoulder direito deve ser Ok");
}

#[test]
fn test_validate_trimf_incoherent() {
    let result = validate_trimf(&[22.0, 10.0, 0.0]);
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}

#[test]
fn test_validate_trimf_wrong_params() {
    let result = validate_trimf(&[1.0, 2.0]);
    assert!(
        result.is_err(),
        "Esperava Err para 2 params mas obteve: {:?}",
        result
    );
    let result = validate_trimf(&[1.0, 2.0, 3.0, 4.0]);
    assert!(
        result.is_err(),
        "Esperava Err para 4 params mas obteve: {:?}",
        result
    );
}

#[test]
fn test_validate_trapmf_ok() {
    validate_trapmf(&[0.0, 0.0, 20.0, 40.0]).expect("trapmf [0,0,20,40] deve ser Ok");
    validate_trapmf(&[60.0, 80.0, 100.0, 100.0]).expect("shoulder trapmf deve ser Ok");
}

#[test]
fn test_validate_trapmf_incoherent() {
    let result = validate_trapmf(&[40.0, 20.0, 0.0, 0.0]);
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}

#[test]
fn test_validate_gaussmf_ok() {
    validate_gaussmf(&[50.0, 15.0]).expect("gaussmf [50,15] deve ser Ok");
}

#[test]
fn test_validate_gaussmf_zero_sigma() {
    let result = validate_gaussmf(&[50.0, 0.0]);
    assert!(
        result.is_err(),
        "Esperava Err para sigma=0 mas obteve: {:?}",
        result
    );
}

#[test]
fn test_validate_gaussmf_negative_sigma() {
    let result = validate_gaussmf(&[50.0, -1.0]);
    assert!(
        result.is_err(),
        "Esperava Err para sigma negativo mas obteve: {:?}",
        result
    );
}

#[test]
fn test_validate_gaussmf_wrong_params() {
    let result = validate_gaussmf(&[50.0]);
    assert!(
        result.is_err(),
        "Esperava Err para 1 param mas obteve: {:?}",
        result
    );
    let result = validate_gaussmf(&[50.0, 15.0, 10.0]);
    assert!(
        result.is_err(),
        "Esperava Err para 3 params mas obteve: {:?}",
        result
    );
}
