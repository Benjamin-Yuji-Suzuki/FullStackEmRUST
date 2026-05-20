use axum::Router;
use leptos_config::LeptosOptions;
use server::routes::api_routes;
use server::state::AppState;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::OnceLock;
use tower::ServiceExt;

static TEST_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn get_test_pool() -> PgPool {
    if let Some(pool) = TEST_POOL.get() {
        return pool.clone();
    }
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://ben:1234@localhost/fuzzysimulated_test".into());
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to connect to test DB. Set DATABASE_URL or create 'fuzzysimulated_test' DB");
    let _ = TEST_POOL.set(pool.clone());
    pool
}

pub fn build_test_state(pool: PgPool) -> AppState {
    let opts = LeptosOptions::builder()
        .output_name("test")
        .build();
    AppState { pool, leptos_options: opts }
}

pub struct TestApp {
    pub router: Router,
}

impl TestApp {
    pub async fn new() -> Self {
        let pool = get_test_pool().await;
        let state = build_test_state(pool);
        let router = Router::new()
            .nest("/api", api_routes())
            .with_state(state);
        Self { router }
    }

    pub async fn call(
        &mut self,
        req: http::Request<axum::body::Body>,
    ) -> axum::response::Response {
        self.router
            .clone()
            .oneshot(req)
            .await
            .expect("Request failed")
    }
}
