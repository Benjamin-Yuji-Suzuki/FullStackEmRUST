use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::engine;
use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems/{id}/simulate", post(simulate))
        .route("/systems/{id}/simulations", get(list_simulations))
        .route("/simulations/{id}", delete(delete_simulation))
        .route("/simulations/compare", post(compare_simulations))
        .route("/simulations/{id}/report", get(export_report))
        .route("/systems/{id}/duplicate", post(duplicate_system))
        .route("/systems/{id}/export", get(export_system))
        .route("/systems/import", post(import_system))
        .route("/systems/{id}/sweep", post(sweep))
        .route("/systems/{id}/rule-matrix", post(rule_matrix))
        .route("/systems/{id}/surface", post(surface))
        .route("/systems/{id}/simulate-tsk", post(simulate_tsk))
        .route("/systems/{id}/svg", get(svg_export))
        .route("/systems/{id}/diagnostic", post(diagnostic))
        .route("/systems/{id}/optimize-pso", post(optimize_pso))
        .route("/systems/{id}/optimize-pso-auto", post(optimize_pso_auto))
        .route("/systems/{id}/apply-pso-params", post(apply_pso_params))
        .route("/systems/{id}/analyze-surface", post(analyze_surface))
}

#[derive(Deserialize)]
pub struct SweepRequest {
    pub variable: String,
    pub start: f64,
    pub end: f64,
    pub step: f64,
    pub fixed: std::collections::HashMap<String, f64>,
}

#[derive(Deserialize)]
pub struct CompareRequest {
    pub simulation_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct SurfaceRequest {
    pub x: String,
    pub y: String,
    pub x_resolution: Option<usize>,
    pub y_resolution: Option<usize>,
}

#[derive(Deserialize)]
pub struct AnalyzeSurfaceRequest {
    pub x_var: String,
    pub y_var: String,
}

async fn simulate(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<SimulateRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // validate system exists and is complete
    let _system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let var_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM fuzzy_variables WHERE system_id = $1"
    )
    .bind(system_id)
    .fetch_one(&state.pool)
    .await?;

    let rule_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM fuzzy_rules WHERE system_id = $1"
    )
    .bind(system_id)
    .fetch_one(&state.pool)
    .await?;

    if var_count < 2 || rule_count == 0 {
        return Err(AppError::Validation("Sistema incompleto: precisa de ao menos 2 variáveis e 1 regra".into()));
    }

    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;
    let outputs = engine::evaluate_mamdani(&var_infos, &rule_infos, &req.inputs);
    let outputs_json = serde_json::to_value(&outputs).unwrap_or_else(|_| json!({}));

    sqlx::query(
        "INSERT INTO simulations (system_id, inputs, outputs) VALUES ($1, $2, $3)"
    )
    .bind(system_id)
    .bind(json!(req.inputs))
    .bind(&outputs_json)
    .execute(&state.pool)
    .await?;

    Ok(Json(json!({
        "outputs": outputs_json,
        "inputs": req.inputs,
        "system_id": system_id,
    })))
}

async fn list_simulations(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<Vec<Simulation>>, AppError> {
    let rows = sqlx::query_as::<_, Simulation>(
        "SELECT * FROM simulations WHERE system_id = $1 ORDER BY executed_at DESC"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows))
}

async fn delete_simulation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM simulations WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Simulação não encontrada".into()));
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn compare_simulations(
    State(state): State<AppState>,
    Json(req): Json<CompareRequest>,
) -> Result<Json<Vec<Simulation>>, AppError> {
    if req.simulation_ids.len() < 2 {
        return Err(AppError::Validation("Selecione ao menos 2 simulações para comparar".into()));
    }

    let mut simulations = Vec::new();
    for id in &req.simulation_ids {
        let sim = sqlx::query_as::<_, Simulation>(
            "SELECT * FROM simulations WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Simulação {} não encontrada", id)))?;
        simulations.push(sim);
    }

    Ok(Json(simulations))
}

