pub mod systems;
pub mod variables;
pub mod rules;
pub mod simulate;
pub mod weather;
pub mod audit_routes;
pub mod optimize;
pub mod scenarios;
pub mod batch;

use axum::Router;
use crate::state::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(systems::routes())
        .merge(variables::routes())
        .merge(rules::routes())
        .merge(simulate::routes())
        .merge(weather::routes())
        .merge(audit_routes::routes())
        .merge(optimize::routes())
        .merge(scenarios::routes())
        .merge(batch::routes())
}
