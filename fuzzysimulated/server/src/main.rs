use axum::Router;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use app::*;
use log::info;
use sqlx::postgres::PgPoolOptions;

mod errors;
mod models;
mod routes;
mod state;

use state::AppState;

async fn fallback_handler(
    uri: axum::http::Uri,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Response {
    let root: &str = &state.leptos_options.site_root;
    let path = uri.path().trim_start_matches('/');
    let file_path = std::path::Path::new(root).join(path);

    match tokio::fs::read(&file_path).await {
        Ok(data) => {
            let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
            axum::response::Response::builder()
                .header("Content-Type", mime.as_ref())
                .body(axum::body::Body::from(data))
                .unwrap()
        }
        Err(_) => (
            axum::http::StatusCode::NOT_FOUND,
            format!("Resource not found: {uri}"),
        ).into_response(),
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost/fuzzysimulated".into());

    info!("connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database. Check DATABASE_URL in .env");
    info!("database connected");

    let migrator = sqlx::migrate::Migrator::new(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"))
    )
    .await
    .expect("Failed to load migrations");
    migrator.run(&pool).await.expect("Failed to run migrations");
    info!("migrations applied");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let site_root = leptos_options.site_root.clone();

    #[cfg(feature = "ssr")] {
        app::server_fns::init_pool(pool.clone());
    }

    let app_state = AppState {
        pool: pool.clone(),
        leptos_options: leptos_options.clone(),
    };

    let app = Router::new()
        .nest("/api", routes::api_routes())
        .leptos_routes(&app_state, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(fallback_handler)
        .with_state(app_state)
        .layer(
            tower::ServiceBuilder::new()
                .layer(tower_http::cors::CorsLayer::permissive()),
        );

    info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
