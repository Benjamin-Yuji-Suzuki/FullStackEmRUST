use super::*;
use serial_test::serial;

#[serial]
#[tokio::test]
async fn test_optimize_compute() {
    let mut app = TestApp::new().await;
    let req = json_post(
        "/api/optimize",
        &serde_json::json!({
            "coef_a": 2.0, "coef_b": 0.0, "coef_c": 4.0,
            "coef_d": 0.0, "coef_e": 0.0, "coef_f": 0.0,
            "x_min": -10.0, "x_max": 10.0,
            "y_min": -10.0, "y_max": 10.0,
            "system_id": null,
        }),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("optimal_x").is_some());
}

#[serial]
#[tokio::test]
async fn test_optimize_validation_error() {
    let mut app = TestApp::new().await;
    let req = json_post(
        "/api/optimize",
        &serde_json::json!({
            "coef_a": 1.0, "coef_b": 0.0, "coef_c": 0.0,
            "coef_d": 0.0, "coef_e": 1.0, "coef_f": 0.0,
            "x_min": 10.0, "x_max": 0.0,
            "y_min": 0.0, "y_max": 10.0,
        }),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[serial]
#[tokio::test]
async fn test_list_optimizations() {
    let mut app = TestApp::new().await;
    let req = json_get("/api/optimizations?system_id=00000000-0000-0000-0000-000000000000");
    let resp = app.call(req).await;
    if resp.status() == StatusCode::BAD_REQUEST {
        return;
    }
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.is_array());
}

#[serial]
#[tokio::test]
async fn test_export_optimization() {
    let mut app = TestApp::new().await;
    let req = json_post(
        "/api/optimize",
        &serde_json::json!({
            "coef_a": 1.0, "coef_b": 0.0, "coef_c": 1.0,
            "coef_d": 0.0, "coef_e": 0.0, "coef_f": 0.0,
            "x_min": -10.0, "x_max": 10.0,
            "y_min": -10.0, "y_max": 10.0,
            "system_id": null,
        }),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    let opt_id = body["id"].as_str().unwrap().to_string();
    let req = json_get(&format!("/api/optimizations/{opt_id}/export"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("optimal_point").is_some());
    assert!(body.get("function").is_some());
}
