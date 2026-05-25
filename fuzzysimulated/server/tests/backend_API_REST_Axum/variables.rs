use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_create_variable() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Var System").await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": "Temperatura", "role": "antecedent", "universe_min": 0.0, "universe_max": 100.0, "resolution": 501}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Temperatura");
}

#[serial]
#[tokio::test]
async fn test_list_variables() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "List Var Sys").await;
    let req = json_get(&format!("/api/systems/{sys_id}/variables"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.is_array());
}

#[serial]
#[tokio::test]
async fn test_get_variable() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Get Var Sys").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Umidade").await;
    let req = json_get(&format!("/api/variables/{var_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Umidade");
}

#[serial]
#[tokio::test]
async fn test_update_variable() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Upd Var Sys").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Antes").await;
    let req = json_put(
        &format!("/api/variables/{var_id}"),
        &serde_json::json!({"name": "Depois", "role": "consequent", "universe_min": 0.0, "universe_max": 50.0}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["name"], "Depois");
    assert_eq!(body["universe_max"], 50.0);
}

#[serial]
#[tokio::test]
async fn test_delete_variable() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Del Var Sys").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Delete").await;
    let req = json_delete(&format!("/api/variables/{var_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[serial]
#[tokio::test]
async fn test_variable_not_found() {
    let mut app = TestApp::new().await;
    let req = json_get("/api/variables/00000000-0000-0000-0000-000000000000");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[serial]
#[tokio::test]
async fn test_create_variable_validation_error() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Var Val Err").await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": "", "role": "invalid", "universe_min": 10.0, "universe_max": 0.0}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
