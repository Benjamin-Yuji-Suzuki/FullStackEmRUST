use axum::{
    extract::{Form, Path, State},
    response::Redirect,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::audit;
use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateSystemForm {
    pub name: String,
    pub description: Option<String>,
    pub defuzz_method: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems", get(list_systems).post(create_system))
        .route("/systems/{id}", get(get_system).put(update_system).delete(delete_system))
        .route("/systems/{id}/status", put(update_system_status))
        .route("/sys/create", post(create_system_form))
        .route("/sys/{id}/delete", post(delete_system_form))
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

    audit::log(&state.pool, Some(system.id), "create", "system",
        Some(system.id),
        &format!("Sistema '{}' criado", system.name),
        None, serde_json::to_value(&system).ok()).await;

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

    let old = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let system = sqlx::query_as::<_, FuzzySystem>(
        "UPDATE fuzzy_systems SET name = $1, description = $2, defuzz_method = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(&name)
    .bind(&req.description)
    .bind(req.defuzz_method.unwrap_or_else(|| "centroid".into()))
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    audit::log(&state.pool, Some(id), "update", "system",
        Some(id),
        &format!("Sistema '{}' atualizado", system.name),
        serde_json::to_value(&old).ok(), serde_json::to_value(&system).ok()).await;

    Ok(Json(system))
}

async fn delete_system(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    if system.status == "favorito" {
        return Err(AppError::Validation("Sistema favoritado não pode ser deletado. Remova o favorito primeiro.".into()));
    }

    let snapshot = build_system_snapshot(&state.pool, id, &system).await?;

    audit::log(&state.pool, Some(id), "delete", "system",
        Some(id),
        &format!("Sistema '{}' removido", system.name),
        Some(snapshot), None).await;

    sqlx::query("DELETE FROM fuzzy_systems WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn create_system_form(
    State(state): State<AppState>,
    Form(req): Form<CreateSystemForm>,
) -> Result<Redirect, AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() { return Err(AppError::Validation("Nome obrigatório".into())); }
    let defuzz = req.defuzz_method.unwrap_or_else(|| "centroid".into());
    let system = sqlx::query_as::<_, FuzzySystem>(
        "INSERT INTO fuzzy_systems (name, description, defuzz_method) VALUES ($1, $2, $3) RETURNING *"
    )
        .bind(&name).bind(&req.description).bind(&defuzz)
        .fetch_one(&state.pool).await?;

    audit::log(&state.pool, Some(system.id), "create", "system",
        Some(system.id),
        &format!("Sistema '{}' criado", system.name),
        None, serde_json::to_value(&system).ok()).await;

    Ok(Redirect::to("/"))
}

async fn delete_system_form(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Redirect, AppError> {
    let system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    if system.status == "favorito" {
        return Err(AppError::Validation("Sistema favoritado não pode ser deletado. Remova o favorito primeiro.".into()));
    }

    let snapshot = build_system_snapshot(&state.pool, id, &system).await?;

    audit::log(&state.pool, Some(id), "delete", "system",
        Some(id),
        &format!("Sistema '{}' removido", system.name),
        Some(snapshot), None).await;

    sqlx::query("DELETE FROM fuzzy_systems WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(Redirect::to("/"))
}

async fn update_system_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<FuzzySystem>, AppError> {
    let valid_statuses = ["ativo", "favorito", "concluido", "desativado"];
    if !valid_statuses.contains(&req.status.as_str()) {
        return Err(AppError::Validation(format!(
            "Status inválido: '{}'. Valores: ativo, favorito, concluido, desativado", req.status
        )));
    }

    let system = sqlx::query_as::<_, FuzzySystem>(
        "UPDATE fuzzy_systems SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(&req.status)
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    audit::log(&state.pool, Some(system.id), "update", "system",
        Some(system.id),
        &format!("Status do sistema '{}' alterado para '{}'", system.name, system.status),
        None, serde_json::to_value(&system).ok()).await;

    Ok(Json(system))
}

async fn build_system_snapshot(
    pool: &sqlx::PgPool,
    id: Uuid,
    system: &FuzzySystem,
) -> Result<serde_json::Value, AppError> {
    let variables: Vec<FuzzyVariable> = sqlx::query_as(
        "SELECT * FROM fuzzy_variables WHERE system_id = $1"
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let mut vars_with_terms = Vec::new();
    for var in &variables {
        let terms: Vec<FuzzyTerm> = sqlx::query_as(
            "SELECT * FROM fuzzy_terms WHERE variable_id = $1"
        )
        .bind(var.id)
        .fetch_all(pool)
        .await?;
        vars_with_terms.push(json!({
            "variable": var,
            "terms": terms,
        }));
    }

    let rules: Vec<FuzzyRule> = sqlx::query_as(
        "SELECT * FROM fuzzy_rules WHERE system_id = $1"
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(json!({
        "system": system,
        "variables": vars_with_terms,
        "rules": rules,
    }))
}
