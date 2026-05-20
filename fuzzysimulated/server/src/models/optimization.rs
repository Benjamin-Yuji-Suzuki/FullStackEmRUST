use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

/// Representa uma otimização de função objetivo quadrática f(x,y) = ax² + bxy + cy² + dx + ey + f.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Optimization {
    pub id: Uuid,
    pub system_id: Option<Uuid>,
    pub coef_a: f64,
    pub coef_b: f64,
    pub coef_c: f64,
    pub coef_d: f64,
    pub coef_e: f64,
    pub coef_f: f64,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub optimal_x: Option<f64>,
    pub optimal_y: Option<f64>,
    pub optimal_value: Option<f64>,
    pub critical_point_type: Option<String>,
    pub explanation: Option<String>,
    pub gradient_at_optimum: Option<Value>,
    pub hessian_matrix: Option<Value>,
    pub executed_at: DateTime<Utc>,
}

/// Requisição de otimização enviada pelo front-end.
#[derive(Debug, Deserialize)]
pub struct OptimizationRequest {
    pub system_id: Option<Uuid>,
    pub coef_a: f64,
    pub coef_b: f64,
    pub coef_c: f64,
    pub coef_d: f64,
    pub coef_e: f64,
    pub coef_f: f64,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

/// Resultado da otimização retornado pela API.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub optimal_x: f64,
    pub optimal_y: f64,
    pub optimal_value: f64,
    pub critical_point_type: String,
    pub explanation: String,
    pub gradient_at_optimum: [f64; 2],
    pub hessian_matrix: [[f64; 2]; 2],
}
