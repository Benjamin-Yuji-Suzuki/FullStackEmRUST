use axum::{
    extract::{Form, Path, State},
    response::Redirect,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::*;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateSystemForm {
    pub name: String,
    pub description: Option<String>,
    pub defuzz_method: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/systems", get(list_systems).post(create_system))
        .route("/systems/{id}", get(get_system).put(update_system).delete(delete_system))
        .route("/sys/create", post(create_system_form))
        .route("/sys/{id}/delete", post(delete_system_form))
}

/// Routes outside /api nest (no state needed for GET form)
pub fn form_routes() -> Router<()> {
    Router::new()
        .route("/novo-sistema", get(create_form_page))
}

async fn list_systems(
    State(state): State<AppState>,
) -> Result<Json<Vec<FuzzySystem>>, AppError> {
    let systems = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(systems))
}

async fn create_system(
    State(state): State<AppState>,
    Json(req): Json<CreateSystemRequest>,
) -> Result<(axum::http::StatusCode, Json<FuzzySystem>), AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Validation("O nome do sistema é obrigatório".into()));
    }
    if name.len() > 255 {
        return Err(AppError::Validation("O nome deve ter no máximo 255 caracteres".into()));
    }

    let defuzz = req.defuzz_method.unwrap_or_else(|| "centroid".into());
    let valid_methods = ["centroid", "bisector", "mom", "lom", "som"];
    if !valid_methods.contains(&defuzz.as_str()) {
        return Err(AppError::Validation(format!(
            "Método de defuzzificação inválido: {defuzz}"
        )));
    }

    let system = sqlx::query_as::<_, FuzzySystem>(
        "INSERT INTO fuzzy_systems (name, description, defuzz_method) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&name)
    .bind(&req.description)
    .bind(&defuzz)
    .fetch_one(&state.pool)
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(system)))
}

async fn get_system(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FuzzySystem>, AppError> {
    let system = sqlx::query_as::<_, FuzzySystem>(
        "SELECT * FROM fuzzy_systems WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    Ok(Json(system))
}

async fn update_system(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSystemRequest>,
) -> Result<Json<FuzzySystem>, AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Validation("O nome do sistema é obrigatório".into()));
    }
    if name.len() > 255 {
        return Err(AppError::Validation("O nome deve ter no máximo 255 caracteres".into()));
    }

    let system = sqlx::query_as::<_, FuzzySystem>(
        "UPDATE fuzzy_systems SET name = $1, description = $2, defuzz_method = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(&name)
    .bind(&req.description)
    .bind(req.defuzz_method.unwrap_or_else(|| "centroid".into()))
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sistema não encontrado".into()))?;

    Ok(Json(system))
}

