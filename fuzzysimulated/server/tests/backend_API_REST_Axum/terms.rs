use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_create_term() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Term Sys").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Pressao").await;
    let req = json_post(
        &format!("/api/variables/{var_id}/terms"),
        &serde_json::json!({"label": "Baixa", "mf_type": "trimf", "params": [0.0, 25.0, 50.0]}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["label"], "Baixa");
}

#[serial]
#[tokio::test]
async fn test_get_term() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Get Term Sys").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Velocidade").await;
    let term_id = create_test_term(&mut app, &var_id, "Alta", "trimf", &[50.0, 75.0, 100.0]).await;
    let req = json_get(&format!("/api/terms/{term_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["label"], "Alta");
}

#[serial]
#[tokio::test]
async fn test_update_term() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Upd Term Sys").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Temperatura2").await;
    let term_id = create_test_term(&mut app, &var_id, "Antes", "trimf", &[0.0, 25.0, 50.0]).await;
    let req = json_put(
        &format!("/api/terms/{term_id}"),
        &serde_json::json!({"label": "Depois", "mf_type": "trapmf", "params": [0.0, 20.0, 30.0, 50.0]}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["label"], "Depois");
}

#[serial]
#[tokio::test]
async fn test_delete_term() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Del Term Sys").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Vazao").await;
    let term_id = create_test_term(&mut app, &var_id, "Alto", "trimf", &[50.0, 75.0, 100.0]).await;
    let req = json_delete(&format!("/api/terms/{term_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[serial]
#[tokio::test]
async fn test_create_term_validation_error() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Term Val Err").await;
    let var_id = create_test_variable(&mut app, &sys_id, "Var").await;
    let req = json_post(
        &format!("/api/variables/{var_id}/terms"),
        &serde_json::json!({"label": "", "mf_type": "trimf", "params": [1.0, 2.0]}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
