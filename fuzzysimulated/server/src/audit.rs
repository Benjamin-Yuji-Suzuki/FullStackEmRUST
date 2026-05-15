use sqlx::PgPool;
use uuid::Uuid;

pub async fn log(pool: &PgPool, system_id: Uuid, action_type: &str, entity_type: &str, description: &str) {
    let _ = sqlx::query(
        "INSERT INTO audit_events (system_id, action_type, entity_type, description) VALUES ($1, $2, $3, $4)"
    )
    .bind(system_id)
    .bind(action_type)
    .bind(entity_type)
    .bind(description)
    .execute(pool)
    .await;
}
