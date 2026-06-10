use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use sqlx::Row;
use crate::engine;
use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/batch", post(process_batch))
        .route("/batch/{system_id}", get(list_batch_results))
        .route("/batch/result/{id}", delete(delete_batch_result))
        .route("/batch/parse-file", post(parse_file))
}

#[derive(Deserialize)]
pub struct BatchRequest {
    pub system_id: Uuid,
    pub inputs: Vec<std::collections::HashMap<String, serde_json::Value>>,
}

/// Mapeia nomes de colunas do dataset_ml.parquet para variáveis do sistema fuzzy
/// e converte attack_vector_primary (string) para gravidade_ataque (numérico).
fn prepare_batch_input(row: &std::collections::HashMap<String, serde_json::Value>) -> std::collections::HashMap<String, f64> {
    let mut out = std::collections::HashMap::new();

    // Mapeamento attack_vector_primary → gravidade_ataque
    let attack_map: std::collections::HashMap<&str, f64> = [
        ("phishing", 20.0),
        ("malware", 40.0),
        ("trojan", 40.0),
        ("dos", 50.0),
        ("ddos", 50.0),
        ("insider", 60.0),
        ("data_breach", 70.0),
        ("apt", 80.0),
        ("ransomware", 85.0),
    ].into_iter().collect();

    for (k, v) in row {
        let mapped_key = match k.as_str() {
            "company_revenue_usd" => "receita_anual_usd",
            "employee_count" => "total_funcionarios",
            "attack_vector_primary" => "gravidade_ataque",
            _ => k.as_str(),
        };

        // Se for attack_vector_primary, tenta mapear string → número
        if k == "attack_vector_primary" {
            if let serde_json::Value::String(s) = v {
                let n = attack_map.get(s.as_str()).copied().unwrap_or(30.0);
                out.insert("gravidade_ataque".to_string(), n);
            }
            continue;
        }

        // Pula colunas que não são inputs do fuzzy
        if matches!(k.as_str(), "total_loss_usd" | "incident_date" | "discovery_date" | "country_hq" | "industry_primary" | "is_public_company" | "systems_affected" | "data_type" | "confidence_tier" | "quality_score" | "quality_grade" | "attributed_group" | "attribution_confidence" | "incident_date_estimated" | "data_source_primary" | "data_source_secondary" | "data_source_type" | "attack_chain") {
            continue;
        }

        // Converte numérico normalmente
        let num = match v {
            serde_json::Value::Number(n) => n.as_f64(),
            _ => None,
        };
        if let Some(n) = num {
            out.insert(mapped_key.to_string(), n);
        }
    }

    out
}

async fn load_engine_data(
    pool: &sqlx::PgPool,
    system_id: Uuid,
) -> Result<(Vec<engine::VarInfo>, Vec<engine::RuleInfo>), AppError> {
    let variables = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE system_id = $1 ORDER BY name"
    )
    .bind(system_id)
    .fetch_all(pool)
    .await?;

    let all_terms: Vec<FuzzyTerm> = sqlx::query_as(
        "SELECT ft.* FROM fuzzy_terms ft \
         JOIN fuzzy_variables fv ON fv.id = ft.variable_id \
         WHERE fv.system_id = $1 \
         ORDER BY fv.name, ft.label"
    )
    .bind(system_id)
    .fetch_all(pool)
    .await?;

    let rules = sqlx::query_as::<_, FuzzyRule>(
        "SELECT * FROM fuzzy_rules WHERE system_id = $1 ORDER BY position"
    )
    .bind(system_id)
    .fetch_all(pool)
    .await?;

    let var_infos: Vec<engine::VarInfo> = variables.iter().map(|v| {
        let terms: Vec<engine::TermInfo> = all_terms.iter()
            .filter(|t| t.variable_id == v.id)
            .map(|t| {
                let params: Vec<f64> = t.params.as_array()
                    .map(|a| a.iter().filter_map(|x| x.as_f64()).collect())
                    .unwrap_or_default();
                engine::TermInfo {
                    term_id: t.id,
                    label: t.label.clone(),
                    mf_type: t.mf_type.clone(),
                    params,
                }
            })
            .collect();
        engine::VarInfo {
            var_id: v.id,
            name: v.name.clone(),
            role: v.role.clone(),
            universe_min: v.universe_min,
            universe_max: v.universe_max,
            resolution: v.resolution as usize,
            terms,
        }
    }).collect();

    let rule_infos: Vec<engine::RuleInfo> = rules.iter().map(|r| engine::RuleInfo {
        rule_text: r.rule_text.clone(),
        weight: r.weight,
    }).collect();

    Ok((var_infos, rule_infos))
}