#[derive(serde::Deserialize)]
struct ReportQuery {
    format: Option<String>,
}

async fn export_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(query): Query<ReportQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let sim = sqlx::query_as::<_, Simulation>(
        "SELECT * FROM simulations WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Simulação não encontrada".into()))?;

    let fmt = query.format.unwrap_or_else(|| "json".into());

    Ok(Json(json!({
        "format": fmt,
        "simulation": sim,
        "message": "Relatório gerado com sucesso"
    })))
}

async fn duplicate_system(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<(axum::http::StatusCode, Json<FuzzySystem>), AppError> {
    let name = req.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let original = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema original não encontrado".into()))?;

    let new_name = if name.is_empty() {
        format!("{} (cópia)", original.name)
    } else {
        name
    };

    let new_system = sqlx::query_as::<_, FuzzySystem>(
        "INSERT INTO fuzzy_systems (name, description, defuzz_method) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&new_name)
    .bind(&original.description)
    .bind(&original.defuzz_method)
    .fetch_one(&state.pool)
    .await?;

    // clone variables, terms, rules
    let variables = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE system_id = $1"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    for var in &variables {
        let new_var = sqlx::query_as::<_, FuzzyVariable>(
            "INSERT INTO fuzzy_variables (system_id, name, role, universe_min, universe_max, resolution) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
        .bind(new_system.id)
        .bind(&var.name)
        .bind(&var.role)
        .bind(var.universe_min)
        .bind(var.universe_max)
        .bind(var.resolution)
        .fetch_one(&state.pool)
        .await?;

        let terms = sqlx::query_as::<_, FuzzyTerm>(
            "SELECT * FROM fuzzy_terms WHERE variable_id = $1"
        )
        .bind(var.id)
        .fetch_all(&state.pool)
        .await?;

        for term in &terms {
            sqlx::query(
                "INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES ($1, $2, $3, $4)"
            )
            .bind(new_var.id)
            .bind(&term.label)
            .bind(&term.mf_type)
            .bind(&term.params)
            .execute(&state.pool)
            .await?;
        }
    }

    let rules = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE system_id = $1 ORDER BY position"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    for rule in &rules {
        sqlx::query(
            "INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES ($1, $2, $3, $4)"
        )
        .bind(new_system.id)
        .bind(&rule.rule_text)
        .bind(rule.weight)
        .bind(rule.position)
        .execute(&state.pool)
        .await?;
    }

    Ok((axum::http::StatusCode::CREATED, Json(new_system)))
}

async fn export_system(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let variables = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE system_id = $1 ORDER BY name"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    let mut var_data = Vec::new();
    for var in &variables {
        let terms = sqlx::query_as::<_, FuzzyTerm>(
            "SELECT * FROM fuzzy_terms WHERE variable_id = $1 ORDER BY label"
        )
        .bind(var.id)
        .fetch_all(&state.pool)
        .await?;
        var_data.push(json!({
            "id": var.id,
            "name": var.name,
            "role": var.role,
            "universe_min": var.universe_min,
            "universe_max": var.universe_max,
            "resolution": var.resolution,
            "terms": terms.into_iter().map(|t| json!({
                "id": t.id,
                "variable_id": t.variable_id,
                "label": t.label,
                "mf_type": t.mf_type,
                "params": t.params,
            })).collect::<Vec<_>>(),
        }));
    }

    let rules = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE system_id = $1 ORDER BY position"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(json!({
        "name": system.name,
        "description": system.description,
        "defuzz_method": system.defuzz_method,
        "variables": var_data,
        "rules": rules,
    })))
}

