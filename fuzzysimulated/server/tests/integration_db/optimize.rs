use uuid::Uuid;

use crate::get_test_pool;
use crate::begin_test_tx;

#[ignore]
#[tokio::test]
async fn test_optimization_persists_in_db() {
    let pool = get_test_pool().await;
    let mut tx = begin_test_tx(&pool).await;

    let sys = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO fuzzy_systems (name, defuzz_method) VALUES ('Teste', 'centroid') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO optimizations (system_id, coef_a, coef_b, coef_c, coef_d, coef_e, coef_f, \
         x_min, x_max, y_min, y_max, optimal_x, optimal_y, optimal_value, critical_point_type, explanation) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"
    )
    .bind(sys.0)
    .bind(1.0).bind(0.0).bind(1.0).bind(0.0).bind(0.0).bind(0.0)
    .bind(-10.0).bind(10.0).bind(-10.0).bind(10.0)
    .bind(0.0).bind(0.0).bind(0.0)
    .bind("mínimo")
    .bind("det(H) > 0 → mínimo local")
    .execute(&mut *tx)
    .await
    .expect("Inserção de otimização falhou");

    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM optimizations WHERE system_id = $1"
    )
    .bind(sys.0)
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    assert_eq!(count, 1, "Deveria haver 1 otimização salva, obteve {count}");
}

#[ignore]
#[tokio::test]
async fn test_optimization_retrieves_correct_columns() {
    let pool = get_test_pool().await;
    let mut tx = begin_test_tx(&pool).await;

    let sys = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO fuzzy_systems (name, defuzz_method) VALUES ('Teste', 'centroid') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO optimizations (system_id, coef_a, coef_b, coef_c, coef_d, coef_e, coef_f, \
         x_min, x_max, y_min, y_max, optimal_x, optimal_y, optimal_value, critical_point_type, explanation) \
         VALUES ($1, 2, 3, 4, 5, 6, 7, -8, 8, -9, 9, 0.5, 0.25, 3.14, 'mínimo', 'explicação de teste')"
    )
    .bind(sys.0)
    .execute(&mut *tx)
    .await
    .unwrap();

    let row = sqlx::query_as::<_, (f64, f64, f64, String)>(
        "SELECT optimal_x, optimal_y, optimal_value, critical_point_type \
         FROM optimizations WHERE system_id = $1"
    )
    .bind(sys.0)
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    assert!((row.0 - 0.5).abs() < 1e-6, "optimal_x = {}, esperado 0.5", row.0);
    assert!((row.1 - 0.25).abs() < 1e-6, "optimal_y = {}, esperado 0.25", row.1);
    assert!((row.2 - 3.14).abs() < 1e-6, "optimal_value = {}, esperado 3.14", row.2);
    assert_eq!(row.3, "mínimo", "critical_point_type = {}, esperado 'mínimo'", row.3);
}
