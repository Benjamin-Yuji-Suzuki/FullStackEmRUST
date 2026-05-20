use uuid::Uuid;

use crate::get_test_pool;
use crate::begin_test_tx;

#[ignore]
#[tokio::test]
async fn test_create_variable_integration() {
    let pool = get_test_pool().await;
    let mut tx = begin_test_tx(&pool).await;

    let sys = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO fuzzy_systems (name, defuzz_method) VALUES ('Teste', 'centroid') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    let var = sqlx::query_as::<_, (Uuid, Uuid, String, String, f64, f64, i32)>(
        "INSERT INTO fuzzy_variables (system_id, name, role, universe_min, universe_max, resolution) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(sys.0)
    .bind("temperatura")
    .bind("antecedent")
    .bind(0.0)
    .bind(50.0)
    .bind(501)
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    assert_eq!(var.2, "temperatura");
    assert_eq!(var.3, "antecedent");
    assert_eq!(var.4, 0.0);
    assert_eq!(var.5, 50.0);
}

#[ignore]
#[tokio::test]
async fn test_create_term_integration() {
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

    let term = sqlx::query_as::<_, (String, String, serde_json::Value)>(
        "INSERT INTO fuzzy_terms (variable_id, label, mf_type, params) VALUES ($1, 'Frio', 'trimf', '[0,0,25]'::jsonb) RETURNING label, mf_type, params"
    )
    .bind(var.0)
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    assert_eq!(term.0, "Frio");
    assert_eq!(term.1, "trimf");
}
