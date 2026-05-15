pub mod server_fns;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use server_fns::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn spawn_async(f: impl std::future::Future<Output = ()> + 'static) {
            wasm_bindgen_futures::spawn_local(f);
        }
    } else {
        fn spawn_async(f: impl std::future::Future<Output = ()> + Send + 'static) {
            leptos::task::spawn(f);
        }
    }
}

// ─────────────────────────────────────────────────────────────
// Shell (SSR entry point)
// ─────────────────────────────────────────────────────────────
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="pt-BR">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

// ─────────────────────────────────────────────────────────────
// App raiz + roteamento
// ─────────────────────────────────────────────────────────────
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/fuzzysimulated.css"/>
        <Title text="FuzzySimulated — Inference Platform"/>

        <Router>
            <div class="shell">
                <Sidebar/>
                <div class="main">
                    <Routes fallback=|| view! { <NotFound/> }>
                        <Route path=StaticSegment("")       view=Dashboard/>
                        <Route path=StaticSegment("sys/create-form") view=CreateSystemForm/>

                        <Route path=StaticSegment("vars")   view=Variaveis/>
                        <Route path=StaticSegment("rules")  view=Regras/>
                        <Route path=StaticSegment("sim")    view=Simulador/>
                        <Route path=StaticSegment("hist")   view=Historico/>
                        <Route path=StaticSegment("batch")  view=BatchDashboard/>
                        <Route path=StaticSegment("analysis") view=Analise/>
                        <Route path=StaticSegment("audit")  view=Auditoria/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

// ─────────────────────────────────────────────────────────────
// Sidebar
// ─────────────────────────────────────────────────────────────
#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <div class="sidebar-logo">
                <div class="logo-mark">"⬡ FuzzySimulated"</div>
                <div class="logo-sub">"Inference Platform"</div>
            </div>

            <nav class="nav-section">
                <div class="nav-label">"Visão Geral"</div>
                <a class="nav-item" href="/">
                    <i class="ti ti-layout-dashboard nav-icon"></i>
                    "Dashboard"
                </a>
            </nav>

            <nav class="nav-section">
                <div class="nav-label">"Construção"</div>
                <a class="nav-item" href="/vars">
                    <i class="ti ti-vector-triangle nav-icon"></i>
                    "Variáveis & Termos"
                </a>
                <a class="nav-item" href="/rules">
                    <i class="ti ti-list-check nav-icon"></i>
                    "Editor de Regras"
                </a>
            </nav>

            <nav class="nav-section">
                <div class="nav-label">"Execução"</div>
                <a class="nav-item" href="/sim">
                    <i class="ti ti-player-play nav-icon"></i>
                    "Simulador"
                </a>
                <a class="nav-item" href="/hist">
                    <i class="ti ti-history nav-icon"></i>
                    "Histórico"
                </a>
                <a class="nav-item" href="/batch">
                    <i class="ti ti-database nav-icon"></i>
                    "Batch"
                </a>
            </nav>

            <nav class="nav-section">
                <div class="nav-label">"Análise"</div>
                <a class="nav-item" href="/analysis">
                    <i class="ti ti-chart-grid-dots nav-icon"></i>
                    "Superfície & Matriz"
                </a>
                <a class="nav-item" href="/audit">
                    <i class="ti ti-history-toggle nav-icon"></i>
                    "Auditoria"
                </a>
            </nav>

            <div class="sidebar-footer">
                <span class="sprint-badge">"⬡ Sprint 2 — CRUDs + API"</span>
                <div class="sidebar-course">"Disciplina QPS · CESUPA"</div>
            </div>
        </aside>
    }
}

