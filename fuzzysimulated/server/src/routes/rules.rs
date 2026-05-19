use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;
use crate::audit;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems/{id}/rules", post(create_rule))
        .route("/rules/{id}", get(get_rule).put(update_rule).delete(delete_rule))
}

async fn create_rule(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<CreateRuleRequest>,
) -> Result<(axum::http::StatusCode, Json<FuzzyRule>), AppError> {
    let text = req.rule_text.trim().to_string();
    if text.is_empty() {
        return Err(AppError::Validation("O texto da regra é obrigatório".into()));
    }

    let weight = req.weight.unwrap_or(1.0);
    if !(0.0..=1.0).contains(&weight) {
        return Err(AppError::Validation("Peso deve estar entre 0.0 e 1.0".into()));
    }

    // get next position
    let max_pos = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(position) FROM fuzzy_rules WHERE system_id = $1"
    )
    .bind(system_id)
    .fetch_one(&state.pool)
    .await?;

    let position = max_pos.unwrap_or(-1) + 1;

    let row = sqlx::query_as::<_, FuzzyRule>(
        "INSERT INTO fuzzy_rules (system_id, rule_text, weight, position) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(system_id)
    .bind(&text)
    .bind(weight)
    .bind(position)
    .fetch_one(&state.pool)
    .await?;

    audit::log(&state.pool, system_id, "create", "rule",
        &format!("Regra #{} adicionada: {}", row.position, row.rule_text)).await;

    Ok((axum::http::StatusCode::CREATED, Json(row)))
}

async fn get_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FuzzyRule>, AppError> {
    let row = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Regra não encontrada".into()))?;

    Ok(Json(row))
}

async fn update_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateRuleRequest>,
) -> Result<Json<FuzzyRule>, AppError> {
    let text = req.rule_text.trim().to_string();
    if text.is_empty() {
        return Err(AppError::Validation("O texto da regra é obrigatório".into()));
    }

    let weight = req.weight.unwrap_or(1.0);
    if !(0.0..=1.0).contains(&weight) {
        return Err(AppError::Validation("Peso deve estar entre 0.0 e 1.0".into()));
    }

    let row = sqlx::query_as::<_, FuzzyRule>(
        "UPDATE fuzzy_rules SET rule_text = $1, weight = $2 WHERE id = $3 RETURNING *"
    )
    .bind(&text)
    .bind(weight)
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Regra não encontrada".into()))?;

    Ok(Json(row))
}

async fn delete_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let rule = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Regra não encontrada".into()))?;

    let sys_id = rule.system_id;

    sqlx::query("DELETE FROM fuzzy_rules WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    audit::log(&state.pool, sys_id, "delete", "rule",
        &format!("Regra removida: {}", rule.rule_text)).await;

    Ok(axum::http::StatusCode::NO_CONTENT)
}
