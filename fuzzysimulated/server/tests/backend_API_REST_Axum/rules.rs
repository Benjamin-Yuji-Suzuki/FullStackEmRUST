use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_create_rule() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": "SE Temperatura = Alta ENTAO Risco = Alto", "weight": 1.0}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["rule_text"], "SE Temperatura = Alta ENTAO Risco = Alto");
}

#[serial]
#[tokio::test]
async fn test_get_rule() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let rule_id = create_test_rule(&mut app, &sys_id).await;
    let req = json_get(&format!("/api/rules/{rule_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["rule_text"], "SE Temperatura = Alta ENTAO Risco = Alto");
}

#[serial]
#[tokio::test]
async fn test_update_rule() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let rule_id = create_test_rule(&mut app, &sys_id).await;
    let req = json_put(
        &format!("/api/rules/{rule_id}"),
        &serde_json::json!({"rule_text": "SE Temperatura = Alta ENTAO Risco = Alto [weight: 0.5]", "weight": 0.5}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["rule_text"], "SE Temperatura = Alta ENTAO Risco = Alto [weight: 0.5]");
    assert!((body["weight"].as_f64().unwrap() - 0.5).abs() < 1e-6);
}

#[serial]
#[tokio::test]
async fn test_delete_rule() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let rule_id = create_test_rule(&mut app, &sys_id).await;
    let req = json_delete(&format!("/api/rules/{rule_id}"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[serial]
#[tokio::test]
async fn test_rule_not_found() {
    let mut app = TestApp::new().await;
    let req = json_get("/api/rules/00000000-0000-0000-0000-000000000000");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
