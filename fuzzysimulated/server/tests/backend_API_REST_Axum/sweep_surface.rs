use super::*;
use serial_test::serial;

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
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": risco_name, "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
    )).await;
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

#[serial]
#[tokio::test]
async fn test_analyze_surface() {
    let mut app = TestApp::new().await;
    let suffix = unique_str();
    let sys_id = create_test_system(&mut app, &format!("PSO Analyze {suffix}")).await;
    let var_a = create_test_variable(&mut app, &sys_id, &format!("input_a_{suffix}")).await;
    create_test_term(&mut app, &var_a, "baixo", "trimf", &[0.0, 0.0, 5.0]).await;
    create_test_term(&mut app, &var_a, "alto", "trimf", &[5.0, 10.0, 10.0]).await;
    let var_b = create_test_variable(&mut app, &sys_id, &format!("input_b_{suffix}")).await;
    create_test_term(&mut app, &var_b, "baixo", "trimf", &[0.0, 0.0, 5.0]).await;
    create_test_term(&mut app, &var_b, "alto", "trimf", &[5.0, 10.0, 10.0]).await;
    let resp_con = app.call(json_post(
        &format!("/api/systems/{sys_id}/variables"),
        &serde_json::json!({"name": format!("output_{suffix}"), "role": "consequent", "universe_min": 0.0, "universe_max": 1.0}),
    )).await;
    let con_id = hyper_body_to_json(resp_con).await["id"].as_str().unwrap().to_string();
    create_test_term(&mut app, &con_id, "baixo", "trimf", &[0.0, 0.0, 0.5]).await;
    create_test_term(&mut app, &con_id, "alto", "trimf", &[0.5, 1.0, 1.0]).await;

    let a_name = format!("input_a_{suffix}");
    let b_name = format!("input_b_{suffix}");
    let out_name = format!("output_{suffix}");
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": format!("SE {a_name} = alto E {b_name} = alto ENTAO {out_name} = alto"), "weight": 1.0}),
    )).await;
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": format!("SE {a_name} = baixo E {b_name} = baixo ENTAO {out_name} = baixo"), "weight": 1.0}),
    )).await;
    let _ = app.call(json_post(
        &format!("/api/systems/{sys_id}/rules"),
        &serde_json::json!({"rule_text": format!("SE {a_name} = alto E {b_name} = baixo ENTAO {out_name} = alto"), "weight": 0.5}),
    )).await;

    let req = json_post(
        &format!("/api/systems/{sys_id}/analyze-surface"),
        &serde_json::json!({"x_var": a_name, "y_var": b_name}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::OK, "analyze-surface deve retornar 200");
    let body: Value = hyper_body_to_json(resp).await;
    assert!(body.get("classification").is_some(), "deve ter classification");
    assert!(body.get("min_point").is_some(), "deve ter min_point");
    assert!(body.get("max_point").is_some(), "deve ter max_point");
    let class = body["classification"].as_str().unwrap_or("");
    assert!(["minimo", "maximo", "minimo_maximo", "sela", "monotonica", "indefinido"].contains(&class),
        "classification invalida: {class}");
}

#[serial]
#[tokio::test]
async fn test_analyze_surface_invalid_vars() {
    let mut app = TestApp::new().await;
    let sys_id = create_test_system(&mut app, "Analyze Invalid").await;
    let req = json_post(
        &format!("/api/systems/{sys_id}/analyze-surface"),
        &serde_json::json!({"x_var": "nao_existe", "y_var": "tambem_nao"}),
    );
    let resp = app.call(req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY, "variavel invalida deve dar 422");
}
