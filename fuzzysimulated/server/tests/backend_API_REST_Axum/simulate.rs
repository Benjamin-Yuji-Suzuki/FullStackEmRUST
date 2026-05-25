use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_simulate() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/simulate"),
        &serde_json::json!({"inputs": {"Temperatura": 80.0}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body["outputs"].is_object());
}

#[serial]
#[tokio::test]
async fn test_simulate_missing_input() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/simulate"),
        &serde_json::json!({"inputs": {}}),
    );
    let resp = app.call(req).await;
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::UNPROCESSABLE_ENTITY);
}

#[serial]
#[tokio::test]
async fn test_list_simulations() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/simulate"),
        &serde_json::json!({"inputs": {"Temperatura": 80.0}}),
    );
    let _resp = app.call(req).await;
    let req = json_get(&format!("/api/systems/{sys_id}/simulations"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.is_array());
}

#[serial]
#[tokio::test]
async fn test_duplicate_system() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/duplicate"),
        &serde_json::json!({"name": "Copia do Sistema"}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Copia do Sistema");
}

// ─── TSK (UC18) ────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_simulate_tsk() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let coeffs = serde_json::json!({
        "inputs": {"Temperatura": 80.0},
        "coeffs": { "Risco_Alto": [50.0, 0.5] }
    });
    let req = json_post(&format!("/api/systems/{sys_id}/simulate-tsk"), &coeffs);
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK, "TSK deve retornar 200");
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body["outputs"].is_object());
    assert_eq!(body["method"], "tsk");
}

#[serial]
#[tokio::test]
async fn test_simulate_tsk_system_not_found() {
    let mut app = TestApp::new().await;
    let fake_id = Uuid::new_v4();
    let req = json_post(
        &format!("/api/systems/{fake_id}/simulate-tsk"),
        &serde_json::json!({"inputs": {"Temperatura": 80.0}, "coeffs": {}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ─── SVG (UC19) ────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_svg_export() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_get(&format!("/api/systems/{sys_id}/svg"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body["svgs"].is_array(), "svgs deve ser array");
    if let Some(svgs) = body["svgs"].as_array() {
        if !svgs.is_empty() {
            assert!(svgs[0]["svg"].as_str().map(|s| s.len() > 0).unwrap_or(false));
        }
    }
}

#[serial]
#[tokio::test]
async fn test_svg_export_system_not_found() {
    let mut app = TestApp::new().await;
    let fake_id = Uuid::new_v4();
    let req = json_get(&format!("/api/systems/{fake_id}/svg"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ─── Diagnostic (UC20) ─────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_diagnostic() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/diagnostic"),
        &serde_json::json!({"inputs": {"Temperatura": 80.0}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("fuzzification").is_some(), "diagnostic deve ter fuzzification");
}

#[serial]
#[tokio::test]
async fn test_diagnostic_system_not_found() {
    let mut app = TestApp::new().await;
    let fake_id = Uuid::new_v4();
    let req = json_post(
        &format!("/api/systems/{fake_id}/diagnostic"),
        &serde_json::json!({"inputs": {"Temperatura": 80.0}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ─── PSO (UC17) ────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_optimize_pso() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/optimize-pso"),
        &serde_json::json!({
            "target_inputs": [{"Temperatura": 80.0}],
            "target_outputs": [{"Risco": 0.8}],
            "population_size": 10,
            "max_iterations": 5
        }),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK, "PSO deve retornar 200: {:?}", resp.status());
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("best_position").is_some(), "PSO deve ter best_position");
    assert!(body.get("best_fitness").is_some(), "PSO deve ter best_fitness");
}

#[serial]
#[tokio::test]
async fn test_optimize_pso_invalid_data() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/optimize-pso"),
        &serde_json::json!({
            "target_inputs": [],
            "target_outputs": [],
            "population_size": 10,
            "max_iterations": 5
        }),
    );
    let resp = app.call(req).await;
    assert!(
        resp.status() == StatusCode::OK || resp.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Esperado 200 ou 422, got {:?}", resp.status()
    );
}