async fn import_system(
    State(state): State<AppState>,
    Json(data): Json<serde_json::Value>,
) -> Result<(axum::http::StatusCode, Json<FuzzySystem>), AppError> {
    let name = data.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("Campo 'name' é obrigatório".into()))?
        .to_string();

    let description = data.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let defuzz_method = data.get("defuzz_method").and_then(|v| v.as_str()).unwrap_or("centroid").to_string();

    let system = sqlx::query_as::<_, FuzzySystem>(
        "INSERT INTO fuzzy_systems (name, description, defuzz_method) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&name)
    .bind(&description)
    .bind(&defuzz_method)
    .fetch_one(&state.pool)
    .await?;

    if let Some(vars) = data.get("variables").and_then(|v| v.as_array()) {
        for var_val in vars {
            let var = sqlx::query_as::<_, FuzzyVariable>(
                "INSERT INTO fuzzy_variables (system_id, name, role, universe_min, universe_max, resolution) \
                 VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
            )
            .bind(system.id)
            .bind(var_val.get("name").and_then(|v| v.as_str()).unwrap_or(""))
            .bind(var_val.get("role").and_then(|v| v.as_str()).unwrap_or("antecedent"))
            .bind(var_val.get("universe_min").and_then(|v| v.as_f64()).unwrap_or(0.0))
            .bind(var_val.get("universe_max").and_then(|v| v.as_f64()).unwrap_or(100.0))
            .bind(var_val.get("resolution").and_then(|v| v.as_i64()).unwrap_or(501) as i32)
            .fetch_one(&state.pool)
            .await?;

            if let Some(terms) = var_val.get("terms").and_then(|v| v.as_array()) {
                for term_val in terms {
                    sqlx::query(
                        "INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES ($1, $2, $3, $4::jsonb)"
                    )
                    .bind(var.id)
                    .bind(term_val.get("label").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(term_val.get("mf_type").and_then(|v| v.as_str()).unwrap_or("trimf"))
                    .bind(term_val.get("params").map(|v| v.to_string()).unwrap_or_else(|| "[0,0,0]".to_string()))
                    .execute(&state.pool)
                    .await?;
                }
            }
        }
    }

    if let Some(rules) = data.get("rules").and_then(|v| v.as_array()) {
        for (i, rule_val) in rules.iter().enumerate() {
            sqlx::query(
                "INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES ($1, $2, $3, $4)"
            )
            .bind(system.id)
            .bind(rule_val.get("rule_text").and_then(|v| v.as_str()).unwrap_or(""))
            .bind(rule_val.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0))
            .bind(i as i32)
            .execute(&state.pool)
            .await?;
        }
    }

    Ok((axum::http::StatusCode::CREATED, Json(system)))
}

#[derive(Deserialize)]
pub struct TskCoeffsRequest {
    pub inputs: std::collections::HashMap<String, f64>,
    pub coeffs: std::collections::HashMap<String, Vec<f64>>,
}

#[derive(Deserialize)]
pub struct DiagnosticRequest {
    pub inputs: std::collections::HashMap<String, f64>,
}

#[derive(Deserialize)]
pub struct OptimizePsoRequest {
    pub target_inputs: Vec<std::collections::HashMap<String, f64>>,
    pub target_outputs: Vec<std::collections::HashMap<String, f64>>,
    pub population_size: Option<usize>,
    pub max_iterations: Option<usize>,
}

async fn simulate_tsk(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<TskCoeffsRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;
    let outputs = engine::evaluate_tsk(&var_infos, &rule_infos, &req.inputs, &req.coeffs);

    sqlx::query(
        "INSERT INTO simulations (system_id, inputs, outputs) VALUES ($1, $2, $3)"
    )
    .bind(system_id)
    .bind(json!(req.inputs))
    .bind(json!(outputs))
    .execute(&state.pool)
    .await?;

    Ok(Json(json!({
        "outputs": outputs,
        "inputs": req.inputs,
        "system_id": system_id,
        "method": "tsk",
    })))
}

