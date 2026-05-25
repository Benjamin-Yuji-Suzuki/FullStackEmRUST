use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_list_audit() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Audit Sys").await;
    let req = json_get(&format!("/api/systems/{sys_id}/audit"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("events").is_some());
}

#[serial]
#[tokio::test]
async fn test_list_orphan_audit() {
    let mut app = TestApp::new().await;
    let req = json_get("/api/audit/orphans");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body["events"].is_array());
    assert!(body["total"].is_number());
}

#[serial]
#[tokio::test]
async fn test_audit_undo_system_delete() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Undo Test").await;
    let req = json_delete(&format!("/api/systems/{sys_id}"));
    let _ = app.call(req).await;
    let req = json_get("/api/audit/orphans");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    let events = body["events"].as_array().unwrap();
    let delete_event = events.iter().find(|e| e["action_type"] == "delete").unwrap();
    let event_id = delete_event["id"].as_str().unwrap();
    let req = json_post(&format!("/api/audit/{event_id}/undo"), &serde_json::json!({}));
    let resp = app.call(req).await;
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
}
