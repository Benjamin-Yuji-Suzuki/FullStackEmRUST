use axum::body::Body;
use axum::http::Request;
use serde_json::Value;

use server::routes::api_routes;
use server::state::AppState;
use sqlx::postgres::PgPoolOptions;

async fn get_weather_json(city: &str) -> Result<(u16, Value), String> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://ben:1234@localhost/fuzzysimulated_test".into());
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .map_err(|e| format!("DB: {e}"))?;
    let opts = leptos_config::LeptosOptions::builder()
        .output_name("test")
        .build();
    let state = AppState { pool, leptos_options: opts };
    let router = axum::Router::new()
        .nest("/api", api_routes())
        .with_state(state);

    let req = Request::get(&format!("/api/weather?city={city}"))
        .body(Body::empty())
        .map_err(|e| format!("Request: {e}"))?;

    let resp = match tower::ServiceExt::oneshot(router, req).await {
        Ok(r) => r,
        Err(e) => return Err(format!("Erro ao chamar rota: {e:?}")),
    };
    let status = resp.status().as_u16();
    let body_bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .map_err(|e| format!("Body: {e}"))?;
    let json: Value = serde_json::from_slice(&body_bytes)
        .map_err(|e| format!("JSON: {e}"))?;
    Ok((status, json))
}

fn has_api_key() -> bool {
    dotenvy::dotenv().ok();
    std::env::var("OPENWEATHER_API_KEY")
        .ok()
        .map_or(false, |k| !k.is_empty())
}

#[tokio::test]
#[ignore]
async fn test_weather_integration_belem() {
    if !has_api_key() {
        eprintln!("SKIP: OPENWEATHER_API_KEY não configurada");
        return;
    }
    let (status, body) = get_weather_json("Belem").await.unwrap();
    assert_eq!(status, 200, "Esperava 200, obteve {status}: {body:?}");
    assert_eq!(body["city"], "Belem");
    assert!(body["temp"].as_f64().is_some(), "temperatura deve ser numérica");
    assert!(body["humidity"].as_f64().is_some(), "umidade deve ser numérica");
    assert!(body["description"].as_str().map_or(false, |d| !d.is_empty()), "descrição não pode ser vazia");
}

#[tokio::test]
#[ignore]
async fn test_weather_integration_sao_paulo() {
    if !has_api_key() {
        eprintln!("SKIP: OPENWEATHER_API_KEY não configurada");
        return;
    }
    let (status, body) = get_weather_json("S%C3%A3o%20Paulo").await.unwrap();
    assert_eq!(status, 200, "Esperava 200, obteve {status}: {body:?}");
    assert_eq!(body["city"], "São Paulo");
    assert!(body["temp"].as_f64().is_some(), "temperatura deve ser numérica");
}

#[tokio::test]
#[ignore]
async fn test_weather_integration_invalid_city() {
    if !has_api_key() {
        eprintln!("SKIP: OPENWEATHER_API_KEY não configurada");
        return;
    }
    let (status, body) = get_weather_json("CidadeInexistenteXYZ123").await.unwrap();
    assert_eq!(status, 404, "Esperava cidade não encontrada, obteve {status}: {body:?}");
}
