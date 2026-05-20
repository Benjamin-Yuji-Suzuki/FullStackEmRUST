use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::audit;
use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;
use server::math::{self, OptimizationInput};

/// Registra as rotas de otimização.
///
/// - `POST /api/optimize` — calcula o ponto ótimo de f(x,y)
/// - `GET /api/optimizations?system_id=UUID` — histórico de otimizações
/// - `GET /api/optimizations/{id}` — detalhe de uma otimização
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/optimize", post(compute_optimal_point))
        .route("/optimizations", get(list_optimizations))
        .route("/optimizations/{id}", get(get_optimization))
}

/// Calcula o ponto ótimo de uma função objetivo quadrática f(x,y) = ax² + bxy + cy² + dx + ey + const.
///
/// Resolve analiticamente o sistema ∇f = 0, classifica o ponto crítico
/// pela Hessiana e retorna a solução com explicação em linguagem natural.
///
/// # Errors
/// Retorna `AppError::Validation` se o sistema linear for singular (det = 0)
/// ou se o domínio for inválido.
async fn compute_optimal_point(
    State(state): State<AppState>,
    Json(req): Json<OptimizationRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let input = OptimizationInput {
        coef_a: req.coef_a,
        coef_b: req.coef_b,
        coef_c: req.coef_c,
        coef_d: req.coef_d,
        coef_e: req.coef_e,
        coef_f: req.coef_f,
        x_min: req.x_min,
        x_max: req.x_max,
        y_min: req.y_min,
        y_max: req.y_max,
    };

    let result = math::solve_quadratic_optimization(&input)
        .map_err(AppError::Validation)?;

    // Persistir no banco
    let x = result.optimal_x;
    let y = result.optimal_y;
    let value = result.optimal_value;
    let point_type = &result.critical_point_type;
    let explanation = &result.explanation;
    let gradient = result.gradient_at_optimum;
    let hessian = result.hessian_matrix;
    let record_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO optimizations \
         (system_id, coef_a, coef_b, coef_c, coef_d, coef_e, coef_f, \
          x_min, x_max, y_min, y_max, \
          optimal_x, optimal_y, optimal_value, critical_point_type, explanation, \
          gradient_at_optimum, hessian_matrix) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17::jsonb, $18::jsonb) \
         RETURNING id"
    )
    .bind(req.system_id)
    .bind(req.coef_a)
    .bind(req.coef_b)
    .bind(req.coef_c)
    .bind(req.coef_d)
    .bind(req.coef_e)
    .bind(req.coef_f)
    .bind(req.x_min)
    .bind(req.x_max)
    .bind(req.y_min)
    .bind(req.y_max)
    .bind(x)
    .bind(y)
    .bind(value)
    .bind(&point_type)
    .bind(&explanation)
    .bind(json!(gradient).to_string())
    .bind(json!(hessian).to_string())
    .fetch_one(&state.pool)
    .await?;

    // Audit se tiver system_id
    if let Some(sid) = req.system_id {
        audit::log(
            &state.pool,
            Some(sid),
            "create",
            "optimization",
            Some(record_id),
            &format!("Otimização calculada: ponto ({:.4}, {:.4}) — {}", x, y, point_type),
            None, serde_json::to_value(&Optimization {
                id: record_id,
                system_id: Some(sid),
                coef_a: req.coef_a, coef_b: req.coef_b,
                coef_c: req.coef_c, coef_d: req.coef_d,
                coef_e: req.coef_e, coef_f: req.coef_f,
                x_min: req.x_min, x_max: req.x_max,
                y_min: req.y_min, y_max: req.y_max,
                optimal_x: Some(x), optimal_y: Some(y),
                optimal_value: Some(value),
                critical_point_type: Some(point_type.clone()),
                explanation: Some(explanation.clone()),
                gradient_at_optimum: Some(json!(gradient)),
                hessian_matrix: Some(json!(hessian)),
                executed_at: chrono::Utc::now(),
            }).ok()).await;
    }

    Ok(Json(json!({
        "id": record_id,
        "optimal_x": x,
        "optimal_y": y,
        "optimal_value": value,
        "critical_point_type": point_type,
        "explanation": explanation,
        "gradient_at_optimum": gradient,
        "hessian_matrix": hessian,
        "coef_a": req.coef_a,
        "coef_b": req.coef_b,
        "coef_c": req.coef_c,
        "coef_d": req.coef_d,
        "coef_e": req.coef_e,
        "coef_f": req.coef_f,
    })))
}

/// Lista o histórico de otimizações de um sistema.
///
/// Query params: `system_id` (UUID, opcional)
async fn list_optimizations(
    State(state): State<AppState>,
    Query(params): Query<Option<OptimizationQuery>>,
) -> Result<Json<Vec<Optimization>>, AppError> {
    match params.and_then(|p| p.system_id) {
        Some(sid) => {
            let rows = sqlx::query_as::<_, Optimization>(
                "SELECT * FROM optimizations WHERE system_id = $1 ORDER BY executed_at DESC"
            )
            .bind(sid)
            .fetch_all(&state.pool)
            .await?;
            Ok(Json(rows))
        }
        None => {
            let rows = sqlx::query_as::<_, Optimization>(
                "SELECT * FROM optimizations ORDER BY executed_at DESC LIMIT 100"
            )
            .fetch_all(&state.pool)
            .await?;
            Ok(Json(rows))
        }
    }
}

/// Retorna os detalhes de uma otimização específica.
async fn get_optimization(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Optimization>, AppError> {
    let opt = sqlx::query_as::<_, Optimization>(
        "SELECT * FROM optimizations WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Otimização não encontrada".into()))?;

    Ok(Json(opt))
}

#[derive(Debug, Deserialize)]
struct OptimizationQuery {
    system_id: Option<Uuid>,
}
