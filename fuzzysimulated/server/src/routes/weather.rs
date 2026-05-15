use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::errors::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CityQuery {
    pub city: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/weather", get(get_weather))
}

async fn get_weather(
    State(state): State<AppState>,
    Query(query): Query<CityQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let city = query.city.ok_or_else(|| AppError::Validation("Parâmetro 'city' é obrigatório".into()))?;

    if city.trim().is_empty() {
        return Err(AppError::Validation("Informe o nome de uma cidade".into()));
    }

    let api_key = std::env::var("OPENWEATHER_API_KEY")
        .map_err(|_| AppError::External("OPENWEATHER_API_KEY não configurada".into()))?;

    if api_key.is_empty() {
        return Err(AppError::External("OPENWEATHER_API_KEY está vazia. Configure no .env".into()));
    }

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        urlencoding(&city), api_key
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| AppError::External(format!("Erro ao criar cliente HTTP: {e}")))?;

    let resp = client.get(&url).send().await
        .map_err(|e| AppError::External(format!("Falha ao conectar com OpenWeather: {e}")))?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(AppError::NotFound(format!("Cidade '{}' não encontrada", city)));
    }

    if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(AppError::External("Chave da API OpenWeather inválida".into()));
    }

    let body: serde_json::Value = resp.json().await
        .map_err(|e| AppError::External(format!("Erro ao ler resposta: {e}")))?;

    let temp = body["main"]["temp"].as_f64().ok_or_else(|| {
        AppError::External("Resposta inesperada da API".into())
    })?;

    let humidity = body["main"]["humidity"].as_f64().ok_or_else(|| {
        AppError::External("Resposta inesperada da API".into())
    })?;

    Ok(Json(json!({
        "city": city,
        "temp": temp,
        "humidity": humidity,
        "description": body["weather"][0]["description"].as_str().unwrap_or(""),
    })))
}

fn urlencoding(s: &str) -> String {
    s.replace(' ', "%20")
}