async fn svg_export(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let (var_infos, _) = load_engine_data(&state.pool, system_id).await?;
    let svgs = engine::generate_svg(&var_infos);

    Ok(Json(json!({
        "system_id": system_id,
        "svgs": svgs.into_iter().map(|(name, svg)| json!({
            "variable": name,
            "svg": svg,
        })).collect::<Vec<_>>(),
    })))
}

async fn diagnostic(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<DiagnosticRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;
    let diag = engine::generate_diagnostic(&var_infos, &rule_infos, &req.inputs)
        .map_err(AppError::Validation)?;

    Ok(Json(diag))
}

async fn optimize_pso(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<OptimizePsoRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;
    let pop = req.population_size.unwrap_or(20);
    let iters = req.max_iterations.unwrap_or(50);

    let (best_pos, best_fit, _history) = engine::optimize_with_pso(
        &var_infos, &rule_infos,
        &req.target_inputs, &req.target_outputs,
        pop, iters,
    ).map_err(AppError::Validation)?;

    Ok(Json(json!({
        "system_id": system_id,
        "best_position": best_pos,
        "best_fitness": best_fit,
    })))
}

#[derive(Deserialize)]
pub struct OptimizePsoAutoRequest {
    pub population_size: Option<usize>,
    pub max_iterations: Option<usize>,
}

