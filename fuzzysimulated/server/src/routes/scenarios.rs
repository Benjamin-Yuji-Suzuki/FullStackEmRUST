use axum::{
    extract::{Path, State},
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::AppError;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Scenario {
    pub id: Uuid,
    pub system_id: Uuid,
    pub name: String,
    pub inputs: Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems/{id}/scenarios", get(list_scenarios).post(create_scenario))
        .route("/scenarios/{id}", delete(delete_scenario))
}

async fn list_scenarios(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<Vec<Scenario>>, AppError> {
    let scenarios = sqlx::query_as::<_, Scenario>(
        "SELECT * FROM scenarios WHERE system_id = $1 ORDER BY created_at DESC"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(scenarios))
}

async fn create_scenario(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<Value>,
) -> Result<(axum::http::StatusCode, Json<Scenario>), AppError> {
    let name = req.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("Campo 'name' é obrigatório".into()))?
        .to_string();
    if name.trim().is_empty() {
        return Err(AppError::Validation("Nome do cenário não pode ser vazio".into()));
    }
    let inputs = req.get("inputs")
        .ok_or_else(|| AppError::Validation("Campo 'inputs' é obrigatório".into()))?
        .clone();

    let scenario = sqlx::query_as::<_, Scenario>(
        "INSERT INTO scenarios (system_id, name, inputs) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(system_id)
    .bind(&name)
    .bind(&inputs)
    .fetch_one(&state.pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(scenario)))
}

async fn delete_scenario(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM scenarios WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Cenário não encontrado".into()));
    }
    Ok(axum::http::StatusCode::NO_CONTENT)
}
