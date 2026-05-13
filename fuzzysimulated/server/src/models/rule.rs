use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FuzzyRule {
    pub id: Uuid,
    pub system_id: Uuid,
    pub rule_text: String,
    pub weight: f64,
    pub position: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateRuleRequest {
    pub rule_text: String,
    pub weight: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRuleRequest {
    pub rule_text: String,
    pub weight: Option<f64>,
}
