use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

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

fn row_to_f64_map(row: &std::collections::HashMap<String, serde_json::Value>) -> std::collections::HashMap<String, f64> {
    row.iter().filter_map(|(k, v)| {
        let num = match v {
            serde_json::Value::Number(n) => n.as_f64(),
            serde_json::Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        };
        num.map(|n| (k.clone(), n))
    }).collect()
}

async fn load_engine_data(
    pool: &sqlx::PgPool,
    system_id: Uuid,
) -> Result<(Vec<engine::VarInfo>, Vec<engine::RuleInfo>), AppError> {
    let variables = sqlx::query_as::<_, FuzzyVariable>(
        "SELECT * FROM fuzzy_variables WHERE system_id = $1"
    )
    .bind(system_id)
    .fetch_all(pool)
    .await?;

    let all_terms: Vec<FuzzyTerm> = sqlx::query_as(
        "SELECT ft.* FROM fuzzy_terms ft \
         JOIN fuzzy_variables fv ON fv.id = ft.variable_id \
         WHERE fv.system_id = $1"
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

    let mut results = Vec::new();
    let mut errors = 0;

    for (i, input_row) in req.inputs.iter().enumerate() {
        let numeric_row = row_to_f64_map(input_row);
        let engine_input: &std::collections::HashMap<String, f64> = &numeric_row;
        match engine::evaluate_mamdani(&var_infos, &rule_infos, engine_input) {
            outputs if !outputs.is_empty() => {
                let output_val = outputs.values().copied().sum::<f64>() / outputs.len() as f64;
                let outputs_json = serde_json::to_value(&outputs).unwrap_or_else(|_| json!({}));

                let record = sqlx::query_as::<_, BatchResult>(
                    "INSERT INTO batch_results (system_id, source_file, row_index, inputs, output) \
                     VALUES ($1, 'batch-api', $2, $3::jsonb, $4) RETURNING *"
                )
                .bind(req.system_id)
                .bind(i as i32)
                .bind(json!(input_row))
                .bind(output_val)
                .fetch_one(&state.pool)
                .await?;

                results.push(json!({
                    "id": record.id,
                    "row_index": record.row_index,
                    "inputs": record.inputs,
                    "output": record.output,
                    "outputs_detail": outputs_json,
                    "executed_at": record.executed_at,
                }));
            }
            _ => {
                errors += 1;
            }
        }
    }

    Ok(Json(json!({
        "system_id": req.system_id,
        "system_name": system.name,
        "total": req.inputs.len(),
        "processed": results.len(),
        "errors": errors,
        "results": results,
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
