mod unit;
mod integration_db;
#[allow(non_snake_case)]
mod backend_API_REST_Axum;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::OnceLock;

static TEST_POOL: OnceLock<PgPool> = OnceLock::new();

async fn get_test_pool() -> PgPool {
    if let Some(pool) = TEST_POOL.get() {
        return pool.clone();
    }
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost/fuzzysimulated_test".into());
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test DB. Set DATABASE_URL or create 'fuzzysimulated_test' DB");
    let _ = TEST_POOL.set(pool.clone());
    pool
}

pub(crate) async fn begin_test_tx(
    pool: &PgPool,
) -> sqlx::Transaction<'static, sqlx::Postgres> {
    pool.begin().await.expect("Failed to begin test transaction")
}
