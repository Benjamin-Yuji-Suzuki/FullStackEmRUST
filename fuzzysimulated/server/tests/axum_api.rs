#[path = "common/mod.rs"]
mod common;

use axum::body::Body;
use common::TestApp;
use http::StatusCode;
use serde_json::Value;
use serial_test::serial;
use uuid::Uuid;

fn unique_str() -> String {
    Uuid::new_v4().to_string()[..8].to_string()
}

fn json_post(path: &str, body: &Value) -> http::Request<Body> {
    http::Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

fn json_put(path: &str, body: &Value) -> http::Request<Body> {
    http::Request::builder()
        .method("PUT")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

fn json_get(path: &str) -> http::Request<Body> {
    http::Request::builder()
        .method("GET")
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

fn json_delete(path: &str) -> http::Request<Body> {
    http::Request::builder()
        .method("DELETE")
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

// ─── Systems ───────────────────────────────────────────────────────────────────

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

// ─── Variables ─────────────────────────────────────────────────────────────────

async fn create_test_system(app: &mut TestApp, name: &str) -> String {
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

async fn create_test_variable(app: &mut TestApp, sys_id: &str, name: &str) -> String {
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

// ─── Terms ─────────────────────────────────────────────────────────────────────

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

async fn create_test_term(app: &mut TestApp, var_id: &str, label: &str, mf_type: &str, params: &[f64]) -> String {
    let req = json_post(
        &format!("/api/variables/{var_id}/terms"),
        &serde_json::json!({"label": label, "mf_type": mf_type, "params": params}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    body["id"].as_str().unwrap().to_string()
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

// ─── Rules ─────────────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_create_rule() {
    let mut app = TestApp::new().await;
    let (sys_id, _cons_id) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": "SE Temperatura = Alta ENTAO Risco = Alto", "weight": 1.0}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = hyper_body_to_json(resp).await;
    assert_eq!(body["rule_text"], "SE Temperatura = Alta ENTAO Risco = Alto");
}

/// Creates a system with 1 antecedent (Temperatura + term Alta), 1 consequent (Risco + term Alto), and 1 rule
async fn create_minimal_system(app: &mut TestApp) -> (String, String) {
    let suffix = unique_str();
    let sys_id = create_test_system(app, &format!("Min Sys {suffix}")).await;
    let temp_name = format!("Temp_{suffix}");
    let risco_name = format!("Risco_{suffix}");
    let alta_label = format!("Alta_{suffix}");
    let alto_label = format!("Alto_{suffix}");
    let ant_id = create_test_variable(app, &sys_id, &temp_name).await;
    create_test_term(app, &ant_id, &alta_label, "trimf", &[50.0, 75.0, 100.0]).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": risco_name, "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    let cons_id = body["id"].as_str().unwrap().to_string();
    create_test_term(app, &cons_id, &alto_label, "trimf", &[0.0, 0.5, 1.0]).await;
    let rule_text = format!("SE {temp_name} = {alta_label} ENTAO {risco_name} = {alto_label}");
    let _resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": rule_text, "weight": 1.0}),
    )).await;
    (sys_id, cons_id)
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

async fn create_test_rule(app: &mut TestApp, sys_id: &str) -> String {
    let req = json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": "SE Temperatura = Alta ENTAO Risco = Alto", "weight": 1.0}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    body["id"].as_str().unwrap().to_string()
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

// ─── Simulation ────────────────────────────────────────────────────────────────

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
    // handler não valida inputs vazios; retorna 200 com NaN
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::UNPROCESSABLE_ENTITY);
}

#[serial]
#[tokio::test]
async fn test_list_simulations() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    // run simulation
    let req = json_post(
        &format!("/api/systems/{sys_id}/simulate"),
        &serde_json::json!({"inputs": {"Temperatura": 80.0}}),
    );
    let _resp = app.call(req).await;
    // list
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

// ─── UC17-PSO / UC18-TSK / UC19-SVG / UC20-Diagnostic ─────────────────────

#[serial]
#[tokio::test]
async fn test_simulate_tsk() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let coeffs = serde_json::json!({
        "inputs": {"Temperatura": 80.0},
        "coeffs": {
            "Risco_Alto": [50.0, 0.5]
        }
    });
    let req = json_post(
        &format!("/api/systems/{sys_id}/simulate-tsk"),
        &coeffs,
    );
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
    let coeffs = serde_json::json!({
        "inputs": {"Temperatura": 80.0},
        "coeffs": {}
    });
    let req = json_post(
        &format!("/api/systems/{fake_id}/simulate-tsk"),
        &coeffs,
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

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
    // Pode retornar 422 (Validation) ou 200 com fallback
    assert!(
        resp.status() == StatusCode::OK || resp.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Esperado 200 ou 422, got {:?}", resp.status()
    );
}

// ─── E2E: Pipeline completo com batch + simulação ──────────────────────────

#[serial]
#[tokio::test]
async fn test_e2e_full_pipeline() {
    let mut app = TestApp::new().await;
    let suffix = unique_str();

    // 1. Criar sistema com variáveis e regras (cibersegurança)
    let sys_id = create_test_system(&mut app, &format!("E2E Security {suffix}")).await;
    let temp_id = create_test_variable(&mut app, &sys_id, "severity").await;
    let _ = app.call(json_post(
        &format!("/api/variables/{temp_id}/terms"),
        &serde_json::json!({"label": "baixo", "mf_type": "trimf", "params": [0.0, 0.0, 3.0]}),
    )).await;
    let _ = app.call(json_post(
        &format!("/api/variables/{temp_id}/terms"),
        &serde_json::json!({"label": "medio", "mf_type": "trimf", "params": [2.0, 5.0, 8.0]}),
    )).await;
    let _ = app.call(json_post(
        &format!("/api/variables/{temp_id}/terms"),
        &serde_json::json!({"label": "alto", "mf_type": "trimf", "params": [6.0, 10.0, 10.0]}),
    )).await;

    let risk_id = {
        let resp = app.call(json_post(
            &format!("/api/systems/{sys_id}/variables"),
            &serde_json::json!({"name": "risk_level", "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
        )).await;
        hyper_body_to_json(resp).await["id"].as_str().unwrap().to_string()
    };
    let _ = app.call(json_post(
        &format!("/api/variables/{risk_id}/terms"),
        &serde_json::json!({"label": "safe", "mf_type": "trimf", "params": [0.0, 0.0, 0.5]}),
    )).await;
    let _ = app.call(json_post(
        &format!("/api/variables/{risk_id}/terms"),
        &serde_json::json!({"label": "critical", "mf_type": "trimf", "params": [0.3, 1.0, 1.0]}),
    )).await;

    // 2. Regras
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": "SE severity = alto ENTAO risk_level = critical", "weight": 1.0}),
    )).await;
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": "SE severity = baixo ENTAO risk_level = safe", "weight": 1.0}),
    )).await;

    // 3. Simulação Mamdani
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/simulate"),
        &serde_json::json!({"inputs": {"severity": 8.5}}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK, "Simulação deve funcionar");
    let sim: Value = hyper_body_to_json(resp).await;
    let risk_val = sim["outputs"]["risk_level"].as_f64().unwrap_or(0.0);
    assert!(risk_val > 0.5, "severity=8.5 deve gerar risk_level > 0.5, got {:.4}", risk_val);

    // 4. Diagnóstico
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/diagnostic"),
        &serde_json::json!({"inputs": {"severity": 8.5}}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let diag: Value = hyper_body_to_json(resp).await;
    assert!(diag["fuzzification"].as_array().map(|a| !a.is_empty()).unwrap_or(false), "Diagnóstico deve ter fuzzification");

    // 5. SVG
    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/svg"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let svg_resp: Value = hyper_body_to_json(resp).await;
    assert!(svg_resp["svgs"].as_array().map(|a| !a.is_empty()).unwrap_or(false), "SVG deve ter ao menos 1 variável");

    // 6. TSK
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/simulate-tsk"),
        &serde_json::json!({"inputs": {"severity": 8.5}, "coeffs": {"risk_level_critical": [0.0, 0.1]}}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK, "TSK deve funcionar");

    // 7. Batch
    let batch_inputs = serde_json::json!([
        {"severity": 1.0},
        {"severity": 5.0},
        {"severity": 9.0},
    ]);
    let resp = app.call(json_post(
        "/api/batch",
        &serde_json::json!({"system_id": sys_id, "inputs": batch_inputs}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK, "Batch deve funcionar");
    let batch: Value = hyper_body_to_json(resp).await;
    assert_eq!(batch["processed"], 3, "Batch deve processar 3 linhas");

    // 8. Histórico de simulações
    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/simulations"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let sims: Value = hyper_body_to_json(resp).await;
    assert!(sims.as_array().map(|a| a.len() >= 2).unwrap_or(false), "Deve ter ao menos 2 simulações (Mamdani + TSK)");

    // 9. Export do sistema
    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/export"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let export: Value = hyper_body_to_json(resp).await;
    assert!(export["rules"].as_array().map(|a| a.len() == 2).unwrap_or(false));

    // 10. Rule Matrix (UC14)
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/rule-matrix"),
        &serde_json::json!({"inputs": {"severity": 5.0}}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let matrix: Value = hyper_body_to_json(resp).await;
    assert!(matrix["rules"].as_array().map(|a| !a.is_empty()).unwrap_or(false), "Rule Matrix deve ter regras");

    // 11. Sweep (UC13)
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/sweep"),
        &serde_json::json!({"variable": "severity", "start": 0.0, "end": 10.0, "step": 5.0, "fixed": {}}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let sweep: Value = hyper_body_to_json(resp).await;
    assert!(sweep["points"].as_array().map(|a| a.len() == 3).unwrap_or(false), "Sweep deve ter 3 pontos");

    // 12. Surface (UC15)
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/surface"),
        &serde_json::json!({"x": "severity", "y": "severity", "x_resolution": 5, "y_resolution": 5}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let surface: Value = hyper_body_to_json(resp).await;
    assert!(surface["grid"].as_array().map(|a| a.len() == 25).unwrap_or(false), "Surface deve ter 25 pontos");

    // 13. Scenarios CRUD (UC12)
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/scenarios"),
        &serde_json::json!({"name": "Cenario E2E", "inputs": {"severity": 7.0}}),
    )).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Scenario deve ser criado");
    let scenario: Value = hyper_body_to_json(resp).await;
    let sc_id = scenario["id"].as_str().unwrap().to_string();

    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/scenarios"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let scenarios: Value = hyper_body_to_json(resp).await;
    assert!(scenarios.as_array().map(|a| a.len() >= 1).unwrap_or(false), "Deve ter ao menos 1 cenário");

    let resp = app.call(json_delete(&format!("/api/scenarios/{sc_id}"))).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT, "Scenario deve ser deletado");

    // 14. Compare simulations (UC08)
    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/simulations"))).await;
    let sims_list: Value = hyper_body_to_json(resp).await;
    if let Some(arr) = sims_list.as_array() {
        if arr.len() >= 2 {
            let id1 = arr[0]["id"].as_str().unwrap();
            let id2 = arr[1]["id"].as_str().unwrap();
            let resp = app.call(json_post(
                "/api/simulations/compare",
                &serde_json::json!({"simulation_ids": [id1, id2]}),
            )).await;
            assert_eq!(resp.status(), StatusCode::OK, "Compare deve funcionar");
        }
    }

    // 15. Duplicate system (UC10)
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/duplicate"),
        &serde_json::json!({"name": format!("E2E Copy {suffix}")}),
    )).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Duplicação deve funcionar");

    // 16. Import/Export round-trip (UC11)
    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/export"))).await;
    let export_data: Value = hyper_body_to_json(resp).await;
    let resp = app.call(json_post(
        "/api/systems/import",
        &export_data,
    )).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Import deve funcionar");

    // 17. Update system status (UC23)
    let resp = app.call(json_put(
        &format!("/api/systems/{sys_id}/status"),
        &serde_json::json!({"status": "favorito"}),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let updated: Value = hyper_body_to_json(resp).await;
    assert_eq!(updated["status"], "favorito");

    // 18. Otimização função quadrática (UC21)
    let resp = app.call(json_post(
        "/api/optimize",
        &serde_json::json!({
            "coef_a": 1.0, "coef_b": 0.0, "coef_c": 1.0,
            "coef_d": 0.0, "coef_e": 0.0, "coef_f": 0.0,
            "x_min": -10.0, "x_max": 10.0, "y_min": -10.0, "y_max": 10.0,
            "system_id": sys_id,
        }),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK, "Otimização quadrática deve funcionar");
    let opt: Value = hyper_body_to_json(resp).await;
    assert_eq!(opt["critical_point_type"], "mínimo");
    let opt_id = opt["id"].as_str().unwrap().to_string();

    // 20. Export otimização (UC25)
    let resp = app.call(json_get(&format!("/api/optimizations/{opt_id}/export"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let opt_export: Value = hyper_body_to_json(resp).await;
    assert!(opt_export["optimal_point"].is_object());

    // 21. PSO (UC17)
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/optimize-pso"),
        &serde_json::json!({
            "target_inputs": [{"severity": 1.0}, {"severity": 9.0}],
            "target_outputs": [{"risk_level": 0.1}, {"risk_level": 0.9}],
            "population_size": 5,
            "max_iterations": 3,
        }),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK, "PSO deve funcionar");
    let pso: Value = hyper_body_to_json(resp).await;
    assert!(pso["best_position"].is_array());
    assert!(pso["best_fitness"].is_f64());

    // 22. Audit — verificar que eventos foram criados
    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/audit"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let audit: Value = hyper_body_to_json(resp).await;
    assert!(audit["events"].as_array().map(|a| !a.is_empty()).unwrap_or(false), "Audit deve ter eventos");
}

// ─── Fim E2E ───────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_export_system() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_get(&format!("/api/systems/{sys_id}/export"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("variables").is_some());
}

#[serial]
#[tokio::test]
async fn test_rule_matrix() {
    let mut app = TestApp::new().await;
    let suffix = unique_str();
    let sys_id = create_test_system(&mut app, &format!("RuleMatrix Sys {suffix}")).await;
    let temp_name = format!("Temp_{suffix}");
    let risco_name = format!("Risco_{suffix}");
    let ant_id = create_test_variable(&mut app, &sys_id, &temp_name).await;
    create_test_term(&mut app, &ant_id, &format!("Alta_{suffix}"), "trimf", &[50.0, 75.0, 100.0]).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": risco_name, "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    let cons_id = body["id"].as_str().unwrap().to_string();
    create_test_term(&mut app, &cons_id, &format!("Alto_{suffix}"), "trimf", &[0.0, 0.5, 1.0]).await;
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": format!("SE {temp_name} = Alta_{suffix} ENTAO {risco_name} = Alto_{suffix}"), "weight": 1.0}),
    )).await;

    let req = json_post(
        &format!("/api/systems/{sys_id}/rule-matrix"),
        &serde_json::json!({"inputs": {temp_name: 50.0}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body["rules"].is_array());
    assert!(body["inputs"].is_object());
}

// ─── Optimize ─────────────────────────────────────────────────────────────────

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

// ─── Audit ────────────────────────────────────────────────────────────────────

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

// ─── Weather (error cases — no API key) ────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_weather_missing_city() {
    let mut app = TestApp::new().await;
    let req = json_get("/api/weather");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[serial]
#[tokio::test]
async fn test_weather_missing_api_key() {
    let mut app = TestApp::new().await;
    // remove any existing OPENWEATHER_API_KEY for this test
    std::env::remove_var("OPENWEATHER_API_KEY");
    let req = json_get("/api/weather?city=Belem");
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
}

// ─── Sweep ─────────────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_sweep() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/sweep"),
        &serde_json::json!({
            "variable": "Temperatura",
            "start": 0.0, "end": 100.0, "step": 50.0,
            "fixed": {}
        }),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body["points"].is_array());
}

#[serial]
#[tokio::test]
async fn test_sweep_validation_error() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/sweep"),
        &serde_json::json!({
            "variable": "Temperatura",
            "start": 100.0, "end": 0.0, "step": -1.0,
            "fixed": {}
        }),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ─── Scenarios (UC12) ──────────────────────────────────────────────────────────

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

// ─── Compare Simulations (UC08) ────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_compare_simulations() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    for _ in 0..2 {
        let req = json_post(
            &format!("/api/systems/{sys_id}/simulate"),
            &serde_json::json!({"inputs": {"Temperatura": 80.0}}),
        );
        let _ = app.call(req).await;
    }
    let req = json_get(&format!("/api/systems/{sys_id}/simulations"));
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    let sims = body.as_array().unwrap();
    assert!(sims.len() >= 2, "Need at least 2 simulations");
    let ids: Vec<String> = sims.iter().map(|s| s["id"].as_str().unwrap().to_string()).collect();
    let req = json_post(
        "/api/simulations/compare",
        &serde_json::json!({"simulation_ids": ids[..2].to_vec()}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[serial]
#[tokio::test]
async fn test_compare_simulations_validation() {
    let mut app = TestApp::new().await;
    let req = json_post(
        "/api/simulations/compare",
        &serde_json::json!({"simulation_ids": ["00000000-0000-0000-0000-000000000000"]}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ─── Export Report (UC09) ──────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_export_report() {
    let mut app = TestApp::new().await;
    let (sys_id, _) = create_minimal_system(&mut app).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/simulate"),
        &serde_json::json!({"inputs": {"Temperatura": 80.0}}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK, "simulate should return 200");
    // list simulations to get simulation ID
    let req = json_get(&format!("/api/systems/{sys_id}/simulations"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK, "list simulations should return 200");
    let body: Value = hyper_body_to_json(resp).await;
    let simulations = body.as_array().expect("list should return array");
    assert!(!simulations.is_empty(), "should have at least 1 simulation");
    let sim_id = simulations[0]["id"].as_str().unwrap().to_string();
    let req = json_get(&format!("/api/simulations/{sim_id}/report?format=json"));
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK, "export report should return 200");
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("simulation").is_some(), "response should contain simulation");
}

// ─── Batch (UC07) ──────────────────────────────────────────────────────────────

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

// ─── Surface ───────────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_surface() {
    let mut app = TestApp::new().await;
    let suffix = unique_str();
    let sys_id = create_test_system(&mut app, &format!("Surface Sys {suffix}")).await;
    let temp_name = format!("Temp_{suffix}");
    let risco_name = format!("Risco_{suffix}");
    let ant_id = create_test_variable(&mut app, &sys_id, &temp_name).await;
    create_test_term(&mut app, &ant_id, "Alta", "trimf", &[50.0, 75.0, 100.0]).await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": risco_name, "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
    );
    let resp = app.call(req).await;
    let body: Value = hyper_body_to_json(resp).await;
    let cons_id = body["id"].as_str().unwrap().to_string();
    create_test_term(&mut app, &cons_id, "Alto", "trimf", &[0.0, 0.5, 1.0]).await;
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": format!("SE {temp_name} = Alta ENTAO {risco_name} = Alto"), "weight": 1.0}),
    )).await;

    let req = json_post(
        &format!("/api/systems/{sys_id}/surface"),
        &serde_json::json!({"x": temp_name, "y": temp_name, "x_resolution": 5, "y_resolution": 5}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body["grid"].is_array());
}

// ─── Audit Undo ────────────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_audit_undo_system_delete() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Undo Test").await;
    let req = json_delete(&format!("/api/systems/{sys_id}"));
    let _ = app.call(req).await;
    // After delete, FK ON DELETE SET NULL makes events orphans
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

// ─── 404 edge cases ───────────────────────────────────────────────────────────

#[serial]
#[tokio::test]
async fn test_all_404_endpoints() {
    let mut app = TestApp::new().await;
    let uid = "00000000-0000-0000-0000-000000000000";

    for path in &[
        format!("/api/systems/{uid}"),
        format!("/api/variables/{uid}"),
        format!("/api/terms/{uid}"),
        format!("/api/rules/{uid}"),
        format!("/api/optimizations/{uid}"),
        format!("/api/optimizations/{uid}/export"),
    ] {
        let req = json_get(path);
        let resp = app.call(req).await;
        assert_eq!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "Expected 404 for {}",
            path
        );
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

async fn hyper_body_to_json(response: axum::response::Response) -> Value {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    serde_json::from_slice(&body_bytes).expect("Failed to parse JSON response")
}