async fn process_batch(
    State(state): State<AppState>,
    Json(req): Json<BatchRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(req.system_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    let (var_infos, rule_infos) = load_engine_data(&state.pool, req.system_id).await?;

    if req.inputs.is_empty() {
        return Err(AppError::Validation("Nenhum input fornecido".into()));
    }

    let mut results = Vec::with_capacity(req.inputs.len());

    for (i, input_row) in req.inputs.iter().enumerate() {
        let engine_input = prepare_batch_input(input_row);
        let fuzzy_output = engine::evaluate_mamdani(&var_infos, &rule_infos, &engine_input);

        let target = input_row.get("total_loss_usd")
            .and_then(|v| v.as_f64())
            .map(|v| (v / 1_000_000.0).clamp(0.0, 100.0))
            .or_else(|| {
                if fuzzy_output.is_empty() { return None; }
                let sum = fuzzy_output.values().copied().sum::<f64>();
                let avg = sum / fuzzy_output.len() as f64;
                if avg.is_finite() { Some(avg) } else { None }
            })
            .unwrap_or(50.0);

        let fuzzy_avg = if !fuzzy_output.is_empty() {
            fuzzy_output.values().copied().sum::<f64>() / fuzzy_output.len() as f64
        } else { f64::NAN };

        let outputs_json = serde_json::to_value(&fuzzy_output).unwrap_or_else(|_| json!({}));
        let mapped_inputs = serde_json::to_value(&engine_input).unwrap_or_else(|_| json!({}));

        results.push((i as i32, mapped_inputs, target, fuzzy_avg, outputs_json));
    }

    // Bulk INSERT único em vez de 778 inserts individuais
    let now = chrono::Utc::now();
    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO batch_results (system_id, source_file, row_index, inputs, output, executed_at) "
    );
    query_builder.push_values(&results, |mut b, (idx, inputs, output, _favg, _odet)| {
        b.push_bind(req.system_id)
         .push_bind("batch-api")
         .push_bind(idx)
         .push_bind(inputs)
         .push_bind(output)
         .push_bind(now);
    });
    query_builder.push(" RETURNING id, row_index, inputs, output, executed_at");
    let inserted = query_builder
        .build()
        .fetch_all(&state.pool)
        .await?;

    let response_results: Vec<serde_json::Value> = inserted.into_iter().map(|row: sqlx::postgres::PgRow| {
        let id: Uuid = row.get("id");
        let row_idx: i32 = row.get("row_index");
        let inputs: serde_json::Value = row.get("inputs");
        let output: f64 = row.get("output");
        let executed_at: chrono::DateTime<chrono::Utc> = row.get("executed_at");
        let idx = row_idx as usize;
        let (_, _, _, fuzzy_avg, outputs_json) = &results[idx];
        json!({
            "id": id,
            "row_index": row_idx,
            "inputs": inputs,
            "output": output,
            "fuzzy_output": fuzzy_avg,
            "outputs_detail": outputs_json,
            "executed_at": executed_at,
        })
    }).collect();

    Ok(Json(json!({
        "system_id": req.system_id,
        "system_name": system.name,
        "total": req.inputs.len(),
        "processed": response_results.len(),
        "errors": 0,
        "results": response_results,
    })))
}

async fn list_batch_results(
    State(state): State<AppState>,
    Path(system_id): Path<Uuid>,
) -> Result<Json<Vec<BatchResult>>, AppError> {
    let rows = sqlx::query_as::<_, BatchResult>(
        "SELECT * FROM batch_results WHERE system_id = $1 ORDER BY row_index ASC"
    )
    .bind(system_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows))
}