// ─────────────────────────────────────────────────────────────
// Topbar
// ─────────────────────────────────────────────────────────────
#[component]
fn Topbar(breadcrumb: &'static str) -> impl IntoView {
    view! {
        <div class="topbar">
            <span class="topbar-breadcrumb">{breadcrumb}</span>
            <div class="topbar-right">
                <button class="btn">
                    <i class="ti ti-file-description"></i>
                    "Docs"
                </button>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Dashboard — SSR + WASM via sinal simples
// ⚠️ SSR não usa Resource (não é Send no WASM)
// ─────────────────────────────────────────────────────────────
#[component]
fn Dashboard() -> impl IntoView {
    let systems = RwSignal::new(Vec::<SystemInfo>::new());
    let loaded = RwSignal::new(false);

    spawn_async({
        let s = systems.clone();
        let l = loaded.clone();
        async move { let data = list_systems().await; s.set(data); l.set(true); }
    });

    view! {
        <Topbar breadcrumb="Dashboard"/>
        <div class="content">

            <div class="section-header">
                <div class="section-title">"Sistemas Fuzzy"</div>
                <div style="display:flex;gap:8px">
                    <a class="btn btn-primary" href="/novo-sistema" target="_self"
                        style="font-size:10px;padding:5px 12px;text-decoration:none">
                        <i class="ti ti-plus"></i>"Criar Sistema"
                    </a>
                </div>
            </div>

            <div class="systems-grid">
                {move || {
                    if !loaded.get() {
                        return view! { <div class="loading">"Carregando..."</div> }.into_any();
                    }
                    let list = systems.get();
                    if list.is_empty() {
                        view! { <div class="empty-state">"Nenhum sistema cadastrado. Crie o primeiro!"</div> }.into_any()
                    } else {
                        view! {
                            <For each=move || list.clone() key=|s| s.id.clone() let:sys>
                                {
                                    let sid = sys.id.clone();
                                    view! {
                                        <div class="system-card">
                                            <div class="system-card-top">
                                                <div>
                                                    <div class="system-name">{sys.name.clone()}</div>
                                                    <div class="system-desc">{sys.description.clone().unwrap_or_default()}</div>
                                                </div>
                                                <span class="tag tag-green">"Ativo"</span>
                                            </div>
                                            <div style="font-size:10px;color:var(--text3)">
                                                "Defuzz: " <span style="color:var(--amber)">{sys.defuzz_method.clone()}</span>
                                                " · Criado: " <span>{sys.created_at[..10].to_string()}</span>
                                            </div>
                                            <div class="system-meta">
                                                <span>"ID: " {sys.id[..8].to_string()}"..."</span>
                                                <div class="system-actions">
                                                    <a class="icon-btn" href={format!("/audit?id={}", sid)}>
                                                        <i class="ti ti-history"></i>
                                                    </a>
                                                    <form action={format!("/api/sys/{sid}/delete")} method="post" target="_self" style="display:inline">
                                                        <button type="submit" class="icon-btn">
                                                            <i class="ti ti-trash"></i>
                                                        </button>
                                                    </form>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            </For>
                        }.into_any()
                    }
                }}
            </div>

        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Auditoria — funcional (dados reais do banco)
// ─────────────────────────────────────────────────────────────
#[component]
fn Auditoria() -> impl IntoView {
    let systems_list = LocalResource::new(|| async move { list_systems().await });
    let selected_id = RwSignal::new(String::new());

    let selected_id_clone = selected_id;
    let events = LocalResource::new(move || {
        let id = selected_id_clone.get();
        async move {
            if id.is_empty() {
                AuditSummary { events: vec![], total: 0 }
            } else {
                list_audit_events(id).await
            }
        }
    });

    view! {
        <Topbar breadcrumb="Auditoria"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px">
                <div class="section-title">"Histórico de Alterações (UC16)"</div>
            </div>

            <div class="panel" style="margin-bottom:16px">
                <div class="panel-title">"Selecione um Sistema"</div>
                {move || {
                    let list = systems_list.get();
                    view! {
                        <select class="text-input" style="margin-top:8px"
                            prop:value=move || selected_id.get()
                            on:change=move |e| selected_id.set(event_target_value(&e))>
                            <option value="">"— Selecione —"</option>
                            {move || list.clone().unwrap_or_default().into_iter().map(|s| view! {
                                <option value={s.id.clone()}>{s.name.clone()}</option>
                            }).collect_view()}
                        </select>
                    }
                }}
            </div>

            <Suspense fallback=|| view! { <div class="loading">"Carregando..."</div> }>
            {move || {
                let id = selected_id.get();
                if id.is_empty() {
                    return view! { <div class="empty-state">"Selecione um sistema para ver o histórico."</div> }.into_any();
                }

                match events.get() {
                    None => view! { <div class="loading">"Carregando..."</div> }.into_any(),
                    Some(summary) => {
                        if summary.events.is_empty() {
                            view! { <div class="empty-state">"Nenhuma alteração registrada para este sistema."</div> }.into_any()
                        } else {
                            let total = summary.total;
                            let events = summary.events;
                            view! {
                                <div style="font-size:11px;color:var(--text3);margin-bottom:12px">
                                    {total}" evento(s) registrado(s)"
                                </div>
                                <div class="timeline">
                                    <For each=move || events.clone() key=|e| e.id.clone() let:evt>
                                        <div class="timeline-item">
                                            <div class="timeline-dot" data-action=evt.action_type.clone()></div>
                                            <div class="timeline-content">
                                                <div class="timeline-header">
                                                    <span class="tag tag-amber">{evt.action_type.clone()}</span>
                                                    <span class="tag tag-teal">{evt.entity_type.clone()}</span>
                                                    <span style="font-size:10px;color:var(--text3);margin-left:auto">
                                                        {evt.created_at[..19].replace("T", " ")}
                                                    </span>
                                                </div>
                                                <div class="timeline-desc">{evt.description.clone()}</div>
                                            </div>
                                        </div>
                                    </For>
                                </div>
                            }.into_any()
                        }
                    }
                }
            }}
            </Suspense>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Create System (form page)
// ─────────────────────────────────────────────────────────────
#[component]
fn CreateSystemForm() -> impl IntoView {
    view! {
        <Topbar breadcrumb="Novo Sistema"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:20px">
                <div class="section-title">"Novo Sistema Fuzzy"</div>
            </div>

            <div class="panel" style="max-width:500px">
                <form id="create-form" action="/api/sys/create" method="post" target="_self">
                    <label class="input-label">"Nome *"</label>
                    <input type="text" name="name" class="text-input" placeholder="Ex: Conforto Térmico" required/>

                    <label class="input-label">"Descrição"</label>
                    <input type="text" name="description" class="text-input" placeholder="Opcional"/>

                    <label class="input-label">"Método de Defuzzificação"</label>
                    <select name="defuzz_method" class="text-input">
                        <option value="centroid">"Centroide"</option>
                        <option value="bisector">"Bissetor"</option>
                        <option value="mom">"Mean of Maximum"</option>
                        <option value="lom">"Largest of Maximum"</option>
                        <option value="som">"Smallest of Maximum"</option>
                    </select>

                    <div style="display:flex;gap:10px;margin-top:16px">
                        <a class="btn" href="/">"Cancelar"</a>
                        <button type="submit" class="btn btn-primary">"Criar Sistema"</button>
                    </div>
                </form>
                <script>
                    {r#"document.getElementById('create-form').addEventListener('submit', async function(e) {
                        e.preventDefault();
                        var data = new FormData(this);
                        var body = JSON.stringify({
                            name: data.get('name'),
                            description: data.get('description') || null,
                            defuzz_method: data.get('defuzz_method') || 'centroid'
                        });
                        await fetch('/api/systems', {
                            method: 'POST',
                            headers: {'Content-Type': 'application/json'},
                            body: body
                        });
                        window.location.href = '/';
                    });"#}
                </script>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Variáveis & Termos (UC02) — CRUD
// ─────────────────────────────────────────────────────────────
#[component]
fn Variaveis() -> impl IntoView {
    let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
    let selected_sys = RwSignal::new(String::new());
    let variables = RwSignal::new(Vec::<serde_json::Value>::new());

    // Load systems + auto-select from URL param
    {
        let sl = systems_list.clone();
        let ss = selected_sys.clone();
        let v = variables.clone();
        spawn_async(async move {
            let systems = list_systems().await;
            leptos::logging::log!("[Variaveis] loaded {} systems", systems.len());
            sl.set(systems);

            #[cfg(target_arch = "wasm32")]
            {
                let search = web_sys::window()
                    .and_then(|w| w.location().search().ok())
                    .unwrap_or_default();
                leptos::logging::log!("[Variaveis] URL search: {}", &search);
                if let Some(id) = search.split("s=").nth(1).and_then(|s| s.split('&').next()) {
                    if !id.is_empty() {
                        leptos::logging::log!("[Variaveis] auto-selecting system: {}", id);
                        ss.set(id.to_string());
                        let vars = list_variables(id).await;
                        leptos::logging::log!("[Variaveis] loaded {} variables", vars.len());
                        v.set(vars);
                    }
                }
            }
        });
    }

    view! {
        <Topbar breadcrumb="Variáveis & Termos"/>
        <div class="content">
            <div class="section-header"><div class="section-title">"Variáveis & Termos (UC02)"</div></div>
            <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:400px">
                <select class="text-input" style="margin-bottom:0"
                    prop:value=move || selected_sys.get()
                    on:change=move |e| {
                        selected_sys.set(event_target_value(&e));
                        let v = variables.clone();
                        let s = event_target_value(&e);
                        spawn_async(async move { v.set(list_variables(&s).await); });
                    }>
                    <option value="">"— Sistema —"</option>
                    {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                </select>
            </div>
            {move || {
                let sid = selected_sys.get();
                if sid.is_empty() { return view! { <div class="empty-state">"Selecione um sistema"</div> }.into_any(); }
                let vars = variables.get();
                view! {
                    <div class="var-layout">
                        <div class="var-sidebar">
                            <div class="var-group-label">"Variáveis"</div>
                            {if vars.is_empty() {
                                view! { <div style="font-size:10px;color:var(--text3);padding:8px">"Nenhuma variável ainda."</div> }.into_any()
                            } else {
                                vars.iter().map(|v| {
                                    let name = v["name"].as_str().unwrap_or("?").to_string();
                                    let role = v["role"].as_str().unwrap_or("").to_string();
                                    let dot = if role=="antecedent"{"var(--amber)"}else{"var(--teal)"};
                                    view! { <div class="var-item"><span class="var-dot" style=format!("background:{dot}")></span>{name}</div> }
                                }).collect_view().into_any()
                            }}
                        </div>
                        <div class="var-panel">
                            <div style="display:flex;gap:12px;align-items:center;flex-wrap:wrap;margin-bottom:12px">
                                <a class="btn btn-primary" style="font-size:10px;padding:4px 10px" href={format!("/add-var?s={}", sid)} target="_self">
                                    <i class="ti ti-plus"></i>"Variável"
                                </a>
                                <a class="btn" style="font-size:10px;padding:4px 10px" href={format!("/add-term?s={}", sid)} target="_self">
                                    <i class="ti ti-plus"></i>"Termo"
                                </a>
                            </div>
                            <div class="section-title" style="font-size:11px">"Termos"</div>
                            <div class="term-chips">
                                {vars.first().and_then(|v| v["terms"].as_array().map(|terms| {
                                    terms.iter().map(|t| {
                                        let label = t["label"].as_str().unwrap_or("?").to_string();
                                        let mf = t["mf_type"].as_str().unwrap_or("").to_string();
                                        view! { <div class="term-chip active">{label}" ["{mf}"]"</div> }
                                    }).collect_view()
                                })).unwrap_or_default()}
                            </div>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

 fn Regras() -> impl IntoView {
     let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
     let selected_sys = RwSignal::new(String::new());
     let rules = RwSignal::new(Vec::<serde_json::Value>::new());

    spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

    view! {
        <Topbar breadcrumb="Editor de Regras"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px"><div class="section-title">"Editor de Regras (UC03)"</div></div>
            <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:400px">
                <label class="input-label">"Sistema"</label>
                <select class="text-input" style="margin-bottom:0"
                    on:change=move |e| {
                        let sid = event_target_value(&e);
                        selected_sys.set(sid.clone());
                        let r = rules.clone();
                        spawn_async(async move { r.set(serde_json::to_value(list_rules(&sid).await).unwrap_or_default().as_array().cloned().unwrap_or_default()); });
                    }>
                     <option value="">"— Selecione —"</option>
                     {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                 </select>
             </div>
             {move || {
                 let rs = rules.get();
                 if rs.is_empty() { return view! { <div class="empty-state">"Selecione um sistema para ver as regras."</div> }.into_any(); }
                 view! {
                     <div class="rule-hint">"Formato: SE &lt;variável&gt; É &lt;termo&gt; E ... ENTÃO &lt;variável&gt; É &lt;termo&gt; [peso]"</div>
                     {rs.iter().map(|r| {
                         let text = r["rule_text"].as_str().unwrap_or("").to_string();
                         view! { <div class="rule-row"><div class="rule-num">{r["position"].as_i64().unwrap_or(0)}</div><div class="rule-text">"SE " {text}</div><div class="rule-weight">"w=" {r["weight"].as_f64().unwrap_or(1.0)}</div></div> }
                     }).collect_view()}
                 }.into_any()
             }}
         </div>
     }
 }

 #[component]
 fn Simulador() -> impl IntoView {
     let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
     let selected_sys = RwSignal::new(String::new());

    spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

    view! {
        <Topbar breadcrumb="Simulador"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px"><div class="section-title">"Simulador (UC04)"</div></div>
             <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:400px">
                 <label class="input-label">"Sistema"</label>
                 <select class="text-input" style="margin-bottom:0"
                     on:change=move |e| selected_sys.set(event_target_value(&e))>
                     <option value="">"— Selecione —"</option>
                     {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                 </select>
             </div>
             <div class="sim-layout">
                 <div class="panel"><div class="panel-title">"Entradas"</div><div style="color:var(--text3);font-size:11px;padding:16px 0">"Configuração de inputs em breve."</div></div>
                 <div class="panel"><div class="panel-title">"Resultado"</div><div style="color:var(--text3);font-size:11px;padding:16px 0">"Execute uma simulação para ver o resultado."</div></div>
             </div>
         </div>
     }
 }

 #[component]
 fn Historico() -> impl IntoView {
     let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
     let selected_sys = RwSignal::new(String::new());
     let sims = RwSignal::new(Vec::<serde_json::Value>::new());

    spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

    view! {
        <Topbar breadcrumb="Histórico"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px"><div class="section-title">"Histórico (UC06)"</div></div>
            <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:400px">
                <label class="input-label">"Sistema"</label>
                <select class="text-input" style="margin-bottom:0"
                    on:change=move |e| {
                        let sid = event_target_value(&e);
                        selected_sys.set(sid.clone());
                        let s = sims.clone();
                        spawn_async(async move { s.set(list_simulations(&sid).await.into_iter().map(|si| serde_json::json!(si)).collect()); });
                    }>
                     <option value="">"— Selecione —"</option>
                     {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                 </select>
             </div>
             {move || {
                 let list = sims.get();
                 if list.is_empty() { return view! { <div class="empty-state">"Nenhuma simulação encontrada."</div> }.into_any(); }
                 view! {
                     <div class="hist-wrap">
                         <table class="hist-table"><thead><tr><th>"Entradas"</th><th>"Saída"</th><th>"Data"</th></tr></thead>
                         <tbody>{list.iter().map(|s| {
                             view! { <tr><td style="font-size:10px">{s["inputs"].to_string()}</td><td>{s["outputs"].to_string()}</td><td>{s["executed_at"].as_str().unwrap_or("")[..19].replace("T"," ")}</td></tr> }
                         }).collect_view()}</tbody></table>
                     </div>
                 }.into_any()
             }}
         </div>
     }
 }

 #[component]
 fn BatchDashboard() -> impl IntoView {
     view! { <Topbar breadcrumb="Inferência em Lote"/><div class="content"><div class="empty-state">"Batch — em construção"</div></div> }
 }

 #[component]
 fn Analise() -> impl IntoView {
     view! { <Topbar breadcrumb="Análise"/><div class="content"><div class="empty-state">"Análise — em construção"</div></div> }
 }

// ── 404 ──
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <div class="not-found-code">"404"</div>
            <div>"Página não encontrada"</div>
            <a class="btn" href="/">"← Voltar ao Dashboard"</a>
        </div>
    }
}
