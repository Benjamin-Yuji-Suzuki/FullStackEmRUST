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
    State(_state): State<AppState>,
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
    let mut encoded = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => encoded.push(byte as char),
            b' ' => encoded.push_str("%20"),
            _ => encoded.push_str(&format!("%{:02X}", byte)),
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::urlencoding;

    #[test]
    fn test_urlencoding_ascii() {
        assert_eq!(urlencoding("hello"), "hello");
    }

    #[test]
    fn test_urlencoding_with_spaces() {
        assert_eq!(urlencoding("São Paulo"), "S%C3%A3o%20Paulo");
    }

    #[test]
    fn test_urlencoding_special_chars() {
        assert_eq!(urlencoding("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn test_urlencoding_empty() {
        assert_eq!(urlencoding(""), "");
    }
}