async fn delete_system(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM fuzzy_systems WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Sistema não encontrado".into()));
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub async fn create_form_page() -> axum::response::Html<&'static str> {
    axum::response::Html(r#"<!DOCTYPE html>
<html lang="pt-BR"><head><meta charset="utf-8"><title>Novo Sistema</title>
<style>
body{background:#12120F;color:#F0EDE4;font-family:monospace;padding:40px;max-width:500px;margin:auto}
label{font-size:10px;color:#605A4A;display:block;margin-bottom:4px;margin-top:14px;text-transform:uppercase;letter-spacing:1px}
input,select{width:100%;padding:8px 10px;background:#222219;border:1px solid rgba(239,159,39,0.35);border-radius:5px;color:#F0EDE4;font-family:monospace;font-size:12px;box-sizing:border-box}
.btn{padding:8px 14px;border-radius:5px;cursor:pointer;font-family:monospace;font-size:11px;text-decoration:none;display:inline-block}
.btn-primary{background:#EF9F27;color:#1A1200;border:1px solid #EF9F27;font-weight:700}
.btn-cancel{background:transparent;color:#A09880;border:1px solid rgba(239,159,39,0.35)}
h2{font-family:sans-serif;color:#F0EDE4}
</style></head><body>
<h2>Novo Sistema Fuzzy</h2>
<form action="/api/sys/create" method="post">
<label>Nome *</label><input type="text" name="name" required/>
<label>Descrição</label><input type="text" name="description"/>
<label>Método</label>
<select name="defuzz_method">
<option value="centroid">Centroide</option>
<option value="bisector">Bissetor</option>
<option value="mom">Mean of Maximum</option>
<option value="lom">Largest of Maximum</option>
<option value="som">Smallest of Maximum</option>
</select>
<div style="margin-top:20px;display:flex;gap:10px">
<a class="btn btn-cancel" href="/">Cancelar</a>
<button type="submit" class="btn btn-primary">Criar Sistema</button>
</div>
</form>
</body></html>"#)
}

async fn create_system_form(
    State(state): State<AppState>,
    Form(req): Form<CreateSystemForm>,
) -> Result<Redirect, AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Validation("Nome obrigatório".into()));
    }
    let defuzz = req.defuzz_method.unwrap_or_else(|| "centroid".into());

    sqlx::query(
        "INSERT INTO fuzzy_systems (name, description, defuzz_method) VALUES ($1, $2, $3)"
    )
    .bind(&name)
    .bind(&req.description)
    .bind(&defuzz)
    .execute(&state.pool)
    .await?;

    Ok(Redirect::to("/"))
}

pub async fn add_var_page() -> axum::response::Html<&'static str> {
    axum::response::Html(r#"<!DOCTYPE html>
<html lang="pt-BR"><head><meta charset="utf-8"><title>Adicionar Variável</title>
<style>body{background:#12120F;color:#F0EDE4;font-family:monospace;padding:40px;max-width:500px;margin:auto}
label{font-size:10px;color:#605A4A;display:block;margin-bottom:4px;margin-top:14px;text-transform:uppercase;letter-spacing:1px}
input,select{width:100%;padding:8px 10px;background:#222219;border:1px solid rgba(239,159,39,0.35);border-radius:5px;color:#F0EDE4;font-family:monospace;font-size:12px;box-sizing:border-box}
.btn{padding:8px 14px;border-radius:5px;cursor:pointer;font-family:monospace;font-size:11px;text-decoration:none;display:inline-block}
.btn-primary{background:#EF9F27;color:#1A1200;border:1px solid #EF9F27;font-weight:700}
.btn-cancel{background:transparent;color:#A09880;border:1px solid rgba(239,159,39,0.35)}h2{font-family:sans-serif}
</style></head><body>
<h2>Adicionar Variável</h2>
<form id="f" onsubmit="return false;">
<label>Sistema</label><select id="s-sys" class="text-input"></select>
<label>Nome</label><input id="s-name" class="text-input" required/>
<label>Papel</label><select id="s-role" class="text-input"><option value="antecedent">Antecedente</option><option value="consequent">Consequente</option></select>
<label>Universo min</label><input id="s-min" class="text-input" value="0" step="0.1"/>
<label>Universo max</label><input id="s-max" class="text-input" value="100" step="0.1"/>
<div style="margin-top:20px;display:flex;gap:10px">
<a class="btn btn-cancel" href="/vars" target="_self">Cancelar</a>
<button class="btn btn-primary" onclick="addVar()">Adicionar</button></div></form>
<script>
async function loadSystems(){var r=await fetch('/api/systems');var d=await r.json();var sel=document.getElementById('s-sys');
d.forEach(function(s){var o=document.createElement('option');o.value=s.id;o.text=s.name;sel.appendChild(o);});
var p=new URLSearchParams(location.search).get('s');if(p)sel.value=p;}
async function addVar(){var sid=document.getElementById('s-sys').value;var body=JSON.stringify({name:document.getElementById('s-name').value,role:document.getElementById('s-role').value,universe_min:parseFloat(document.getElementById('s-min').value),universe_max:parseFloat(document.getElementById('s-max').value)});
var r=await fetch('/api/systems/'+sid+'/variables',{method:'POST',headers:{'Content-Type':'application/json'},body:body});
if(r.ok)location.href='/vars?s='+sid;else alert('Erro: '+r.status);}
loadSystems();
</script></body></html>"#)
}

pub async fn add_term_page() -> axum::response::Html<&'static str> {
    axum::response::Html(r#"<!DOCTYPE html>
<html lang="pt-BR"><head><meta charset="utf-8"><title>Adicionar Termo</title>
<style>body{background:#12120F;color:#F0EDE4;font-family:monospace;padding:40px;max-width:500px;margin:auto}
label{font-size:10px;color:#605A4A;display:block;margin-bottom:4px;margin-top:14px;text-transform:uppercase;letter-spacing:1px}
input,select{width:100%;padding:8px 10px;background:#222219;border:1px solid rgba(239,159,39,0.35);border-radius:5px;color:#F0EDE4;font-family:monospace;font-size:12px;box-sizing:border-box}
.btn{padding:8px 14px;border-radius:5px;cursor:pointer;font-family:monospace;font-size:11px;text-decoration:none;display:inline-block}
.btn-primary{background:#EF9F27;color:#1A1200;border:1px solid #EF9F27;font-weight:700}
.btn-cancel{background:transparent;color:#A09880;border:1px solid rgba(239,159,39,0.35)}h2{font-family:sans-serif}
</style></head><body>
<h2>Adicionar Termo Linguístico</h2>
<form id="f" onsubmit="return false;">
<label>Sistema</label><select id="t-sys" class="text-input" onchange="loadVars()"></select>
<label>Variável</label><select id="t-var" class="text-input"></select>
<label>Rótulo</label><input id="t-label" class="text-input" required/>
<label>Tipo MF</label><select id="t-mf" class="text-input"><option value="trimf">trimf [a,b,c]</option><option value="trapmf">trapmf [a,b,c,d]</option><option value="gaussmf">gaussmf [mean,sigma]</option></select>
<label>Parâmetros</label><input id="t-params" class="text-input" placeholder="ex: 0,10,22"/>
<div style="margin-top:20px;display:flex;gap:10px">
<a class="btn btn-cancel" href="/vars" target="_self">Cancelar</a>
<button class="btn btn-primary" onclick="addTerm()">Adicionar</button></div></form>
<script>
async function loadSystems(){var r=await fetch('/api/systems');var d=await r.json();var sel=document.getElementById('t-sys');
d.forEach(function(s){var o=document.createElement('option');o.value=s.id;o.text=s.name;sel.appendChild(o);});}
async function loadVars(){var sid=document.getElementById('t-sys').value;if(!sid)return;
var r=await fetch('/api/systems/'+sid+'/variables');var d=await r.json();
var sel=document.getElementById('t-var');sel.innerHTML='';
d.forEach(function(v){var o=document.createElement('option');o.value=v.id;o.text=v.name;sel.appendChild(o);});}
async function addTerm(){var params=document.getElementById('t-params').value.split(',').map(function(x){return parseFloat(x.trim())});
var body=JSON.stringify({label:document.getElementById('t-label').value,mf_type:document.getElementById('t-mf').value,params:params});
var vid=document.getElementById('t-var').value;
var r=await fetch('/api/variables/'+vid+'/terms',{method:'POST',headers:{'Content-Type':'application/json'},body:body});
if(r.ok)location.href='/vars?s='+document.getElementById('t-sys').value;else{alert('Erro: '+r.status);console.log(r);}}
loadSystems();
</script></body></html>"#)
}

async fn delete_system_form(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Redirect, AppError> {
    let result = sqlx::query("DELETE FROM fuzzy_systems WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Sistema não encontrado".into()));
    }

    Ok(Redirect::to("/"))
}
