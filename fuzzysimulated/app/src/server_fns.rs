use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── Tipos ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub defuzz_method: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub id: String,
    pub system_id: String,
    pub name: String,
    pub role: String,
    pub universe_min: f64,
    pub universe_max: f64,
    pub resolution: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermInfo {
    pub id: String,
    pub variable_id: String,
    pub label: String,
    pub mf_type: String,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleInfo {
    pub id: String,
    pub system_id: String,
    pub rule_text: String,
    pub weight: f64,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationInfo {
    pub id: String,
    pub system_id: String,
    pub inputs: Value,
    pub outputs: Value,
    pub weather_data: Option<Value>,
    pub city: Option<String>,
    pub executed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub system_id: String,
    pub action_type: String,
    pub entity_type: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    pub events: Vec<AuditEvent>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResponse {
    pub events: Vec<AuditEvent>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub city: String,
    pub temp: f64,
    pub humidity: f64,
}

// ── Helper ──

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        async fn api_get<T: serde::de::DeserializeOwned>(url: &str) -> Option<T> {
            gloo_net::http::Request::get(url).send().await.ok()?.json().await.ok()
        }
        async fn api_post<T: serde::de::DeserializeOwned, B: serde::Serialize>(url: &str, body: &B) -> Option<T> {
            gloo_net::http::Request::post(url)
                .json(body).ok()?
                .send().await.ok()?
                .json().await.ok()
        }
        async fn api_put<T: serde::de::DeserializeOwned, B: serde::Serialize>(url: &str, body: &B) -> Option<T> {
            gloo_net::http::Request::put(url)
                .json(body).ok()?
                .send().await.ok()?
                .json().await.ok()
        }
        async fn api_delete(url: &str) -> bool {
            gloo_net::http::Request::delete(url).send().await.ok().is_some()
        }
    } else {
        async fn api_get<T: serde::de::DeserializeOwned>(url: &str) -> Option<T> {
            reqwest::get(url).await.ok()?.json().await.ok()
        }
        async fn api_post<T: serde::de::DeserializeOwned, B: serde::Serialize>(url: &str, body: &B) -> Option<T> {
            reqwest::Client::new()
                .post(url).json(body).send().await.ok()?
                .json().await.ok()
        }
        async fn api_put<T: serde::de::DeserializeOwned, B: serde::Serialize>(url: &str, body: &B) -> Option<T> {
            reqwest::Client::new()
                .put(url).json(body).send().await.ok()?
                .json().await.ok()
        }
        async fn api_delete(url: &str) -> bool {
            reqwest::Client::new().delete(url).send().await.ok().is_some()
        }
    }
}

// ── Systems ──

pub async fn list_systems() -> Vec<SystemInfo> {
    api_get("/api/systems").await.unwrap_or_default()
}

pub async fn get_system(id: &str) -> Option<SystemInfo> {
    api_get(&format!("/api/systems/{id}")).await
}

pub async fn create_system(name: &str, description: Option<&str>, defuzz_method: &str) -> Option<SystemInfo> {
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "defuzz_method": defuzz_method,
    });
    api_post("/api/systems", &body).await
}

// ── Variables ──

pub async fn list_variables(system_id: &str) -> Vec<serde_json::Value> {
    api_get(&format!("/api/systems/{system_id}/variables")).await.unwrap_or_default()
}

pub async fn create_variable(system_id: &str, name: &str, role: &str, min: f64, max: f64) -> Option<VariableInfo> {
    let body = serde_json::json!({
        "name": name, "role": role,
        "universe_min": min, "universe_max": max,
        "resolution": 501
    });
    api_post(&format!("/api/systems/{system_id}/variables"), &body).await
}

pub async fn get_variable(id: &str) -> Option<VariableInfo> {
    api_get(&format!("/api/variables/{id}")).await
}

pub async fn get_term(id: &str) -> Option<TermInfo> {
    api_get(&format!("/api/terms/{id}")).await
}

pub async fn get_rule(id: &str) -> Option<RuleInfo> {
    api_get(&format!("/api/rules/{id}")).await
}

pub async fn delete_variable(id: &str) -> bool {
    api_delete(&format!("/api/variables/{id}")).await
}

pub async fn create_term(variable_id: &str, label: &str, mf_type: &str, params: Vec<f64>) -> Option<TermInfo> {
    let body = serde_json::json!({ "label": label, "mf_type": mf_type, "params": params });
    api_post(&format!("/api/variables/{variable_id}/terms"), &body).await
}

