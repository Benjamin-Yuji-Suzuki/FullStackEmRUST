use uuid::Uuid;

use crate::get_test_pool;
use crate::begin_test_tx;

#[ignore]
#[tokio::test]
async fn test_create_system_integration() {
    let pool = get_test_pool().await;
    let mut tx = begin_test_tx(&pool).await;

    let row = sqlx::query_as::<_, (Uuid, String, Option<String>, String)>(
        "INSERT INTO fuzzy_systems (name, description, defuzz_method) VALUES ($1, $2, $3) RETURNING id, name, description, defuzz_method"
    )
    .bind("Sistema Teste")
    .bind(Some("Descrição"))
    .bind("centroid")
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to insert system");

    assert_eq!(row.1, "Sistema Teste");
    assert_eq!(row.2, Some("Descrição".to_string()));
    assert_eq!(row.3, "centroid");
}

#[ignore]
#[tokio::test]
async fn test_cascade_delete_system() {
    let pool = get_test_pool().await;
    let mut tx = begin_test_tx(&pool).await;

    let sys = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO fuzzy_systems (name, defuzz_method) VALUES ('Teste', 'centroid') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    let var = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO fuzzy_variables (system_id, name, role, universe_min, universe_max, resolution) \
         VALUES ($1, 'temp', 'antecedent', 0, 50, 501) RETURNING id"
    )
    .bind(sys.0)
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES ($1, 'Frio', 'trimf', '[0,0,25]'::jsonb)"
    )
    .bind(var.0)
    .execute(&mut *tx)
    .await
    .unwrap();

    sqlx::query("DELETE FROM fuzzy_systems WHERE id = $1")
        .bind(sys.0)
        .execute(&mut *tx)
        .await
        .unwrap();

    let var_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM fuzzy_variables WHERE system_id = $1")
            .bind(sys.0)
            .fetch_one(&mut *tx)
            .await
            .unwrap();

    assert_eq!(var_count, 0, "Variáveis devem ser excluídas em cascata");
}
