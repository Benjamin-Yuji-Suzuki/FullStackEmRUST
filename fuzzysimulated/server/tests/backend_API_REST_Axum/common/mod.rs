use axum::Router;
use leptos_config::LeptosOptions;
use server::routes::api_routes;
use server::state::AppState;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower::ServiceExt;

pub async fn create_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://ben:1234@localhost/fuzzysimulated_test".into());
    PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to connect to test DB. Set DATABASE_URL or create 'fuzzysimulated_test' DB")
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
        let pool = create_test_pool().await;
        // Clean any data left by previous test runs
        let _ = sqlx::query("TRUNCATE fuzzy_systems CASCADE")
            .execute(&pool)
            .await;
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