/// PSO automático: lê batch_results do DB e otimiza MF params sem o usuário precisar
/// especificar função objetivo ou domínio.
async fn optimize_pso_auto(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<OptimizePsoAutoRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;

    let antecedents: Vec<&str> = var_infos.iter()
        .filter(|v| v.role == "antecedent")
        .map(|v| v.name.as_str())
        .collect();
    let consequents: Vec<&str> = var_infos.iter()
        .filter(|v| v.role == "consequent")
        .map(|v| v.name.as_str())
        .collect();

    if consequents.is_empty() {
        return Err(AppError::Validation("Sistema não tem variável consequente".into()));
    }

    let batch_rows = sqlx::query_as::<_, crate::models::BatchResult>(
        "SELECT * FROM batch_results WHERE system_id = $1 ORDER BY row_index"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    if batch_rows.is_empty() {
        return Err(AppError::Validation(
            "Nenhum resultado batch encontrado. Execute o batch primeiro.".into()
        ));
    }

    let mut target_inputs: Vec<std::collections::HashMap<String, f64>> = Vec::new();
    let mut target_outputs: Vec<std::collections::HashMap<String, f64>> = Vec::new();

    for row in &batch_rows {
        let input_map = row_to_f64_filtered(&row.inputs, &antecedents);
        if input_map.is_empty() { continue; }
        let mut out_map = std::collections::HashMap::new();
        for cons in &consequents {
            out_map.insert(cons.to_string(), row.output);
        }
        target_inputs.push(input_map);
        target_outputs.push(out_map);
    }

    if target_inputs.is_empty() {
        return Err(AppError::Validation(
            "Nenhum batch result com inputs válidos para este sistema.".into()
        ));
    }

    let pop = req.population_size.unwrap_or(30);
    let iters = req.max_iterations.unwrap_or(100);

    let (best_pos, best_fit, _history) = engine::optimize_with_pso(
        &var_infos, &rule_infos,
        &target_inputs, &target_outputs,
        pop, iters,
    ).map_err(AppError::Validation)?;

    Ok(Json(json!({
        "system_id": system_id,
        "best_position": best_pos,
        "best_fitness": best_fit,
        "trained_on": target_inputs.len(),
        "population_size": pop,
        "max_iterations": iters,
    })))
}

fn row_to_f64_filtered(
    row: &serde_json::Value,
    allowed_keys: &[&str],
) -> std::collections::HashMap<String, f64> {
    let mut result = std::collections::HashMap::new();
    if let Some(obj) = row.as_object() {
        for key in allowed_keys {
            if let Some(val) = obj.get(*key) {
                let num = match val {
                    serde_json::Value::Number(n) => n.as_f64(),
                    serde_json::Value::String(s) => s.parse::<f64>().ok(),
                    _ => None,
                };
                if let Some(n) = num {
                    result.insert(key.to_string(), n);
                }
            }
        }
    }
    result
}

#[derive(Deserialize)]
pub struct ApplyPsoParamsRequest {
    pub params: Vec<f64>,
}

async fn apply_pso_params(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<ApplyPsoParamsRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let terms = sqlx::query_as::<_, FuzzyTerm>(
        "SELECT ft.* FROM fuzzy_terms ft \
         JOIN fuzzy_variables fv ON fv.id = ft.variable_id \
         WHERE fv.system_id = $1 \
         ORDER BY fv.name, ft.label"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    let mut idx = 0;
    let mut updated = 0u32;
    for term in &terms {
        let n = match term.mf_type.as_str() {
            "trimf" => 3,
            "trapmf" => 4,
            "gaussmf" => 2,
            _ => continue,
        };
        if idx + n > req.params.len() {
            return Err(AppError::Validation("Parametros insuficientes".into()));
        }
        let mut new_params: Vec<f64> = req.params[idx..idx + n].to_vec();
        if term.mf_type == "trimf" || term.mf_type == "trapmf" {
            new_params.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        }
        let params_json = serde_json::to_value(&new_params).unwrap_or_default();
        sqlx::query("UPDATE fuzzy_terms SET params = $1::jsonb WHERE id = $2")
            .bind(&params_json)
            .bind(term.id)
            .execute(&state.pool)
            .await?;
        idx += n;
        updated += 1;
    }

    Ok(Json(json!({ "updated_terms": updated, "system_id": system_id })))
}

async fn load_engine_data(
    pool: &sqlx::PgPool,
    system_id: Uuid,
) -> Result<(Vec<engine::VarInfo>, Vec<engine::RuleInfo>), AppError> {
    let variables = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE system_id = $1"
    )
    .bind(system_id)
    .fetch_all(pool)
    .await?;

    let all_terms: Vec<FuzzyTerm> = sqlx::query_as(
        "SELECT ft.* FROM fuzzy_terms ft \
         JOIN fuzzy_variables fv ON fv.id = ft.variable_id \
         WHERE fv.system_id = $1"
    )
    .bind(system_id)
    .fetch_all(pool)
    .await?;

    let rules = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE system_id = $1 ORDER BY position"
    )
    .bind(system_id)
    .fetch_all(pool)
    .await?;

    let var_infos: Vec<engine::VarInfo> = variables.iter().map(|v| {
        let terms: Vec<engine::TermInfo> = all_terms.iter()
            .filter(|t| t.variable_id == v.id)
            .map(|t| {
                let params: Vec<f64> = t.params.as_array()
                    .map(|a| a.iter().filter_map(|x| x.as_f64()).collect())
                    .unwrap_or_default();
                engine::TermInfo {
                    term_id: t.id,
                    label: t.label.clone(),
                    mf_type: t.mf_type.clone(),
                    params,
                }
            })
            .collect();
        engine::VarInfo {
            var_id: v.id,
            name: v.name.clone(),
            role: v.role.clone(),
            universe_min: v.universe_min,
            universe_max: v.universe_max,
            resolution: v.resolution as usize,
            terms,
        }
    }).collect();

    let rule_infos: Vec<engine::RuleInfo> = rules.iter().map(|r| engine::RuleInfo {
        rule_text: r.rule_text.clone(),
        weight: r.weight,
    }).collect();

    Ok((var_infos, rule_infos))
}

async fn sweep(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<SweepRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.start >= req.end {
        return Err(AppError::Validation("Início deve ser menor que fim".into()));
    }
    if req.step <= 0.0 {
        return Err(AppError::Validation("Passo deve ser maior que zero".into()));
    }

    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;

    let mut points = Vec::new();
    let mut x = req.start;
    while x <= req.end {
        let mut inputs = req.fixed.clone();
        inputs.insert(req.variable.clone(), x);
        let outputs = engine::evaluate_mamdani(&var_infos, &rule_infos, &inputs);
        let y = outputs.values().copied().sum::<f64>() / outputs.len().max(1) as f64;
        points.push(json!({ "x": x, "y": y }));
        x += req.step;
    }

    Ok(Json(json!({ "points": points, "variable": req.variable })))
}

