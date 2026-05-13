use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FuzzyTerm {
    pub id: Uuid,
    pub variable_id: Uuid,
    pub label: String,
    pub mf_type: String,
    pub params: Value,
}

#[derive(Debug, Deserialize)]
pub struct CreateTermRequest {
    pub label: String,
    pub mf_type: String,
    pub params: Vec<f64>,
}
