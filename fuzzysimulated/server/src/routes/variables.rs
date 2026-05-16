use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;
use serde_json::json;

use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;
use crate::audit;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems/{id}/variables", get(list_variables).post(create_variable))
        .route("/variables/{id}", get(get_variable).put(update_variable).delete(delete_variable))
        .route("/variables/{id}/terms", post(create_term))
        .route("/terms/{id}", get(get_term).put(update_term).delete(delete_term))
}

async fn list_variables(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let variables = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE system_id = $1 ORDER BY name"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    let mut result = Vec::new();
    for var in &variables {
        let terms = sqlx::query_as::<_, FuzzyTerm>(
            "SELECT * FROM fuzzy_terms WHERE variable_id = $1 ORDER BY label"
        )
        .bind(var.id)
        .fetch_all(&state.pool)
        .await?;

        result.push(json!({
            "id": var.id,
            "system_id": var.system_id,
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

    Ok(Json(result))
}

async fn create_variable(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
    Json(req): Json<CreateVariableRequest>,
) -> Result<(axum::http::StatusCode, Json<FuzzyVariable>), AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Validation("O nome da variável é obrigatório".into()));
    }

    let role = req.role.trim().to_lowercase();
    if role != "antecedent" && role != "consequent" {
        return Err(AppError::Validation("Papel deve ser 'antecedent' ou 'consequent'".into()));
    }

    if role == "consequent" {
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM fuzzy_variables WHERE system_id = $1 AND role = 'consequent'"
        )
        .bind(system_id)
        .fetch_one(&state.pool)
        .await?;

        if existing > 0 {
            return Err(AppError::Validation("Este sistema já possui variável de saída (Mamdani permite apenas uma)".into()));
        }
    }

    if req.universe_min >= req.universe_max {
        return Err(AppError::Validation("universe_min deve ser menor que universe_max".into()));
    }

    let resolution = req.resolution.unwrap_or(501);
    if resolution < 2 {
        return Err(AppError::Validation("Resolução mínima é 2".into()));
    }

    let row = sqlx::query_as::<_, FuzzyVariable>(
        "INSERT INTO fuzzy_variables (system_id, name, role, universe_min, universe_max, resolution) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(system_id)
    .bind(&name)
    .bind(&role)
    .bind(req.universe_min)
    .bind(req.universe_max)
    .bind(resolution)
    .fetch_one(&state.pool)
    .await?;

    audit::log(&state.pool, system_id, "create", "variable",
        &format!("Variável '{}' adicionada como {}", row.name, row.role)).await;

    Ok((axum::http::StatusCode::CREATED, Json(row)))
}

async fn get_variable(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FuzzyVariable>, AppError> {
    let row = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Variável não encontrada".into()))?;

    Ok(Json(row))
}

async fn get_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FuzzyTerm>, AppError> {
    let row = sqlx::query_as::<_, FuzzyTerm>(
        "SELECT * FROM fuzzy_terms WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Termo não encontrado".into()))?;

    Ok(Json(row))
}

async fn update_variable(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateVariableRequest>,
) -> Result<Json<FuzzyVariable>, AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() { return Err(AppError::Validation("Nome obrigatório".into())); }

    let resolution = req.resolution.unwrap_or(501);
    if resolution < 2 { return Err(AppError::Validation("Resolução mínima é 2".into())); }

    let row = sqlx::query_as::<_, FuzzyVariable>(
        "UPDATE fuzzy_variables SET name = $1, universe_min = $2, universe_max = $3, resolution = $4 WHERE id = $5 RETURNING *"
    )
    .bind(&name).bind(req.universe_min).bind(req.universe_max).bind(resolution).bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Variável não encontrada".into()))?;

    Ok(Json(row))
}

async fn update_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateTermRequest>,
) -> Result<Json<FuzzyTerm>, AppError> {
    let label = req.label.trim().to_string();
    if label.is_empty() { return Err(AppError::Validation("Rótulo obrigatório".into())); }

    let params_json = json!(req.params);
    let row = sqlx::query_as::<_, FuzzyTerm>(
        "UPDATE fuzzy_terms SET label = $1, mf_type = $2, params = $3 WHERE id = $4 RETURNING *"
    )
    .bind(&label).bind(&req.mf_type).bind(&params_json).bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Termo não encontrada".into()))?;

    Ok(Json(row))
}

async fn delete_variable(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let var = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Variável não encontrada".into()))?;

    let sys_id = var.system_id;

    sqlx::query("DELETE FROM fuzzy_variables WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    audit::log(&state.pool, sys_id, "delete", "variable",
        &format!("Variável '{}' removida", var.name)).await;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn create_term(
    State(state): State<AppState>,
    Path(variable_id): Path<Uuid>,
    Json(req): Json<CreateTermRequest>,
) -> Result<(axum::http::StatusCode, Json<FuzzyTerm>), AppError> {
    let label = req.label.trim().to_string();
    if label.is_empty() {
        return Err(AppError::Validation("O rótulo do termo é obrigatório".into()));
    }

    let mf_type = req.mf_type.trim().to_lowercase();
    if !["trimf", "trapmf", "gaussmf"].contains(&mf_type.as_str()) {
        return Err(AppError::Validation("Tipo de MF deve ser 'trimf', 'trapmf' ou 'gaussmf'".into()));
    }

    match mf_type.as_str() {
        "trimf" => {
            if req.params.len() != 3 {
                return Err(AppError::Validation("trimf requer 3 parâmetros: [a, b, c]".into()));
            }
            if req.params[0] > req.params[1] || req.params[1] > req.params[2] {
                return Err(AppError::Validation("trimf: a ≤ b ≤ c".into()));
            }
        }
        "trapmf" => {
            if req.params.len() != 4 {
                return Err(AppError::Validation("trapmf requer 4 parâmetros: [a, b, c, d]".into()));
            }
            if req.params[0] > req.params[1] || req.params[1] > req.params[2] || req.params[2] > req.params[3] {
                return Err(AppError::Validation("trapmf: a ≤ b ≤ c ≤ d".into()));
            }
        }
        "gaussmf" => {
            if req.params.len() != 2 {
                return Err(AppError::Validation("gaussmf requer 2 parâmetros: [mean, sigma]".into()));
            }
            if req.params[1] <= 0.0 {
                return Err(AppError::Validation("gaussmf: sigma > 0".into()));
            }
        }
        _ => unreachable!(),
    }

    let var = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE id = $1"
    )
    .bind(variable_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Variável não encontrada".into()))?;

    let params_json = json!(req.params);

    let row = sqlx::query_as::<_, FuzzyTerm>(
        "INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(variable_id)
    .bind(&label)
    .bind(&mf_type)
    .bind(&params_json)
    .fetch_one(&state.pool)
    .await?;

    audit::log(&state.pool, var.system_id, "create", "term",
        &format!("Termo '{}' ({}) adicionado à '{}'", row.label, row.mf_type, var.name)).await;

    Ok((axum::http::StatusCode::CREATED, Json(row)))
}

async fn delete_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let var_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT variable_id FROM fuzzy_terms WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Termo não encontrado".into()))?;

    sqlx::query("DELETE FROM fuzzy_terms WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}