#[derive(Deserialize)]
pub struct RuleMatrixRequest {
    pub inputs: std::collections::HashMap<String, f64>,
}

async fn rule_matrix(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<RuleMatrixRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (var_infos, _ri) = load_engine_data(&state.pool, system_id).await?;
    let rules = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE system_id = $1 ORDER BY position"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    let mut var_map = std::collections::HashMap::new();
    for var in &var_infos {
        var_map.insert(var.name.clone(), var);
    }

    let mut rows = Vec::new();
    for rule in &rules {
        let activation = engine::compute_rule_activation(&rule.rule_text, &var_map, &req.inputs);
        rows.push(json!({
            "rule_id": rule.id,
            "rule_text": rule.rule_text,
            "position": rule.position,
            "weight": rule.weight,
            "activation": activation,
        }));
    }

    Ok(Json(json!({ "rules": rows, "system_id": system_id, "inputs": req.inputs })))
}

async fn surface(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<SurfaceRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let x_res = req.x_resolution.unwrap_or(20).min(50);
    let y_res = req.y_resolution.unwrap_or(20).min(50);
    let x_name = req.x.clone();
    let y_name = req.y.clone();

    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;

    let x_range = var_infos.iter().find(|v| v.name == x_name)
        .map(|v| (v.universe_min, v.universe_max))
        .ok_or_else(|| AppError::Validation(format!("Variável '{x_name}' não encontrada")))?;
    let y_range = var_infos.iter().find(|v| v.name == y_name)
        .map(|v| (v.universe_min, v.universe_max))
        .ok_or_else(|| AppError::Validation(format!("Variável '{y_name}' não encontrada")))?;

    let mut grid = Vec::new();
    for xi in 0..x_res {
        for yi in 0..y_res {
            let x_val = x_range.0 + (xi as f64 / (x_res - 1) as f64) * (x_range.1 - x_range.0);
            let y_val = y_range.0 + (yi as f64 / (y_res - 1) as f64) * (y_range.1 - y_range.0);
            let mut inputs = std::collections::HashMap::new();
            inputs.insert(x_name.clone(), x_val);
            inputs.insert(y_name.clone(), y_val);
            let outputs = engine::evaluate_mamdani(&var_infos, &rule_infos, &inputs);
            let z = outputs.values().copied().sum::<f64>() / outputs.len().max(1) as f64;
            grid.push(json!({ "x": x_val, "y": y_val, "z": z }));
        }
    }

    Ok(Json(json!({ "grid": grid, "x_var": x_name, "y_var": y_name })))
}

/// PSO Surface Analyzer — explora superfície de saída com PSO e classifica (min/max/sela)
async fn analyze_surface(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<AnalyzeSurfaceRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (var_infos, rule_infos) = load_engine_data(&state.pool, system_id).await?;

    let x_range = var_infos.iter().find(|v| v.name == req.x_var)
        .map(|v| (v.universe_min, v.universe_max))
        .ok_or_else(|| AppError::Validation(format!("Variável '{}' não encontrada", req.x_var)))?;
    let y_range = var_infos.iter().find(|v| v.name == req.y_var)
        .map(|v| (v.universe_min, v.universe_max))
        .ok_or_else(|| AppError::Validation(format!("Variável '{}' não encontrada", req.y_var)))?;

    let result = engine::explore_output_surface(
        &var_infos, &rule_infos,
        &req.x_var, &req.y_var,
        x_range, y_range,
    ).map_err(AppError::Validation)?;

    Ok(Json(result))
}
