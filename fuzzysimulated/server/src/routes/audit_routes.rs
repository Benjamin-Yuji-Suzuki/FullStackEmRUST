use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::errors::AppError;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems/{id}/audit", get(list_audit))
        .route("/audit/{id}/undo", post(undo_event))
}

async fn list_audit(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
        "SELECT id::text, action_type, entity_type, description, created_at::text, system_id::text \
         FROM audit_events WHERE system_id = $1 ORDER BY created_at DESC"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    let events: Vec<serde_json::Value> = rows.into_iter().map(|(id, action, entity, desc, created, _sid)| {
        json!({
            "id": id,
            "action_type": action,
            "entity_type": entity,
            "description": desc,
            "created_at": created,
        })
    }).collect();

    Ok(Json(json!({ "events": events, "total": events.len() })))
}

async fn undo_event(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let event = sqlx::query_as::<_, (String, String)>(
        "SELECT action_type, description FROM audit_events WHERE id = $1"
    )
    .bind(event_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Evento não encontrado".into()))?;

    Ok(Json(json!({
        "message": "Undo registrado",
        "event_id": event_id,
        "action": event.0,
        "description": event.1,
    })))
}
