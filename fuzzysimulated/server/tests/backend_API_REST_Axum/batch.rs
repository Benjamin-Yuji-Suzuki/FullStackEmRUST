use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_batch_process() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        "/api/batch",
        &serde_json::json!({
            "system_id": sys_id,
            "inputs": [{"Temperatura": 80.0}, {"Temperatura": 20.0}]
        }),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["total"], 2);
    assert_eq!(body["processed"], 2);
}

#[serial]
#[tokio::test]
async fn test_batch_process_empty() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        "/api/batch",
        &serde_json::json!({"system_id": sys_id, "inputs": []}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[serial]
#[tokio::test]
async fn test_batch_list_results() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        "/api/batch",
        &serde_json::json!({
            "system_id": sys_id,
            "inputs": [{"Temperatura": 50.0}]
        }),
    );
    let _ = app.call(req).await;
    let req = json_get(&format!("/api/batch/{sys_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.is_array());
}

#[serial]
#[tokio::test]
async fn test_batch_delete_result() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        "/api/batch",
        &serde_json::json!({
            "system_id": sys_id,
            "inputs": [{"Temperatura": 50.0}]
        }),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    if let Some(results) = body["results"].as_array() {
        if let Some(first) = results.first() {
            let batch_id = first["id"].as_str().unwrap();
            let req = json_delete(&format!("/api/batch/result/{batch_id}"));
            let resp = app.call(req).await;
            assert_eq!(resp.status(), StatusCode::NO_CONTENT);
        }
    }
}

#[serial]
#[tokio::test]
async fn test_batch_system_not_found() {
    let mut app = TestApp::new().await;
    let req = json_post(
        "/api/batch",
        &serde_json::json!({
            "system_id": "00000000-0000-0000-0000-000000000000",
            "inputs": [{"x": 1.0}]
        }),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