async fn delete_batch_result(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM batch_results WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Resultado batch não encontrado".into()));
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct ParseFileRequest {
    pub filename: Option<String>,
    pub data: String,
}

async fn parse_file(
    Json(req): Json<ParseFileRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let data = base64_decode(&req.data).map_err(|e| AppError::Validation(format!("Base64 invalido: {e}")))?;
    let ext = req.filename.as_deref().unwrap_or("dados.parquet").to_lowercase();

    let info = if ext.ends_with(".csv") {
        let json_str = String::from_utf8(data).map_err(|e| AppError::Validation(format!("UTF-8 invalido: {e}")))?;
        parse_csv_to_json(&json_str)?
    } else {
        parse_parquet_bytes_to_json(&data)?
    };

    let col_names: Vec<&str> = info.rows.first()
        .and_then(|r| r.as_object())
        .map(|o| o.keys().map(|k| k.as_str()).collect())
        .unwrap_or_default();

    Ok(Json(json!({
        "columns": col_names,
        "rows": info.rows,
        "total": info.total,
    })))
}

struct ParsedData {
    rows: Vec<serde_json::Value>,
    total: usize,
}

fn parse_csv_to_json(text: &str) -> Result<ParsedData, AppError> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let headers: Vec<String> = lines.next()
        .ok_or_else(|| AppError::Validation("CSV vazio".into()))?
        .split(',')
        .map(|h| h.trim().trim_matches('"').to_string())
        .collect();

    let mut rows = Vec::new();
    for line in lines {
        let vals = split_csv_line(line);
        let mut obj = serde_json::Map::new();
        for (i, h) in headers.iter().enumerate() {
            let raw = vals.get(i).map(|s| s.as_str()).unwrap_or("");
            let val = parse_number(raw).unwrap_or(serde_json::Value::String(raw.to_string()));
            obj.insert(h.clone(), val);
        }
        if !obj.is_empty() {
            rows.push(serde_json::Value::Object(obj));
        }
    }

    let total = rows.len();
    Ok(ParsedData { rows, total })
}

fn split_csv_line(line: &str) -> Vec<String> {
    let mut vals = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    for ch in line.chars() {
        match ch {
            '"' => in_q = !in_q,
            ',' if !in_q => {
                vals.push(std::mem::take(&mut cur));
            }
            c => cur.push(c),
        }
    }
    vals.push(cur);
    vals
}

fn parse_number(s: &str) -> Option<serde_json::Value> {
    let s = s.trim().trim_matches('"');
    if s.is_empty() { return None; }
    if s.contains('.') || s.contains(',') {
        s.replace(',', ".").parse::<f64>().ok().map(serde_json::Value::from)
    } else {
        s.parse::<i64>().ok().map(serde_json::Value::from)
            .or_else(|| s.parse::<f64>().ok().map(serde_json::Value::from))
    }
}

fn parse_parquet_bytes_to_json(data: &[u8]) -> Result<ParsedData, AppError> {
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    use bytes::Bytes;

    let owned = Bytes::copy_from_slice(data);
    let builder = ParquetRecordBatchReaderBuilder::try_new(owned)
        .map_err(|e| AppError::Validation(format!("Erro ao ler parquet: {e}")))?;
    let reader = builder.build()
        .map_err(|e| AppError::Validation(format!("Erro ao abrir parquet: {e}")))?;

    let mut rows = Vec::new();
    for batch_result in reader {
        let batch = batch_result
            .map_err(|e| AppError::Validation(format!("Erro ao ler batch: {e}")))?;
        let schema = batch.schema();
        let col_names: Vec<&str> = schema.fields().iter().map(|f| f.name().as_str()).collect();

        for row_idx in 0..batch.num_rows() {
            let mut obj = serde_json::Map::new();
            for (col_idx, col_name) in col_names.iter().enumerate() {
                let array = batch.column(col_idx);
                let val = if array.is_null(row_idx) {
                    serde_json::Value::Null
                } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::Float64Array>() {
                    serde_json::Value::from(a.value(row_idx))
                } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::Int64Array>() {
                    serde_json::Value::from(a.value(row_idx))
                } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::StringArray>() {
                    serde_json::Value::String(a.value(row_idx).to_string())
                } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::LargeStringArray>() {
                    serde_json::Value::String(a.value(row_idx).to_string())
                } else if let Some(a) = array.as_any().downcast_ref::<arrow::array::BooleanArray>() {
                    serde_json::Value::Bool(a.value(row_idx))
                } else {
                    serde_json::Value::String(format!("{:?}", array))
                };
                obj.insert(col_name.to_string(), val);
            }
            rows.push(serde_json::Value::Object(obj));
        }
    }

    let total = rows.len();
    Ok(ParsedData { rows, total })
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    let clean: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    base64::engine::general_purpose::STANDARD.decode(&clean)
        .map_err(|e| format!("Base64 invalido: {e}"))
}
