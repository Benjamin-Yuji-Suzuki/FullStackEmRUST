use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_create_system() {
    let mut app = TestApp::new().await;
    let req = json_post(
        "/api/systems",
        &serde_json::json!({"name": "Sistema Teste", "description": "desc", "defuzz_method": "centroid"}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Sistema Teste");
}

#[serial]
#[tokio::test]
async fn test_list_systems() {
    let mut app = TestApp::new().await;
    let req = json_get("/api/systems");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.is_array());
}

#[serial]
#[tokio::test]
async fn test_get_system_by_id() {
    let mut app = TestApp::new().await;
    let id = create_test_system(&mut app, "Get Test").await;
    let req = json_get(&format!("/api/systems/{id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Get Test");
}

#[serial]
#[tokio::test]
async fn test_update_system() {
    let mut app = TestApp::new().await;
    let id = create_test_system(&mut app, "Update Before").await;
    let req = json_put(
        &format!("/api/systems/{id}"),
        &serde_json::json!({"name": "Update After", "description": "updated", "defuzz_method": "centroid"}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Update After");
}

#[serial]
#[tokio::test]
async fn test_delete_system() {
    let mut app = TestApp::new().await;
    let id = create_test_system(&mut app, "Delete Me").await;
    let req = json_delete(&format!("/api/systems/{id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    let req = json_get(&format!("/api/systems/{id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[serial]
#[tokio::test]
async fn test_update_system_status() {
    let mut app = TestApp::new().await;
    let id = create_test_system(&mut app, "Status Test").await;
    let req = json_put(
        &format!("/api/systems/{id}/status"),
        &serde_json::json!({"status": "ativo"}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["status"], "ativo");
}

#[serial]
#[tokio::test]
async fn test_system_not_found() {
    let mut app = TestApp::new().await;
    let id = "00000000-0000-0000-0000-000000000000";
    let req = json_get(&format!("/api/systems/{id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[serial]
#[tokio::test]
async fn test_create_system_validation_error() {
    let mut app = TestApp::new().await;
    let req = json_post("/api/systems", &serde_json::json!({"name": "", "defuzz_method": "invalido"}));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
