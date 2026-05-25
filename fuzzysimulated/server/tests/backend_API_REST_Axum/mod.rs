mod common;

pub use common::TestApp;

use axum::body::Body;
use http::StatusCode;
use serde_json::Value;
use uuid::Uuid;

pub fn unique_str() -> String {
    Uuid::new_v4().to_string()[..8].to_string()
}

pub fn json_post(path: &str, body: &Value) -> http::Request<Body> {
    http::Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

pub fn json_put(path: &str, body: &Value) -> http::Request<Body> {
    http::Request::builder()
        .method("PUT")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

pub fn json_get(path: &str) -> http::Request<Body> {
    http::Request::builder()
        .method("GET")
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

pub fn json_delete(path: &str) -> http::Request<Body> {
    http::Request::builder()
        .method("DELETE")
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

pub async fn hyper_body_to_json(response: axum::response::Response) -> Value {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    serde_json::from_slice(&body_bytes).expect("Failed to parse JSON response")
}

pub async fn create_test_system(app: &mut TestApp, name: &str) -> String {
    let req = json_post(
        "/api/systems",
        &serde_json::json!({"name": name, "description": null, "defuzz_method": "centroid"}),
    );
    let resp = app.call(req).await;
    let status = resp.status();
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "create_test_system({name}) failed: {:?}",
        body
    );
    body["id"].as_str().unwrap().to_string()
}

pub async fn create_test_variable(app: &mut TestApp, sys_id: &str, name: &str) -> String {
    let req = json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": name, "role": "antecedent", "universe_min": 0.0, "universe_max": 100.0}),
    );
    let resp = app.call(req).await;
    let status = resp.status();
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "create_test_variable({name}) failed: {:?}",
        body
    );
    body["id"].as_str().unwrap().to_string()
}

pub async fn create_test_term(app: &mut TestApp, var_id: &str, label: &str, mf_type: &str, params: &[f64]) -> String {
    let req = json_post(
        &format!("/api/variables/{var_id}/terms"),
        &serde_json::json!({"label": label, "mf_type": mf_type, "params": params}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    body["id"].as_str().unwrap().to_string()
}

pub async fn create_test_rule(app: &mut TestApp, sys_id: &str) -> String {
    let req = json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": "SE Temperatura = Alta ENTAO Risco = Alto", "weight": 1.0}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    body["id"].as_str().unwrap().to_string()
}

/// Creates a system with 1 antecedent (Temperatura + term Alta), 1 consequent (Risco + term Alto), and 1 rule
pub async fn create_minimal_system(app: &mut TestApp) -> (String, String) {
    let suffix = unique_str();
    let sys_id = create_test_system(app, &format!("Min Sys {suffix}")).await;
    let temp_name = format!("Temp_{suffix}");
    let ant_id = create_test_variable(app, &sys_id, &temp_name).await;
    create_test_term(app, &ant_id, &format!("Alta_{suffix}"), "trimf", &[50.0, 75.0, 100.0]).await;
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": format!("Risco_{suffix}"), "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
    )).await;
    let body: Value = hyper_body_to_json(resp).await;
    let cons_id = body["id"].as_str().unwrap().to_string();
    create_test_term(app, &cons_id, &format!("Alto_{suffix}"), "trimf", &[0.0, 0.5, 1.0]).await;
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": format!("SE {temp_name} = Alta_{suffix} ENTAO Risco_{suffix} = Alto_{suffix}"), "weight": 1.0}),
    )).await;
    (sys_id, cons_id)
}

pub mod systems;
pub mod variables;
pub mod terms;
pub mod rules;
pub mod simulate;
pub mod batch;
pub mod scenarios;
pub mod sweep_surface;
pub mod audit;
pub mod compare_export;
pub mod misc;
pub mod pipeline;
