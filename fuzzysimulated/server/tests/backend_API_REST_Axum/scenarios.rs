use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_create_scenario() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/scenarios"),
        &serde_json::json!({"name": "Cenario Teste", "inputs": {"temp": 80.0}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Cenario Teste");
}

#[serial]
#[tokio::test]
async fn test_create_scenario_validation_error() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/scenarios"),
        &serde_json::json!({"name": "", "inputs": {}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[serial]
#[tokio::test]
async fn test_list_scenarios() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/scenarios"),
        &serde_json::json!({"name": "Cenario 1", "inputs": {"x": 1.0}}),
    );
    let _ = app.call(req).await;
    let req = json_get(&format!("/api/systems/{sys_id}/scenarios"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 1);
}

#[serial]
#[tokio::test]
async fn test_delete_scenario() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/scenarios"),
        &serde_json::json!({"name": "Delete Me", "inputs": {"x": 1.0}}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    let scenario_id = body["id"].as_str().unwrap().to_string();
    let req = json_delete(&format!("/api/scenarios/{scenario_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[serial]
#[tokio::test]
async fn test_delete_scenario_not_found() {
    let mut app = TestApp::new().await;
    let req = json_delete("/api/scenarios/00000000-0000-0000-0000-000000000000");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
