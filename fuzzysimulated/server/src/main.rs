use axum::Router;
use axum::response::IntoResponse;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use app::*;
use log::info;
use sqlx::postgres::PgPoolOptions;

mod audit;
mod errors;
mod models;
mod routes;
mod state;

use state::AppState;

use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeader;

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

    let app_state = AppState {
        pool: pool.clone(),
        leptos_options: leptos_options.clone(),
    };

    let site_root = std::path::PathBuf::from("target/site");
    let pkg_dir = site_root.join("pkg");

    // serve /pkg/ directly as static files (before leptos routes)
    let pkg_svc = tower_http::services::ServeDir::new(&pkg_dir)
        .precompressed_gzip()
        .precompressed_br()
        .append_index_html_on_directories(false);

    let app = Router::new()
        .nest("/api", routes::api_routes())
        .nest_service("/pkg", pkg_svc)
        .leptos_routes(&app_state, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(axum::routing::any_service(
            tower_http::services::ServeDir::new(&site_root)
                .append_index_html_on_directories(false)
        ))
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
