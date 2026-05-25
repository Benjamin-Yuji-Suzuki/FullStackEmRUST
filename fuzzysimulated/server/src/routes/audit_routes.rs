use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::audit;
use crate::errors::AppError;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems/{id}/audit", get(list_audit))
        .route("/audit/orphans", get(list_orphan_audit))
        .route("/audit/{id}/undo", post(undo_event))
}

fn entity_table(entity_type: &str) -> Result<&'static str, AppError> {
    match entity_type {
        "system" => Ok("fuzzy_systems"),
        "variable" => Ok("fuzzy_variables"),
        "term" => Ok("fuzzy_terms"),
        "rule" => Ok("fuzzy_rules"),
        "optimization" => Ok("optimizations"),
        other => Err(AppError::Validation(format!("entity_type desconhecido: {other}"))),
    }
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

    let events: Vec<serde_json::Value> = rows.into_iter().map(|(id, action, entity, desc, created, sid)| {
        json!({
            "id": id,
            "system_id": sid,
            "action_type": action,
            "entity_type": entity,
            "description": desc,
            "created_at": created,
        })
    }).collect();

    Ok(Json(json!({ "events": events, "total": events.len() })))
}

async fn list_orphan_audit(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, Option<String>)>(
        "SELECT id::text, action_type, entity_type, description, created_at::text, system_id::text \
         FROM audit_events WHERE system_id IS NULL ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;

    let events: Vec<serde_json::Value> = rows.into_iter().map(|(id, action, entity, desc, created, sid)| {
        json!({
            "id": id,
            "system_id": sid,
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
    let event = sqlx::query_as::<_, (String, String, Option<Uuid>, Option<Value>, Option<Value>, Option<Uuid>, String)>(
        "SELECT action_type, entity_type, entity_id, snapshot_before, snapshot_after, system_id, description \
         FROM audit_events WHERE id = $1"
    )
    .bind(event_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Evento não encontrado".into()))?;

    let (action_type, entity_type, entity_id, snapshot_before, _snapshot_after, system_id, description) = event;

    if action_type.starts_with("undo-") {
        return Err(AppError::Validation("Este evento já foi desfeito".into()));
    }

    let restored_system_id = if action_type == "delete" && entity_type == "system" {
        snapshot_before.as_ref().and_then(|s| {
            s.get("system").or(Some(s))
                .and_then(|v| v.get("id"))
                .and_then(|v| v.as_str())
                .and_then(|id| Uuid::parse_str(id).ok())
        })
    } else {
        None
    };

    let table = entity_table(&entity_type)?;

    match action_type.as_str() {
        "delete" => {
            let snapshot = snapshot_before.ok_or_else(|| {
                AppError::Validation("Snapshot anterior não disponível para undo".into())
            })?;

            if entity_type == "system" && snapshot.get("system").and_then(|v| v.as_object()).is_some() {
                restore_system_snapshot(&state.pool, &snapshot).await?;
            } else {
                let sql = format!(
                    "INSERT INTO {} SELECT * FROM jsonb_populate_record(NULL::{}, $1::jsonb)",
                    table, table
                );
                sqlx::query(&sql)
                    .bind(snapshot)
                    .execute(&state.pool)
                    .await?;
            }
        }
        "update" => {
            let snapshot = snapshot_before.ok_or_else(|| {
                AppError::Validation("Snapshot anterior não disponível para undo".into())
            })?;
            let eid = entity_id.ok_or_else(|| {
                AppError::Validation("ID da entidade não disponível para undo".into())
            })?;
            let sql = format!(
                "UPDATE {} AS t SET ({}) = (SELECT {} FROM jsonb_populate_record(NULL::{}, $1::jsonb)) WHERE t.id = $2",
                table,
                snapshot_object_fields(&snapshot),
                snapshot_object_fields(&snapshot),
                table,
            );
            sqlx::query(&sql)
                .bind(snapshot)
                .bind(eid)
                .execute(&state.pool)
                .await?;
        }
        "create" => {
            let eid = entity_id.ok_or_else(|| {
                AppError::Validation("ID da entidade não disponível para undo".into())
            })?;
            let sql = format!("DELETE FROM {} WHERE id = $1", table);
            sqlx::query(&sql)
                .bind(eid)
                .execute(&state.pool)
                .await?;
        }
        _ => {
            return Err(AppError::Validation(format!(
                "Tipo de ação '{}' não suportada para undo",
                action_type
            )));
        }
    }

    // Re-link ao sistema restaurado (remove da lista de órfãos)
    if let Some(sys_id) = restored_system_id {
        sqlx::query("UPDATE audit_events SET system_id = $1 WHERE id = $2")
            .bind(sys_id)
            .bind(event_id)
            .execute(&state.pool)
            .await?;
    }

    // Marcar evento original como desfeito
    sqlx::query("UPDATE audit_events SET action_type = 'undo-' || action_type WHERE id = $1")
        .bind(event_id)
        .execute(&state.pool)
        .await?;

    // Registrar novo evento de auditoria para o undo
    audit::log(
        &state.pool,
        system_id,
        "undo",
        &entity_type,
        entity_id,
        &format!("Desfez '{}' em {}: {}", action_type, entity_type, description),
        None,
        None,
    )
    .await;

    Ok(Json(json!({
        "message": "Undo executado com sucesso",
        "event_id": event_id,
        "action": action_type,
        "entity_type": entity_type,
    })))
}

async fn restore_system_snapshot(
    pool: &sqlx::PgPool,
    snapshot: &Value,
) -> Result<(), AppError> {
    let sys = snapshot.get("system").ok_or_else(|| {
        AppError::Validation("Snapshot de sistema inválido: 'system' ausente".into())
    })?;

    sqlx::query(
        "INSERT INTO fuzzy_systems SELECT * FROM jsonb_populate_record(NULL::fuzzy_systems, $1::jsonb)"
    )
    .bind(sys.clone())
    .execute(pool)
    .await?;

    if let Some(vars) = snapshot.get("variables").and_then(|v| v.as_array()) {
        for entry in vars {
            if let Some(var_val) = entry.get("variable") {
                sqlx::query(
                    "INSERT INTO fuzzy_variables SELECT * FROM jsonb_populate_record(NULL::fuzzy_variables, $1::jsonb)"
                )
                .bind(var_val.clone())
                .execute(pool)
                .await?;
            }
            if let Some(terms) = entry.get("terms").and_then(|t| t.as_array()) {
                for term in terms {
                    sqlx::query(
                        "INSERT INTO fuzzy_terms SELECT * FROM jsonb_populate_record(NULL::fuzzy_terms, $1::jsonb)"
                    )
                    .bind(term.clone())
                    .execute(pool)
                    .await?;
                }
            }
        }
    }

    if let Some(rules) = snapshot.get("rules").and_then(|r| r.as_array()) {
        for rule in rules {
            sqlx::query(
                "INSERT INTO fuzzy_rules SELECT * FROM jsonb_populate_record(NULL::fuzzy_rules, $1::jsonb)"
            )
            .bind(rule.clone())
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

fn snapshot_object_fields(snapshot: &Value) -> String {
    match snapshot {
        Value::Object(map) => map.keys().cloned().collect::<Vec<_>>().join(", "),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{entity_table, snapshot_object_fields};
    use serde_json::json;

    #[test]
    fn test_entity_table_system() {
        assert_eq!(entity_table("system").unwrap(), "fuzzy_systems");
    }

    #[test]
    fn test_entity_table_variable() {
        assert_eq!(entity_table("variable").unwrap(), "fuzzy_variables");
    }

    #[test]
    fn test_entity_table_term() {
        assert_eq!(entity_table("term").unwrap(), "fuzzy_terms");
    }

    #[test]
    fn test_entity_table_rule() {
        assert_eq!(entity_table("rule").unwrap(), "fuzzy_rules");
    }

    #[test]
    fn test_entity_table_optimization() {
        assert_eq!(entity_table("optimization").unwrap(), "optimizations");
    }

    #[test]
    fn test_entity_table_unknown() {
        assert!(entity_table("invalid").is_err());
    }

    #[test]
    fn test_snapshot_object_fields_extracts_keys() {
        let v = json!({"id": "abc", "name": "test", "value": 42});
        let result = snapshot_object_fields(&v);
        let fields: Vec<&str> = result.split(", ").collect();
        assert_eq!(fields.len(), 3);
        assert!(fields.contains(&"id"));
        assert!(fields.contains(&"name"));
        assert!(fields.contains(&"value"));
    }

    #[test]
    fn test_snapshot_object_fields_non_object() {
        assert_eq!(snapshot_object_fields(&json!("string")), "");
        assert_eq!(snapshot_object_fields(&json!(123)), "");
        assert_eq!(snapshot_object_fields(&json!(null)), "");
    }

    #[test]
    fn test_snapshot_object_fields_empty() {
        assert_eq!(snapshot_object_fields(&json!({})), "");
    }
}
