use axum::{
    extract::{Path, State},
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
        .route("/batch", post(process_batch))
        .route("/batch/{system_id}", get(list_batch_results))
        .route("/batch/result/{id}", delete(delete_batch_result))
}

#[derive(Deserialize)]
pub struct BatchRequest {
    pub system_id: Uuid,
    pub inputs: Vec<std::collections::HashMap<String, f64>>,
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

async fn process_batch(
    State(state): State<AppState>,
    Json(req): Json<BatchRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(req.system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let (var_infos, rule_infos) = load_engine_data(&state.pool, req.system_id).await?;

    if req.inputs.is_empty() {
        return Err(AppError::Validation("Nenhum input fornecido".into()));
    }

    let mut results = Vec::new();
    let mut errors = 0;

    for (i, input_row) in req.inputs.iter().enumerate() {
        match engine::evaluate_mamdani(&var_infos, &rule_infos, input_row) {
            outputs if !outputs.is_empty() => {
                let output_val = outputs.values().copied().sum::<f64>() / outputs.len() as f64;
                let outputs_json = serde_json::to_value(&outputs).unwrap_or_else(|_| json!({}));

                let record = sqlx::query_as::<_, BatchResult>(
                    "INSERT INTO batch_results (system_id, source_file, row_index, inputs, output) \
                     VALUES ($1, 'batch-api', $2, $3::jsonb, $4) RETURNING *"
                )
                .bind(req.system_id)
                .bind(i as i32)
                .bind(json!(input_row))
                .bind(output_val)
                .fetch_one(&state.pool)
                .await?;

                results.push(json!({
                    "id": record.id,
                    "row_index": record.row_index,
                    "inputs": record.inputs,
                    "output": record.output,
                    "outputs_detail": outputs_json,
                    "executed_at": record.executed_at,
                }));
            }
            _ => {
                errors += 1;
            }
        }
    }

    Ok(Json(json!({
        "system_id": req.system_id,
        "system_name": system.name,
        "total": req.inputs.len(),
        "processed": results.len(),
        "errors": errors,
        "results": results,
    })))
}

async fn list_batch_results(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<Vec<BatchResult>>, AppError> {
    let rows = sqlx::query_as::<_, BatchResult>(
        "SELECT * FROM batch_results WHERE system_id = $1 ORDER BY row_index ASC"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows))
}

async fn delete_batch_result(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM batch_results WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Resultado batch não encontrado".into()));
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}
