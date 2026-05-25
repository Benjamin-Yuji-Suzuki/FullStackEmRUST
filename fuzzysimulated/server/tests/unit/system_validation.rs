use server::validation::{validate_defuzz_method, validate_system_name};

#[test]
fn test_validate_system_name_ok() {
    validate_system_name("Conforto Térmico").expect("nome válido deve ser Ok");
}

#[test]
fn test_validate_system_name_empty() {
    let result = validate_system_name("");
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}

#[test]
fn test_validate_system_name_whitespace() {
    let result = validate_system_name("   ");
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}

#[test]
fn test_validate_system_name_too_long() {
    let long = "a".repeat(256);
    let result = validate_system_name(&long);
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}

#[test]
fn test_validate_defuzz_method_valid() {
    for method in ["centroid", "bisector", "mom", "lom", "som"] {
        validate_defuzz_method(method).expect(&format!("defuzz '{method}' deve ser Ok"));
    }
}

#[test]
fn test_validate_defuzz_method_invalid() {
    let result = validate_defuzz_method("invalid");
    assert!(result.is_err(), "Esperava Err mas obteve: {:?}", result);
}
