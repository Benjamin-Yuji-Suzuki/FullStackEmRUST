use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems", get(list_systems).post(create_system))
        .route("/systems/{id}", get(get_system).put(update_system).delete(delete_system))
}

async fn list_systems(
    State(state): State<AppState>,
) -> Result<Json<Vec<FuzzySystem>>, AppError> {
    let systems = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(systems))
}

async fn create_system(
    State(state): State<AppState>,
    Json(req): Json<CreateSystemRequest>,
) -> Result<(axum::http::StatusCode, Json<FuzzySystem>), AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Validation("O nome do sistema é obrigatório".into()));
    }
    if name.len() > 255 {
        return Err(AppError::Validation("O nome deve ter no máximo 255 caracteres".into()));
    }

    let defuzz = req.defuzz_method.unwrap_or_else(|| "centroid".into());
    let valid_methods = ["centroid", "bisector", "mom", "lom", "som"];
    if !valid_methods.contains(&defuzz.as_str()) {
        return Err(AppError::Validation(format!(
            "Método de defuzzificação inválido: {defuzz}"
        )));
    }

    let system = sqlx::query_as::<_, FuzzySystem>(
        "INSERT INTO fuzzy_systems (name, description, defuzz_method) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&name)
    .bind(&req.description)
    .bind(&defuzz)
    .fetch_one(&state.pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(system)))
}

async fn get_system(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FuzzySystem>, AppError> {
    let system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    Ok(Json(system))
}

async fn update_system(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSystemRequest>,
) -> Result<Json<FuzzySystem>, AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Validation("O nome do sistema é obrigatório".into()));
    }
    if name.len() > 255 {
        return Err(AppError::Validation("O nome deve ter no máximo 255 caracteres".into()));
    }

    let system = sqlx::query_as::<_, FuzzySystem>(
        "UPDATE fuzzy_systems SET name = $1, description = $2, defuzz_method = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(&name)
    .bind(&req.description)
    .bind(req.defuzz_method.unwrap_or_else(|| "centroid".into()))
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    Ok(Json(system))
}

async fn delete_system(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM fuzzy_systems WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Sistema não encontrado".into()));
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}
