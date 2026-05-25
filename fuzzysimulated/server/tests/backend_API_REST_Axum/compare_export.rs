use super::*;
use serial_test::serial;

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
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": risco_name, "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
    )).await;
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
