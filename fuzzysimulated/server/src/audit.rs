use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value;

/// Registra um evento de auditoria no banco de dados.
///
/// Toda mutação (create/update/delete) de entidades do domínio DEVE chamar
/// esta função para manter a trilha de auditoria (`audit_events`).
///
/// # Parâmetros
/// - `pool`: Conexão com o banco PostgreSQL
/// - `system_id`: UUID do sistema fuzzy associado ao evento
/// - `action_type`: Tipo da ação (`"create"`, `"update"`, `"delete"`, etc.)
/// - `entity_type`: Tipo da entidade (`"system"`, `"variable"`, `"term"`, `"rule"`, `"optimization"`)
/// - `entity_id`: UUID da entidade alvo (para undo)
/// - `description`: Descrição legível em português do evento ocorrido
/// - `snapshot_before`: Estado da entidade antes da mutação (para undo de update/delete)
/// - `snapshot_after`: Estado da entidade após a mutação (para undo de create/update)
///
/// # Observação
/// Esta função é fire-and-forget: erros de INSERT são ignorados para não
/// bloquear a operação principal.
pub async fn log(
    pool: &PgPool,
    system_id: Option<Uuid>,
    action_type: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    description: &str,
    snapshot_before: Option<Value>,
    snapshot_after: Option<Value>,
) {
    let _ = sqlx::query(
        "INSERT INTO audit_events (system_id, action_type, entity_type, entity_id, description, snapshot_before, snapshot_after) \
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(system_id)
    .bind(action_type)
    .bind(entity_type)
    .bind(entity_id)
    .bind(description)
    .bind(snapshot_before)
    .bind(snapshot_after)
    .execute(pool)
    .await;
}