pub async fn delete_term(id: &str) -> bool {
    api_delete(&format!("/api/terms/{id}")).await
}

// ── Rules ──

pub async fn list_rules(system_id: &str) -> Vec<RuleInfo> {
    // Fetch from export endpoint
    let data: Option<serde_json::Value> = api_get(&format!("/api/systems/{system_id}/export")).await;
    match data {
        Some(d) => d["rules"].as_array()
            .map(|arr| arr.iter().map(|r| RuleInfo {
                id: r["id"].as_str().unwrap_or("").to_string(),
                system_id: system_id.to_string(),
                rule_text: r["rule_text"].as_str().unwrap_or("").to_string(),
                weight: r["weight"].as_f64().unwrap_or(1.0),
                position: r["position"].as_i64().unwrap_or(0) as i32,
            }).collect())
            .unwrap_or_default(),
        None => vec![],
    }
}

pub async fn create_rule(system_id: &str, rule_text: &str, weight: f64) -> Option<RuleInfo> {
    let body = serde_json::json!({ "rule_text": rule_text, "weight": weight });
    api_post(&format!("/api/systems/{system_id}/rules"), &body).await
}

pub async fn delete_rule(id: &str) -> bool {
    api_delete(&format!("/api/rules/{id}")).await
}

// ── Simulations ──

pub async fn list_simulations(system_id: &str) -> Vec<SimulationInfo> {
    api_get(&format!("/api/systems/{system_id}/simulations")).await.unwrap_or_default()
}

pub async fn run_simulation(system_id: &str, inputs: &serde_json::Value) -> Option<serde_json::Value> {
    api_post(&format!("/api/systems/{system_id}/simulate"), &serde_json::json!({ "inputs": inputs })).await
}

pub async fn get_weather(city: &str) -> Result<WeatherData, String> {
    let url = format!("/api/weather?city={city}");
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let resp = gloo_net::http::Request::get(&url).send().await.map_err(|e| format!("Erro de rede: {e}"))?;
            if !resp.ok() {
                let body: serde_json::Value = resp.json().await.unwrap_or_default();
                let msg = body["error"].as_str().unwrap_or("Erro desconhecido");
                return Err(msg.to_string());
            }
            resp.json().await.map_err(|e| format!("Erro ao decodificar resposta: {e}"))
        } else {
            let resp = reqwest::get(&url).await.map_err(|e| format!("Erro de rede: {e}"))?;
            if !resp.status().is_success() {
                let body: serde_json::Value = resp.json().await.unwrap_or_default();
                let msg = body["error"].as_str().unwrap_or("Erro desconhecido");
                return Err(msg.to_string());
            }
            resp.json().await.map_err(|e| format!("Erro ao decodificar resposta: {e}"))
        }
    }
}

// ── Audit ──

pub async fn list_audit_events(system_id: String) -> AuditSummary {
    let val: Option<serde_json::Value> = api_get(&format!("/api/systems/{system_id}/audit")).await;
    match val {
        Some(v) => {
            let events: Vec<AuditEvent> = serde_json::from_value(v["events"].clone()).unwrap_or_default();
            let total = events.len();
            AuditSummary { events, total }
        }
        None => AuditSummary { events: vec![], total: 0 },
    }
}

// ── Delete system ──

pub async fn update_system(id: &str, name: &str, description: Option<&str>, defuzz_method: &str) -> Option<SystemInfo> {
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "defuzz_method": defuzz_method,
    });
    api_put(&format!("/api/systems/{id}"), &body).await
}

pub async fn delete_system(id: &str) -> bool {
    api_delete(&format!("/api/systems/{id}")).await
}

pub async fn update_variable(id: &str, name: &str, role: &str, universe_min: f64, universe_max: f64, resolution: i32) -> Option<VariableInfo> {
    let body = serde_json::json!({
        "name": name,
        "role": role,
        "universe_min": universe_min,
        "universe_max": universe_max,
        "resolution": resolution,
    });
    api_put(&format!("/api/variables/{id}"), &body).await
}

pub async fn update_term(id: &str, label: &str, mf_type: &str, params: Vec<f64>) -> Option<TermInfo> {
    let body = serde_json::json!({
        "label": label,
        "mf_type": mf_type,
        "params": params,
    });
    api_put(&format!("/api/terms/{id}"), &body).await
}

pub async fn update_rule(id: &str, rule_text: &str, weight: f64) -> Option<RuleInfo> {
    let body = serde_json::json!({
        "rule_text": rule_text,
        "weight": weight,
    });
    api_put(&format!("/api/rules/{id}"), &body).await
}
