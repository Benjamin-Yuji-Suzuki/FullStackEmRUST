use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Simulation {
    pub id: Uuid,
    pub system_id: Uuid,
    pub inputs: Value,
    pub outputs: Value,
    pub weather_data: Option<Value>,
    pub city: Option<String>,
    pub executed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SimulateRequest {
    pub inputs: std::collections::HashMap<String, f64>,
}
