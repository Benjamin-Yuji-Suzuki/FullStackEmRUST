use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FuzzyVariable {
    pub id: Uuid,
    pub system_id: Uuid,
    pub name: String,
    pub role: String,
    pub universe_min: f64,
    pub universe_max: f64,
    pub resolution: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateVariableRequest {
    pub name: String,
    pub role: String,
    pub universe_min: f64,
    pub universe_max: f64,
    pub resolution: Option<i32>,
}
