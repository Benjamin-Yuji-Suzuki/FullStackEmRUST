use sqlx::PgPool;
use uuid::Uuid;

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
/// - `description`: Descrição legível em português do evento ocorrido
///
/// # Observação
/// Esta função é fire-and-forget: erros de INSERT são ignorados para não
/// bloquear a operação principal.
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
