use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

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
        .route("/systems/{id}/rule-matrix", get(rule_matrix))
        .route("/systems/{id}/surface", post(surface))
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub city: String,
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

    // mock simulation result (will integrate logicfuzzy-academic later)
    let output_value = req.inputs.values().sum::<f64>() / req.inputs.len() as f64;

    let outputs = json!({ "resultado": output_value });

    // persist
    sqlx::query(
        "INSERT INTO simulations (system_id, inputs, outputs) VALUES ($1, $2, $3)"
    )
    .bind(system_id)
    .bind(json!(req.inputs))
    .bind(&outputs)
    .execute(&state.pool)
    .await?;

    Ok(Json(json!({
        "outputs": outputs,
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

async fn export_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(format): Query<Option<String>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let sim = sqlx::query_as::<_, Simulation>(
        "SELECT * FROM simulations WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Simulação não encontrada".into()))?;

    let fmt = format.unwrap_or_else(|| "json".into());

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

    let mut points = Vec::new();
    let mut x = req.start;
    while x <= req.end {
        let mut inputs = req.fixed.clone();
        inputs.insert(req.variable.clone(), x);
        let output = inputs.values().sum::<f64>() / inputs.len() as f64;
        points.push(json!({ "x": x, "y": output }));
        x += req.step;
    }

    Ok(Json(json!({ "points": points, "variable": req.variable })))
}

async fn rule_matrix(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rules = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE system_id = $1 ORDER BY position"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    let mut rows = Vec::new();
    for rule in &rules {
        rows.push(json!({
            "rule_id": rule.id,
            "rule_text": rule.rule_text,
            "position": rule.position,
            "activation": 0.0, // mock: no simulation data
        }));
    }

    Ok(Json(json!({ "rules": rows, "system_id": system_id })))
}

async fn surface(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<SurfaceRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let x_res = req.x_resolution.unwrap_or(20).min(100);
    let y_res = req.y_resolution.unwrap_or(20).min(100);

    let mut grid = Vec::new();
    for xi in 0..x_res {
        for yi in 0..y_res {
            let x_val = (xi as f64 / (x_res - 1) as f64) * 100.0;
            let y_val = (yi as f64 / (y_res - 1) as f64) * 100.0;
            let z = (x_val + y_val) / 2.0; // mock
            grid.push(json!({ "x": x_val, "y": y_val, "z": z }));
        }
    }

    Ok(Json(json!({ "grid": grid, "x_var": req.x, "y_var": req.y })))
}
