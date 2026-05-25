use super::*;
use serial_test::serial;

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
    assert!(sims.as_array().map(|a| a.len() >= 2).unwrap_or(false), "Deve ter ao menos 2 simulações");

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

    // 19. Export otimização (UC25)
    let resp = app.call(json_get(&format!("/api/optimizations/{opt_id}/export"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let opt_export: Value = hyper_body_to_json(resp).await;
    assert!(opt_export["optimal_point"].is_object());

    // 20. PSO manual (UC17)
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/optimize-pso"),
        &serde_json::json!({
            "target_inputs": [{"severity": 1.0}, {"severity": 9.0}],
            "target_outputs": [{"risk_level": 0.1}, {"risk_level": 0.9}],
            "population_size": 5,
            "max_iterations": 3,
        }),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK, "PSO manual deve funcionar");
    let pso: Value = hyper_body_to_json(resp).await;
    assert!(pso["best_position"].is_array(), "PSO deve ter best_position");
    assert!(pso["best_fitness"].is_f64(), "PSO deve ter best_fitness");
    assert!(pso["history"].as_array().map(|h| h.len() > 1).unwrap_or(false), "PSO deve ter histórico de convergência");

    // 21. PSO auto from batch (UC17) — usa batch_results criados no passo 7
    let resp = app.call(json_post(
        &format!("/api/systems/{sys_id}/optimize-pso-auto"),
        &serde_json::json!({
            "population_size": 5,
            "max_iterations": 3,
        }),
    )).await;
    assert_eq!(resp.status(), StatusCode::OK, "PSO auto deve usar batch_results e funcionar");
    let pso_auto: Value = hyper_body_to_json(resp).await;
    assert!(pso_auto["best_position"].is_array(), "PSO auto deve ter best_position");
    assert!(pso_auto["best_fitness"].is_f64(), "PSO auto deve ter best_fitness");
    assert!(pso_auto["history"].as_array().map(|h| h.len() > 1).unwrap_or(false), "PSO auto deve ter histórico");

    // 22. Audit — verificar que eventos foram criados
    let resp = app.call(json_get(&format!("/api/systems/{sys_id}/audit"))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let audit: Value = hyper_body_to_json(resp).await;
    assert!(audit["events"].as_array().map(|a| !a.is_empty()).unwrap_or(false), "Audit deve ter eventos");
}
