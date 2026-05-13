use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BatchResult {
    pub id: Uuid,
    pub system_id: Uuid,
    pub source_file: String,
    pub row_index: i32,
    pub inputs: Value,
    pub output: f64,
    pub executed_at: DateTime<Utc>,
}
