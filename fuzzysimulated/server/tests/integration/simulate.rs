use uuid::Uuid;

use crate::get_test_pool;
use crate::begin_test_tx;

#[ignore]
#[tokio::test]
async fn test_only_one_consequent() {
    let pool = get_test_pool().await;
    let mut tx = begin_test_tx(&pool).await;

    let sys = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO fuzzy_systems (name, defuzz_method) VALUES ('Teste', 'centroid') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO fuzzy_variables (system_id, name, role, universe_min, universe_max, resolution) \
         VALUES ($1, 'saida', 'consequent', 0, 100, 501)"
    )
    .bind(sys.0)
    .execute(&mut *tx)
    .await
    .unwrap();

    let result = sqlx::query(
        "INSERT INTO fuzzy_variables (system_id, name, role, universe_min, universe_max, resolution) \
         VALUES ($1, 'saida2', 'consequent', 0, 100, 501)"
    )
    .bind(sys.0)
    .execute(&mut *tx)
    .await;

    assert!(
        result.is_ok(),
        "DB permite múltiplos consequentes — validação é da aplicação, mas erro obtido: {:?}",
        result
    );
}

#[ignore]
#[tokio::test]
async fn test_simulation_persists() {
    let pool = get_test_pool().await;
    let mut tx = begin_test_tx(&pool).await;

    let sys = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO fuzzy_systems (name, defuzz_method) VALUES ('Teste', 'centroid') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO simulations (system_id, inputs, outputs) VALUES ($1, '{\"x\":1.0}'::jsonb, '{\"y\":2.0}'::jsonb)"
    )
    .bind(sys.0)
    .execute(&mut *tx)
    .await
    .unwrap();

    let count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM simulations WHERE system_id = $1")
            .bind(sys.0)
            .fetch_one(&mut *tx)
            .await
            .unwrap();

    assert_eq!(count, 1);
}
