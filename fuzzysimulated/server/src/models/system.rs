use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FuzzySystem {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub defuzz_method: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSystemRequest {
    pub name: String,
    pub description: Option<String>,
    pub defuzz_method: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSystemRequest {
    pub name: String,
    pub description: Option<String>,
    pub defuzz_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}
