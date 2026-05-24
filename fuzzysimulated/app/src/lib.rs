pub mod server_fns;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_query_map;
use leptos_router::StaticSegment;
use server_fns::*;
use serde_json::Value;
use cfg_if::cfg_if;

fn status_color(status: &str) -> &'static str {
    match status {
        "favorito" => "tag-amber",
        "concluido" => "tag-teal",
        "desativado" => "tag-gray",
        _ => "tag-green",
    }
}

fn status_icon(status: &str) -> &'static str {
    match status {
        "favorito" => "ti ti-star",
        "concluido" => "ti ti-circle-check",
        "desativado" => "ti ti-circle-minus",
        _ => "ti ti-circle-check",
    }
}

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
                        <Route path=StaticSegment("")        view=Dashboard/>
                        <Route path=StaticSegment("newsys") view=CreateSystemForm/>
                        <Route path=StaticSegment("editsys") view=EditSystemPage/>
                        <Route path=StaticSegment("add-var") view=AddVarPage/>
                        <Route path=StaticSegment("edit-var") view=EditVarPage/>
                        <Route path=StaticSegment("add-term") view=AddTermPage/>
                        <Route path=StaticSegment("edit-term") view=EditTermPage/>
                        <Route path=StaticSegment("add-rule") view=AddRulePage/>
                        <Route path=StaticSegment("edit-rule") view=EditRulePage/>
                        <Route path=StaticSegment("vars")   view=Variaveis/>
                        <Route path=StaticSegment("rules")  view=Regras/>
                        <Route path=StaticSegment("sim")    view=Simulador/>
                        <Route path=StaticSegment("hist")   view=Historico/>
                        <Route path=StaticSegment("batch")  view=BatchDashboard/>
                        <Route path=StaticSegment("analysis") view=Analise/>
                        <Route path=StaticSegment("audit")  view=Auditoria/>
                        <Route path=StaticSegment("opt")   view=OptimizePage/>
                        <Route path=StaticSegment("import") view=ImportPage/>
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
                <a class="nav-item" href="/opt">
                    <i class="ti ti-math-function nav-icon"></i>
                    "Otimizador"
                </a>
            </nav>

            <div class="sidebar-footer">
                <span class="sprint-badge">"⬡ Sprint 3 — Entrega Final"</span>
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
                <div style="display:flex;gap:6px">
                    <a class="btn btn-primary" href="/newsys" target="_self"
                        style="font-size:10px;padding:5px 12px;text-decoration:none">
                        <i class="ti ti-plus"></i>"Criar Sistema"
                    </a>
                    <a class="btn btn-outline" href="/import" target="_self"
                        style="font-size:10px;padding:5px 12px;text-decoration:none">
                        <i class="ti ti-upload"></i>"Importar"
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
                                    let st = sys.status.clone();
                                    let is_fav = st == "favorito";
                                    let sys_for_status = sys.clone();
                                    let sl2 = systems.clone();
                                    view! {
                                        <div class="system-card">
                                            <div class="system-card-top">
                                                <div>
                                                    <div class="system-name">{sys.name.clone()}</div>
                                                    <div class="system-desc">{sys.description.clone().unwrap_or_default()}</div>
                                                </div>
                                                <span class=format!("tag {}", status_color(&st))>
                                                    <i class=status_icon(&st)></i>
                                                    " " {st.clone()}
                                                </span>
                                            </div>
                                            <div style="font-size:10px;color:var(--text3)">
                                                "Defuzz: " <span style="color:var(--amber)">{sys.defuzz_method.clone()}</span>
                                                " · Criado: " <span>{sys.created_at[..10].to_string()}</span>
                                            </div>
                                            <div class="system-meta" style="margin-top:8px">
                                                <select class="text-input" style="font-size:9px;padding:2px 4px;width:auto"
                                                    prop:value=move || sys_for_status.status.clone()
                                                    on:change={
                                                        let sid_clone = sid.clone();
                                                        let sl3 = sl2.clone();
                                                        move |e| {
                                                            let new_st = event_target_value(&e);
                                                            let id2 = sid_clone.clone();
                                                            let s2 = sl3.clone();
                                                            spawn_async(async move {
                                                                update_system_status(&id2, &new_st).await;
                                                                s2.set(list_systems().await);
                                                            });
                                                        }
                                                    }>
                                                    <option value="ativo">"Ativo"</option>
                                                    <option value="favorito">"Favorito"</option>
                                                    <option value="concluido">"Concluído"</option>
                                                    <option value="desativado">"Desativado"</option>
                                                </select>
                                                <div class="system-actions">
                                                    <a class="icon-btn" href={format!("/editsys?id={}", sid)} target="_self" title="Editar">
                                                        <i class="ti ti-edit"></i>
                                                    </a>
                                                    <a class="icon-btn" href={format!("/audit?id={}", sid)} title="Auditoria">
                                                        <i class="ti ti-history"></i>
                                                    </a>
                                                    <button class="icon-btn" title="Duplicar"
                                                        on:click={let sid2 = sid.clone(); let sl3 = systems.clone();
                                                            move |_| {
                                                                let id = sid2.clone();
                                                                let ss = sl3.clone();
                                                                spawn_async(async move {
                                                                    duplicate_system(&id, "").await;
                                                                    ss.set(list_systems().await);
                                                                });
                                                            }}>
                                                        <i class="ti ti-copy"></i>
                                                    </button>
                                                    <a class="icon-btn" href={format!("/api/systems/{}/export", sid)} target="_self" title="Exportar JSON" download>
                                                        <i class="ti ti-download"></i>
                                                    </a>
                                                    {if is_fav {
                                                        view! { <span class="icon-btn" style="opacity:0.4;cursor:not-allowed" title="Remova o favorito para deletar"><i class="ti ti-lock"></i></span> }.into_any()
                                                    } else {
                                                        view! {
                                                            <form action={format!("/api/sys/{sid}/delete")} method="post" target="_self" style="display:inline">
                                                                <button type="submit" class="icon-btn" title="Deletar">
                                                                    <i class="ti ti-trash"></i>
                                                                </button>
                                                            </form>
                                                        }.into_any()
                                                    }}
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
    let trigger = RwSignal::new(0u32);
    let events = LocalResource::new(move || {
        let id = selected_id_clone.get();
        let _ = trigger.get();
        async move {
            if id.is_empty() {
                AuditSummary { events: vec![], total: 0 }
            } else {
                list_audit_events(id).await
            }
        }
    });

    fn undo_event(event_id: String, trigger: RwSignal<u32>) {
        spawn_async(async move {
            if let Err(e) = undo_audit_event(&event_id).await {
                leptos::logging::error!("Erro ao desfazer: {}", e);
            }
            trigger.update(|v| *v += 1);
        });
    }

    let show_orphans = RwSignal::new(false);
    let orphan_trigger = RwSignal::new(0u32);
    let orphan_events = LocalResource::new(move || {
        let _ = show_orphans.get();
        let _ = orphan_trigger.get();
        async move { list_orphan_audit_events().await }
    });

    view! {
        <Topbar breadcrumb="Auditoria"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px">
                <div class="section-title">"Histórico de Alterações (UC16)"</div>
            </div>

            <div class="panel" style="margin-bottom:16px">
                <div class="panel-title">"Sistemas"</div>
                <div style="display:flex;gap:8px;margin-top:8px;align-items:center">
                    <select class="text-input" style="flex:1"
                        prop:value=move || selected_id.get()
                        on:change=move |e| { selected_id.set(event_target_value(&e)); show_orphans.set(false); }>
                        <option value="">"— Selecione um sistema ativo —"</option>
                        {move || systems_list.get().unwrap_or_default().into_iter().map(|s| view! {
                            <option value={s.id.clone()}>{s.name.clone()}</option>
                        }).collect_view()}
                    </select>
                    <button class="btn-sm btn-outline"
                        on:click=move |_| { show_orphans.update(|v| *v = !*v); orphan_trigger.update(|v| *v += 1); }>
                        <i class="ti ti-trash"></i> " Deletados"
                    </button>
                </div>
            </div>

            {move || if show_orphans.get() {
                view! {
                    <Suspense fallback=|| view! { <div class="loading">"Carregando..."</div> }>
                    {move || match orphan_events.get() {
                        None => view! { <div class="loading">"Carregando..."</div> }.into_any(),
                        Some(summary) => {
                            if summary.events.is_empty() {
                                view! { <div class="empty-state">"Nenhum sistema deletado encontrado."</div> }.into_any()
                            } else {
                                view! {
                                    <div class="panel">
                                        <div class="panel-title" style="color:var(--red)">"Sistemas Deletados"</div>
                                        <div class="timeline">
                                            <For each=move || summary.events.clone() key=|e| e.id.clone() let:evt>
                                                <div class="timeline-item">
                                                    <div class="timeline-dot" data-action="delete"></div>
                                                    <div class="timeline-content">
                                                        <div class="timeline-header">
                                                            <span class="tag tag-red">{evt.action_type.clone()}</span>
                                                            <span class="tag tag-teal">{evt.entity_type.clone()}</span>
                                                            <span style="font-size:10px;color:var(--text3);margin-left:auto">
                                                                {evt.created_at[..19].replace("T", " ")}
                                                            </span>
                                                        </div>
                                                        <div class="timeline-desc">{evt.description.clone()}</div>
                                                        {{
                                                            let eid = evt.id.clone();
                                                            let at = evt.action_type.clone();
                                                            if at.starts_with("undo") {
                                                                view! { <span class="tag" style="margin-top:6px;opacity:0.6">"Restaurado"</span> }.into_any()
                                                            } else {
                                                                view! { <button class="btn-sm btn-outline" style="margin-top:6px;border-color:var(--red);color:var(--red)"
                                                                    on:click=move |_| { undo_event(eid.clone(), orphan_trigger); show_orphans.set(false); }>
                                                                    <i class="ti ti-arrow-back-up"></i> " Restaurar Sistema"
                                                                </button> }.into_any()
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                            </For>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                    </Suspense>
                }.into_any()
            } else if selected_id.get().is_empty() {
                view! { <div class="empty-state">"Selecione um sistema ou clique em 'Deletados'."</div> }.into_any()
            } else {
                view! {
                    <Suspense fallback=|| view! { <div class="loading">"Carregando..."</div> }>
                    {move || match events.get() {
                        None => view! { <div class="loading">"Carregando..."</div> }.into_any(),
                        Some(summary) => {
                            if summary.events.is_empty() {
                                view! { <div class="empty-state">"Nenhuma alteração registrada para este sistema."</div> }.into_any()
                            } else {
                                let total = summary.total;
                                let evts = summary.events;
                                view! {
                                    <div style="font-size:11px;color:var(--text3);margin-bottom:12px">
                                        {total}" evento(s) registrado(s)"
                                    </div>
                                    <div class="timeline">
                                        <For each=move || evts.clone() key=|e| e.id.clone() let:evt>
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
                                                    {{
                                                        let at = evt.action_type.clone();
                                                        let deid = evt.id.clone();
                                                        if at.starts_with("undo") {
                                                            view! { <span class="tag" style="margin-top:6px;opacity:0.6">"Desfeito"</span> }.into_any()
                                                        } else {
                                                            view! { <button class="btn-sm btn-outline" style="margin-top:6px"
                                                                on:click=move |_| undo_event(deid.clone(), trigger)>
                                                                <i class="ti ti-arrow-back-up"></i> " Desfazer"
                                                            </button> }.into_any()
                                                        }
                                                    }}
                                                </div>
                                            </div>
                                        </For>
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                    </Suspense>
                }.into_any()
            }}
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Optimize Page (UC21-UC25)
// ─────────────────────────────────────────────────────────────
#[component]
fn OptimizePage() -> impl IntoView {
    let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
    let selected_sys = RwSignal::new(String::new());
    let coef_a = RwSignal::new("1.0".to_string());
    let coef_b = RwSignal::new("0.0".to_string());
    let coef_c = RwSignal::new("1.0".to_string());
    let coef_d = RwSignal::new("0.0".to_string());
    let coef_e = RwSignal::new("0.0".to_string());
    let coef_f = RwSignal::new("0.0".to_string());
    let x_min = RwSignal::new("-10".to_string());
    let x_max = RwSignal::new("10".to_string());
    let y_min = RwSignal::new("-10".to_string());
    let y_max = RwSignal::new("10".to_string());
    let result = RwSignal::new(None::<OptimizationResult>);
    let loading = RwSignal::new(false);
    let error_msg = RwSignal::new(String::new());
    // UC24: optimization history
    let opt_history = RwSignal::new(Vec::<Value>::new());

    // UC17: PSO state
    let pso_target_in = RwSignal::new("[{\"Temperatura\": 80}]".to_string());
    let pso_target_out = RwSignal::new("[{\"Risco\": 0.8}]".to_string());
    let pso_pop = RwSignal::new("20".to_string());
    let pso_iters = RwSignal::new("50".to_string());
    let pso_result = RwSignal::new(None::<Value>);
    let pso_loading = RwSignal::new(false);

    spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

    // load history when system changes (UC24)
    let load_history = {
        let ss = selected_sys.clone();
        let oh = opt_history.clone();
        move || {
            let sid = ss.get();
            if !sid.is_empty() {
                let oh2 = oh.clone();
                spawn_async(async move {
                    oh2.set(list_optimizations(&sid).await);
                });
            } else {
                oh.set(Vec::new());
            }
        }
    };

    let calculate = move || {
        let parse_or = |s: &str, _name: &str| -> Option<f64> {
            let v: f64 = s.trim().parse().ok()?;
            Some(v)
        };

        let a = match parse_or(&coef_a.get(), "a") { Some(v) => v, None => { error_msg.set("Coeficiente 'a' inválido".into()); return; } };
        let b = match parse_or(&coef_b.get(), "b") { Some(v) => v, None => { error_msg.set("Coeficiente 'b' inválido".into()); return; } };
        let c = match parse_or(&coef_c.get(), "c") { Some(v) => v, None => { error_msg.set("Coeficiente 'c' inválido".into()); return; } };
        let d = match parse_or(&coef_d.get(), "d") { Some(v) => v, None => { error_msg.set("Coeficiente 'd' inválido".into()); return; } };
        let e = match parse_or(&coef_e.get(), "e") { Some(v) => v, None => { error_msg.set("Coeficiente 'e' inválido".into()); return; } };
        let f = match parse_or(&coef_f.get(), "f") { Some(v) => v, None => { error_msg.set("Coeficiente 'f' inválido".into()); return; } };
        let xmn = match parse_or(&x_min.get(), "x_min") { Some(v) => v, None => { error_msg.set("x_min inválido".into()); return; } };
        let xmx = match parse_or(&x_max.get(), "x_max") { Some(v) => v, None => { error_msg.set("x_max inválido".into()); return; } };
        let ymn = match parse_or(&y_min.get(), "y_min") { Some(v) => v, None => { error_msg.set("y_min inválido".into()); return; } };
        let ymx = match parse_or(&y_max.get(), "y_max") { Some(v) => v, None => { error_msg.set("y_max inválido".into()); return; } };

        if xmn >= xmx { error_msg.set("x_min deve ser menor que x_max".into()); return; }
        if ymn >= ymx { error_msg.set("y_min deve ser menor que y_max".into()); return; }

        loading.set(true);
        error_msg.set(String::new());
        let sys_id = selected_sys.get();
        let sys_opt_owned = if sys_id.is_empty() { None } else { Some(sys_id) };
        let r2 = result.clone();
        let ld = loading.clone();
        let em = error_msg.clone();
        spawn_async(async move {
            let res = optimize_function(
                sys_opt_owned.as_deref(), a, b, c, d, e, f, xmn, xmx, ymn, ymx
            ).await;
            match res {
                Some(val) => { r2.set(Some(val)); }
                None => { em.set("Erro ao calcular. Verifique os coeficientes.".into()); }
            }
            ld.set(false);
        });
    };

    view! {
        <Topbar breadcrumb="Otimizador"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px">
                <div class="section-title">"Otimizador de Função Objetivo (UC21–UC25)"</div>
            </div>

            <div class="opt-layout">
                <div class="panel">
                    <div class="panel-title">"Função Objetivo"</div>
                    <div style="font-size:12px;color:var(--text3);margin-bottom:12px;font-family:monospace">
                        "f(x, y) = ax² + bxy + cy² + dx + ey + f"
                    </div>

                    <div class="opt-grid">
                        <div>
                            <label class="input-label">"a"</label>
                            <input type="text" class="text-input" prop:value=move || coef_a.get() on:input=move |e| coef_a.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"b"</label>
                            <input type="text" class="text-input" prop:value=move || coef_b.get() on:input=move |e| coef_b.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"c"</label>
                            <input type="text" class="text-input" prop:value=move || coef_c.get() on:input=move |e| coef_c.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"d"</label>
                            <input type="text" class="text-input" prop:value=move || coef_d.get() on:input=move |e| coef_d.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"e"</label>
                            <input type="text" class="text-input" prop:value=move || coef_e.get() on:input=move |e| coef_e.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"f"</label>
                            <input type="text" class="text-input" prop:value=move || coef_f.get() on:input=move |e| coef_f.set(event_target_value(&e))/>
                        </div>
                    </div>

                    <div class="panel-title" style="margin-top:16px">"Domínio"</div>
                    <div class="opt-grid">
                        <div>
                            <label class="input-label">"x_min"</label>
                            <input type="text" class="text-input" prop:value=move || x_min.get() on:input=move |e| x_min.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"x_max"</label>
                            <input type="text" class="text-input" prop:value=move || x_max.get() on:input=move |e| x_max.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"y_min"</label>
                            <input type="text" class="text-input" prop:value=move || y_min.get() on:input=move |e| y_min.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"y_max"</label>
                            <input type="text" class="text-input" prop:value=move || y_max.get() on:input=move |e| y_max.set(event_target_value(&e))/>
                        </div>
                    </div>

                    <div class="panel" style="margin-top:12px;padding:8px 12px">
                        <label class="input-label">"Sistema (opcional, para auditoria)"</label>
                        <select class="text-input" prop:value=move || selected_sys.get()
                            on:change=move |e| { selected_sys.set(event_target_value(&e)); load_history(); }>
                            <option value="">"— Nenhum —"</option>
                            {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                        </select>
                    </div>

                    {move || { let m = error_msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}

                    <button class="btn btn-primary" style="margin-top:12px" on:click=move |_| calculate()>
                        <i class="ti ti-math-function"></i>"Calcular Ponto Ótimo"
                    </button>
                </div>

                <div class="panel">
                    <div class="panel-title">"Resultado da Otimização"</div>
                    {move || {
                        if loading.get() {
                            return view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Calculando..."</div> }.into_any();
                        }
                        match result.get() {
                            None => view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Preencha os coeficientes e clique em \"Calcular Ponto Ótimo\"."</div> }.into_any(),
                            Some(r) => {
                                let ptype = r.critical_point_type.clone();
                                let type_color = match ptype.as_str() {
                                    "mínimo" => "var(--green)",
                                    "máximo" => "var(--coral)",
                                    "sela" => "var(--amber)",
                                    _ => "var(--text1)",
                                };
                                view! {
                                    <div class="opt-result-grid">
                                        <div class="opt-result-card">
                                            <div class="opt-result-label">"x*"</div>
                                            <div class="opt-result-value">{format!("{:.6}", r.optimal_x)}</div>
                                        </div>
                                        <div class="opt-result-card">
                                            <div class="opt-result-label">"y*"</div>
                                            <div class="opt-result-value">{format!("{:.6}", r.optimal_y)}</div>
                                        </div>
                                        <div class="opt-result-card">
                                            <div class="opt-result-label">"f(x*, y*)"</div>
                                            <div class="opt-result-value">{format!("{:.6}", r.optimal_value)}</div>
                                        </div>
                                        <div class="opt-result-card">
                                            <div class="opt-result-label">"Tipo"</div>
                                            <div class="opt-result-value" style=format!("color:{}", type_color)>{ptype}</div>
                                        </div>
                                    </div>

                                    <div style="margin-top:16px;font-size:11px;color:var(--text3);font-family:monospace;white-space:pre-wrap;background:var(--surface1);padding:12px;border-radius:6px;line-height:1.6">
                                        {r.explanation.clone()}
                                    </div>

                                    <details style="margin-top:12px">
                                        <summary style="cursor:pointer;font-size:11px;color:var(--teal)">"Detalhes Matemáticos"</summary>
                                        <div style="margin-top:8px;font-size:10px;font-family:monospace;background:var(--surface1);padding:12px;border-radius:6px;line-height:1.8">
                                            <div>"Gradiente ∇f no ponto ótimo:"</div>
                                            <div style="padding-left:16px">
                                                "∂f/∂x = " {format!("{:.10}", r.gradient_at_optimum[0])}
                                            </div>
                                            <div style="padding-left:16px">
                                                "∂f/∂y = " {format!("{:.10}", r.gradient_at_optimum[1])}
                                            </div>
                                            <div style="margin-top:8px">"Matriz Hessiana H:"</div>
                                            <div style="padding-left:16px">
                                                "| " {format!("{:.4}", r.hessian_matrix[0][0])} "  " {format!("{:.4}", r.hessian_matrix[0][1])} " |"
                                            </div>
                                            <div style="padding-left:16px">
                                                "| " {format!("{:.4}", r.hessian_matrix[1][0])} "  " {format!("{:.4}", r.hessian_matrix[1][1])} " |"
                                            </div>
                                            <div style="margin-top:8px">
                                                "det(H) = " {format!("{:.4}", r.hessian_matrix[0][0] * r.hessian_matrix[1][1] - r.hessian_matrix[0][1] * r.hessian_matrix[1][0])}
                                            </div>
                                        </div>
                                    </details>

                                    <div style="margin-top:12px;display:flex;gap:8px">
                                        <a class="btn btn-outline" href={format!("/api/optimizations/{}/export", r.id)} target="_self" style="font-size:10px;padding:5px 12px;text-decoration:none">
                                            <i class="ti ti-download"></i>"Exportar Resultado (JSON)"
                                        </a>
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                </div>

                // UC24: Histórico de Otimizações
                <div class="panel">
                    <div class="panel-title">"Histórico de Otimizações (UC24)"</div>
                    {move || {
                        let history = opt_history.get();
                        if history.is_empty() {
                            return view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Selecione um sistema para ver o histórico."</div> }.into_any();
                        }
                        history.iter().map(|opt| {
                            let id = opt["id"].as_str().unwrap_or("");
                            let date = opt["executed_at"].as_str().unwrap_or("");
                            let ptype = opt["critical_point_type"].as_str().unwrap_or("");
                            let x = opt["optimal_x"].as_f64().unwrap_or(0.0);
                            let y = opt["optimal_y"].as_f64().unwrap_or(0.0);
                            let val = opt["optimal_value"].as_f64().unwrap_or(0.0);
                            let type_color = match ptype {
                                "mínimo" => "var(--green)",
                                "máximo" => "var(--coral)",
                                "sela" => "var(--amber)",
                                _ => "var(--text1)",
                            };
                            view! {
                                <div style="display:flex;justify-content:space-between;align-items:center;padding:6px 0;border-bottom:1px solid var(--surface1);font-size:10px">
                                    <div>
                                        <span style="color:var(--text3)">{&date[..19.min(date.len())]}</span>
                                        " | x*=" {format!("{:.4}", x)} " y*=" {format!("{:.4}", y)}
                                        " f=" {format!("{:.4}", val)}
                                        <span style=format!("color:{};margin-left:6px", type_color)>{ptype}</span>
                                    </div>
                                    <a class="icon-btn" href={format!("/api/optimizations/{}/export", id)} target="_self" title="Exportar">
                                        <i class="ti ti-download"></i>
                                    </a>
                                </div>
                            }
                        }).collect_view().into_any()
                    }}
                </div>
                // UC17: PSO - Otimização de Parâmetros MF
                <div class="panel">
                    <div class="panel-title">"Otimização PSO de MF (UC17)"</div>
                    <div style="font-size:11px;color:var(--text3);margin-bottom:8px">
                        "Otimiza parâmetros das funções de pertinência via PSO com dados de referência."
                    </div>
                    <div style="display:flex;gap:8px;flex-wrap:wrap">
                        <div>
                            <label class="input-label">"População"</label>
                            <input type="number" class="text-input" style="width:80px;font-size:10px"
                                prop:value=move || pso_pop.get()
                                on:input=move |e| pso_pop.set(event_target_value(&e))/>
                        </div>
                        <div>
                            <label class="input-label">"Iterações"</label>
                            <input type="number" class="text-input" style="width:80px;font-size:10px"
                                prop:value=move || pso_iters.get()
                                on:input=move |e| pso_iters.set(event_target_value(&e))/>
                        </div>
                        <div style="flex:1;min-width:200px">
                            <label class="input-label">"Target Inputs (JSON)"</label>
                            <textarea class="text-input" style="min-height:50px;font-size:10px;font-family:monospace"
                                prop:value=move || pso_target_in.get()
                                on:input=move |e| pso_target_in.set(event_target_value(&e))/>
                        </div>
                        <div style="flex:1;min-width:200px">
                            <label class="input-label">"Target Outputs (JSON)"</label>
                            <textarea class="text-input" style="min-height:50px;font-size:10px;font-family:monospace"
                                prop:value=move || pso_target_out.get()
                                on:input=move |e| pso_target_out.set(event_target_value(&e))/>
                        </div>
                    </div>
                    <button class="btn btn-primary" style="margin-top:8px" on:click={
                        let ss = selected_sys.clone();
                        let ti = pso_target_in.clone();
                        let to = pso_target_out.clone();
                        let pop = pso_pop.clone();
                        let iters = pso_iters.clone();
                        let pr = pso_result.clone();
                        let pl = pso_loading.clone();
                        move |_| {
                            let sid = ss.get();
                            if sid.is_empty() { return; }
                            let pop_val: usize = pop.get().parse().unwrap_or(20);
                            let iter_val: usize = iters.get().parse().unwrap_or(50);
                            let inputs_val: serde_json::Value = serde_json::from_str(&ti.get()).unwrap_or(serde_json::json!([]));
                            let outputs_val: serde_json::Value = serde_json::from_str(&to.get()).unwrap_or(serde_json::json!([]));
                            pl.set(true);
                            let pr2 = pr.clone();
                            let pl2 = pl.clone();
                            spawn_async(async move {
                                let res = run_pso_optimization(&sid, &inputs_val, &outputs_val, pop_val, iter_val).await;
                                pr2.set(res);
                                pl2.set(false);
                            });
                        }
                    }>
                        <i class="ti ti-math-function"></i>"Executar PSO"
                    </button>
                    {move || {
                        if pso_loading.get() { return view! { <div style="font-size:11px;color:var(--text3);margin-top:8px">"Otimizando..."</div> }.into_any(); }
                        match pso_result.get() {
                            None => view! {}.into_any(),
                            Some(r) => {
                                let best_fit = r["best_fitness"].as_f64().unwrap_or(0.0);
                                let best_pos = r["best_position"].as_array().cloned().unwrap_or_default();
                                view! {
                                    <div style="margin-top:8px;padding:8px;background:var(--surface1);border-radius:4px;font-size:10px">
                                        <div>"Melhor Fitness: " <span style="color:var(--teal);font-weight:600">{format!("{:.6}", best_fit)}</span></div>
                                        <div style="margin-top:4px">{let joined: String = best_pos.iter().map(|p| format!("{:.4} ", p.as_f64().unwrap_or(0.0))).collect(); format!("Parâmetros: [{}]", joined)}</div>
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Create System (form page)
// ─────────────────────────────────────────────────────────────
#[component]
fn CreateSystemForm() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let method = RwSignal::new("centroid".to_string());
    let msg = RwSignal::new(String::new());

    let submit = move || {
        let n = name.get();
        if n.trim().is_empty() { msg.set("Nome obrigatório".into()); return; }
        let d = if desc.get().is_empty() { None } else { Some(desc.get()) };
        let m = method.get();
        let msg2 = msg.clone();
        spawn_async(async move {
            match create_system(&n, d.as_deref(), &m).await {
                Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href("/").ok()); } }
                None => msg2.set("Erro ao criar sistema".into()),
            }
        });
    };

    view! {
        <Topbar breadcrumb="Novo Sistema"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:20px"><div class="section-title">"Novo Sistema Fuzzy"</div></div>
            <div class="panel" style="max-width:500px">
                <label class="input-label">"Nome *"</label>
                <input type="text" class="text-input" placeholder="Ex: Conforto Térmico" prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                <label class="input-label">"Descrição"</label>
                <input type="text" class="text-input" placeholder="Opcional" prop:value=move || desc.get() on:input=move |e| desc.set(event_target_value(&e))/>
                <label class="input-label">"Método de Defuzzificação"</label>
                <select class="text-input" prop:value=move || method.get() on:change=move |e| method.set(event_target_value(&e))>
                    <option value="centroid">"Centroide"</option>
                    <option value="bisector">"Bissetor"</option>
                    <option value="mom">"Mean of Maximum"</option>
                    <option value="lom">"Largest of Maximum"</option>
                    <option value="som">"Smallest of Maximum"</option>
                </select>
                {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| submit()>"Criar Sistema"</button>
                </div>
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
    let selected_var = RwSignal::new(String::new());

    // Load systems + auto-select from URL param
    {
        let sl = systems_list.clone();
        let ss = selected_sys.clone();
        let v = variables.clone();
        let sv = selected_var.clone();
        let _ = (&ss, &v, &sv);
        spawn_async(async move {
            let systems = list_systems().await.into_iter().filter(|s| s.status == "ativo" || s.status == "favorito").collect::<Vec<_>>();
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
                        if let Some(first) = vars.first().and_then(|v| v["id"].as_str()) {
                            sv.set(first.to_string());
                        }
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
                        let sv = selected_var.clone();
                        let s = event_target_value(&e);
                        spawn_async(async move {
                            let vars = list_variables(&s).await;
                            if let Some(first) = vars.first().and_then(|v| v["id"].as_str()) {
                                sv.set(first.to_string());
                            }
                            v.set(vars);
                        });
                    }>
                    <option value="">"— Sistema —"</option>
                    {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                </select>
            </div>
            {move || {
                let sid = selected_sys.get();
                if sid.is_empty() { return view! { <div class="empty-state">"Selecione um sistema"</div> }.into_any(); }
                let vars = variables.get();
                let sel_var_id = selected_var.get();
                view! {
                    <div class="var-layout">
                        <div class="var-sidebar">
                            <div class="var-group-label">"Variáveis"</div>
                            {if vars.is_empty() {
                                view! { <div style="font-size:10px;color:var(--text3);padding:8px">"Nenhuma variável ainda."</div> }.into_any()
                            } else {
                                vars.iter().map(|v| {
                                    let vid = v["id"].as_str().unwrap_or("").to_string();
                                    let name = v["name"].as_str().unwrap_or("?").to_string();
                                    let role = v["role"].as_str().unwrap_or("").to_string();
                                    let dot = if role=="antecedent"{"var(--amber)"}else{"var(--teal)"};
                                    let is_sel = sel_var_id == vid;
                                    let sel_style = if is_sel { "background:var(--surface2);border-left:3px solid var(--blue)" } else { "" };
                                    let vid_del = vid.clone();
                                    let sid_del = sid.clone();
                                    let vars_del = variables.clone();
                                    view! {
                                        <div class="var-item" style=sel_style
                                            on:click=move |_| { selected_var.set(vid.clone()); }>
                                            <span class="var-dot" style=format!("background:{dot}")></span>
                                            <span style="flex:1">{name}</span>
                                            <a class="icon-btn" style="font-size:9px;padding:2px" href={format!("/edit-var?id={}&s={}", vid, sid)}>
                                                <i class="ti ti-edit"></i>
                                            </a>
                                            <button class="icon-btn" style="font-size:9px;padding:2px;color:var(--coral)" 
                                                on:click=move |e| { e.stop_propagation(); let id = vid_del.clone(); spawn_async({ let v = vars_del.clone(); let s = sid_del.clone(); async move { delete_variable(&id).await; v.set(list_variables(&s).await); } }); }>
                                                <i class="ti ti-trash"></i>
                                            </button>
                                        </div>
                                    }
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
                                {vars.iter().find(|v| v["id"].as_str().unwrap_or("") == sel_var_id)
                                    .map(|v| {
                                        let sys_id = v["system_id"].as_str().unwrap_or("").to_string();
                                        v["terms"].as_array().map(|terms| {
                                            terms.iter().map(|t| {
                                                let tid = t["id"].as_str().unwrap_or("").to_string();
                                                let label = t["label"].as_str().unwrap_or("?").to_string();
                                                let mf = t["mf_type"].as_str().unwrap_or("").to_string();
                                                let tid_del = tid.clone();
                                                let sys_id_del = sys_id.clone();
                                                let vars_del = variables.clone();
                                                view! {
                                                    <div class="term-chip active" style="display:inline-flex;align-items:center;gap:4px">
                                                        {label}" ["{mf}"]"
                                                        <a class="icon-btn" style="font-size:7px;padding:1px 3px" href={format!("/edit-term?id={}&s={}", tid, sys_id)}>
                                                            <i class="ti ti-edit"></i>
                                                        </a>
                                                        <button class="icon-btn" style="font-size:7px;padding:1px 3px;color:var(--coral)" 
                                                            on:click=move |e| { e.stop_propagation(); let id = tid_del.clone(); spawn_async({ let v = vars_del.clone(); let s = sys_id_del.clone(); async move { delete_term(&id).await; v.set(list_variables(&s).await); } }); }>
                                                            <i class="ti ti-trash"></i>
                                                        </button>
                                                    </div>
                                                }
                                            }).collect_view()
                                        })
                                    }).flatten().unwrap_or_default()}
                            </div>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

  #[allow(non_snake_case)]
  fn Regras() -> impl IntoView {
      let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
      let selected_sys = RwSignal::new(String::new());
      let rules = RwSignal::new(Vec::<serde_json::Value>::new());

      let load_rules = {
          let ss = selected_sys.clone(); let r = rules.clone();
          move || { let s = ss.get(); if !s.is_empty() { let r2 = r.clone(); spawn_async(async move { r2.set(serde_json::to_value(list_rules(&s).await).unwrap_or_default().as_array().cloned().unwrap_or_default()); }); } }
      };

      spawn_async({
          let sl = systems_list.clone(); let ss = selected_sys.clone(); let r = rules.clone();
          let _ = (&ss, &r);
          async move {
              sl.set(list_systems().await.into_iter().filter(|s| s.status == "ativo" || s.status == "favorito").collect());
              #[cfg(target_arch = "wasm32")]
              if let Some(s) = web_sys::window().and_then(|w| w.location().search().ok()) {
                  if let Some(id) = s.split("s=").nth(1).and_then(|x| x.split('&').next()) {
                      if !id.is_empty() { ss.set(id.to_string()); r.set(serde_json::to_value(list_rules(id).await).unwrap_or_default().as_array().cloned().unwrap_or_default()); }
                  }
              }
          }
      });

      view! {
          <Topbar breadcrumb="Editor de Regras"/>
          <div class="content">
              <div class="section-header" style="margin-bottom:16px"><div class="section-title">"Editor de Regras (UC03)"</div></div>
              <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:400px">
                  <label class="input-label">"Sistema"</label>
                  <select class="text-input" style="margin-bottom:0"
                      prop:value=move || selected_sys.get()
                      on:change=move |e| { selected_sys.set(event_target_value(&e)); load_rules(); }>
                      <option value="">"— Selecione —"</option>
                      {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                  </select>
              </div>
              {move || {
                  let sid = selected_sys.get();
                  if sid.is_empty() { return view! { <div class="empty-state">"Selecione um sistema"</div> }.into_any(); }
                  let rs = rules.get();
                  view! {
                      <div style="display:flex;gap:12px;align-items:center;margin-bottom:12px">
                          <a class="btn btn-primary" style="font-size:10px;padding:4px 10px" href={format!("/add-rule?s={}", sid)} target="_self">
                              <i class="ti ti-plus"></i>"Regra"
                          </a>
                      </div>
                      {if rs.is_empty() {
                          view! { <div class="empty-state">"Nenhuma regra ainda. Crie a primeira!"</div> }.into_any()
                      } else {
                          view! {
                              <div class="rule-hint">"Formato: SE &lt;variável&gt; É &lt;termo&gt; E ... ENTÃO &lt;variável&gt; É &lt;termo&gt; [peso]"</div>
                              {rs.iter().map(|r| {
                                  let text = r["rule_text"].as_str().unwrap_or("").to_string();
                                  let rid = r["id"].as_str().unwrap_or("").to_string();
                                  let rsid = selected_sys.get();
                                  let rid_del = rid.clone();
                                  let rsid_del = rsid.clone();
                                  let rules_del = rules.clone();
                                  view! {
                                      <div class="rule-row">
                                          <div class="rule-num">{r["position"].as_i64().unwrap_or(0)}</div>
                                          <div class="rule-text">"SE " {text}</div>
                                          <div class="rule-weight">"w=" {r["weight"].as_f64().unwrap_or(1.0)}</div>
                                          <a class="icon-btn" style="margin-left:8px" href={format!("/edit-rule?id={}&s={}", rid, rsid)}>
                                              <i class="ti ti-edit"></i>
                                          </a>
                                          <button class="icon-btn" style="color:var(--coral)" 
                                              on:click=move |_| { let id = rid_del.clone(); spawn_async({ let r = rules_del.clone(); let s = rsid_del.clone(); async move { delete_rule(&id).await; r.set(serde_json::to_value(list_rules(&s).await).unwrap_or_default().as_array().cloned().unwrap_or_default()); } }); }>
                                              <i class="ti ti-trash"></i>
                                          </button>
                                      </div>
                                  }
                              }).collect_view()}
                          }.into_any()
                      }}
                  }.into_any()
              }}
          </div>
      }
  }

 #[component]
 fn Simulador() -> impl IntoView {
     let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
     let selected_sys = RwSignal::new(String::new());
     let variables = RwSignal::new(Vec::<serde_json::Value>::new());
     let inputs = RwSignal::new(std::collections::HashMap::<String, f64>::new());
     let result = RwSignal::new(None::<serde_json::Value>);
     let loading = RwSignal::new(false);
     let city_input = RwSignal::new(String::new());
     let weather_msg = RwSignal::new(String::new());

     // Scenario state (UC12)
     let scenarios = RwSignal::new(Vec::<ScenarioInfo>::new());
     let scenario_name = RwSignal::new(String::new());
     let scenario_msg = RwSignal::new(String::new());

     // Sweep state (UC13)
     let sweep_var = RwSignal::new(String::new());
     let sweep_start = RwSignal::new("0".to_string());
     let sweep_end = RwSignal::new("100".to_string());
     let sweep_step = RwSignal::new("10".to_string());
     let sweep_result = RwSignal::new(None::<serde_json::Value>);
     let sweep_loading = RwSignal::new(false);

     spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await.into_iter().filter(|s| s.status != "desativado").collect()); } });

     let load_vars = {
         let ss = selected_sys.clone();
         let v = variables.clone();
         let i = inputs.clone();
         let r = result.clone();
         let sc = scenarios.clone();
         move || {
             let sid = ss.get();
             if !sid.is_empty() {
                 let v2 = v.clone();
                 let i2 = i.clone();
                 let r2 = r.clone();
                 let sc2 = sc.clone();
                 spawn_async(async move {
                     let vars = list_variables(&sid).await;
                     let mut map = std::collections::HashMap::new();
                     for var in &vars {
                         if var["role"].as_str() == Some("antecedent") {
                             if let Some(min) = var["universe_min"].as_f64() {
                                 map.insert(var["name"].as_str().unwrap_or("").to_string(), (min + var["universe_max"].as_f64().unwrap_or(100.0)) / 2.0);
                             }
                         }
                     }
                     i2.set(map);
                     v2.set(vars);
                     r2.set(None);
                     sc2.set(list_scenarios(&sid).await);
                 });
             }
         }
     };

     let run_sim = {
         let ss = selected_sys.clone();
         let inp = inputs.clone();
         let res = result.clone();
         let ld = loading.clone();
         move || {
             let sid = ss.get();
             if sid.is_empty() { return; }
             ld.set(true);
             let inp2 = inp.get();
             let r2 = res.clone();
             let l2 = ld.clone();
             spawn_async(async move {
                 let json_inputs = serde_json::json!(inp2);
                 let output = run_simulation(&sid, &json_inputs).await;
                 r2.set(output);
                 l2.set(false);
             });
         }
     };

     let fetch_weather = {
         let ci = city_input.clone();
         let wm = weather_msg.clone();
         let inp = inputs.clone();
         move || {
             let city = ci.get();
             if city.trim().is_empty() { wm.set("Informe uma cidade".into()); return; }
             let wm2 = wm.clone();
             let inp2 = inp.clone();
             spawn_async(async move {
                 match get_weather(&city).await {
                     Ok(w) => {
                         let mut current = inp2.get();
                         current.insert("temperatura".to_string(), w.temp);
                         current.insert("umidade".to_string(), w.humidity);
                         inp2.set(current);
                         wm2.set(format!("{}: {}°C, {}%", w.city, w.temp, w.humidity));
                     }
                     Err(e) => wm2.set(format!("Erro: {e}")),
                 }
             });
         }
     };

     let save_scenario = {
         let ss = selected_sys.clone();
         let inp = inputs.clone();
         let sn = scenario_name.clone();
         let sm = scenario_msg.clone();
         let sc = scenarios.clone();
         move || {
             let sid = ss.get();
             let name = sn.get();
             if sid.is_empty() || name.trim().is_empty() { sm.set("Informe um nome".into()); return; }
             let inputs_val = serde_json::json!(inp.get());
             let sc2 = sc.clone();
             let sm2 = sm.clone();
             spawn_async(async move {
                 if let Some(_) = create_scenario(&sid, &name, &inputs_val).await {
                     sc2.set(list_scenarios(&sid).await);
                     sm2.set("Cenario salvo!".into());
                 } else {
                     sm2.set("Erro ao salvar".into());
                 }
             });
         }
     };

     let load_scenario = {
         let inp = inputs.clone();
         move |s: ScenarioInfo| {
             if let Some(obj) = s.inputs.as_object() {
                 let mut map = std::collections::HashMap::new();
                 for (k, v) in obj {
                     if let Some(n) = v.as_f64() {
                         map.insert(k.clone(), n);
                     }
                 }
                 inp.set(map);
             }
         }
     };

     let delete_scenario_fn = {
         let ss = selected_sys.clone();
         let sc = scenarios.clone();
         move |id: String| {
             let sc2 = sc.clone();
             let sid = ss.get();
             spawn_async(async move {
                 delete_scenario(&id).await;
                 sc2.set(list_scenarios(&sid).await);
             });
         }
     };

     let run_sweep_fn = {
         let ss = selected_sys.clone();
         let sv = sweep_var.clone();
         let ss2 = sweep_start.clone();
         let se = sweep_end.clone();
         let sst = sweep_step.clone();
         let sr = sweep_result.clone();
         let sl = sweep_loading.clone();
         move || {
             let sid = ss.get();
             let var_name = sv.get();
             if sid.is_empty() || var_name.is_empty() { return; }
             let start: f64 = match ss2.get().parse() { Ok(v) => v, _ => { return; } };
             let end: f64 = match se.get().parse() { Ok(v) => v, _ => { return; } };
             let step: f64 = match sst.get().parse() { Ok(v) => v, _ => { return; } };
             sl.set(true);
             let sr2 = sr.clone();
             let sl2 = sl.clone();
             let fixed = inputs.get();
             spawn_async(async move {
                 let res = run_sweep(&sid, &var_name, start, end, step, &fixed).await;
                 sr2.set(res);
                 sl2.set(false);
             });
         }
     };

     let active_tab = RwSignal::new("mamdani".to_string());

     let tsk_inputs = RwSignal::new(std::collections::HashMap::<String, f64>::new());
     let tsk_coeffs_str = RwSignal::new(String::new());
     let tsk_result = RwSignal::new(None::<serde_json::Value>);
     let tsk_loading = RwSignal::new(false);

     let svg_result = RwSignal::new(None::<serde_json::Value>);
     let svg_loading = RwSignal::new(false);

     let diag_result = RwSignal::new(None::<serde_json::Value>);
     let diag_loading = RwSignal::new(false);

      view! {
         <Topbar breadcrumb="Simulador"/>
         <div class="content">
             <div class="section-header" style="margin-bottom:16px">
                 <div class="section-title">"Simulador"</div>
                 <div style="display:flex;gap:4px;font-size:11px">
                     <button class:btn-primary=move || active_tab.get() == "mamdani" class="btn-sm"
                         on:click=move |_| active_tab.set("mamdani".into())>"Mamdani"</button>
                     <button class:btn-primary=move || active_tab.get() == "tsk" class="btn-sm"
                         on:click=move |_| active_tab.set("tsk".into())>"TSK"</button>
                     <button class:btn-primary=move || active_tab.get() == "svg" class="btn-sm"
                         on:click=move |_| active_tab.set("svg".into())>"SVG"</button>
                     <button class:btn-primary=move || active_tab.get() == "diagnostic" class="btn-sm"
                         on:click=move |_| active_tab.set("diagnostic".into())>"Diagnóstico"</button>
                 </div>
             </div>

              <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:500px">
                  <label class="input-label">"Sistema"</label>
                  <select class="text-input" style="margin-bottom:0"
                      prop:value=move || selected_sys.get()
                      on:change=move |e| { selected_sys.set(event_target_value(&e)); load_vars(); }>
                      <option value="">"— Selecione —"</option>
                      {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                  </select>
              </div>

              // ─── Mamdani Tab ───
              {move || if active_tab.get() == "mamdani" { view! {
              <div class="sim-layout">
                  <div class="panel">
                      <div class="panel-title">"Entradas"</div>
                      <div style="display:flex;gap:8px;align-items:center;margin-bottom:16px">
                          <input type="text" class="text-input" style="margin-bottom:0;flex:1" placeholder="Cidade (ex: Belém)"
                              prop:value=move || city_input.get()
                              on:input=move |e| city_input.set(event_target_value(&e))/>
                          <button class="btn btn-primary" style="font-size:10px;padding:4px 10px" on:click=move |_| fetch_weather()>
                              <i class="ti ti-cloud"></i>"Buscar Clima"
                          </button>
                      </div>
                      {move || { let m = weather_msg.get(); if !m.is_empty() { view! { <div style="font-size:10px;color:var(--teal);margin-bottom:12px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                      {move || {
                          let vars = variables.get();
                          let antecedents: Vec<&serde_json::Value> = vars.iter().filter(|v| v["role"].as_str() == Some("antecedent")).collect();
                          if antecedents.is_empty() {
                              return view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Nenhuma variável antecedente. Configure-as no Editor de Variáveis."</div> }.into_any();
                          }
                           view! {
                              {antecedents.into_iter().map(|var| {
                                  let name = var["name"].as_str().unwrap_or("?").to_string();
                                  let min = var["universe_min"].as_f64().unwrap_or(0.0);
                                  let max = var["universe_max"].as_f64().unwrap_or(100.0);
                                   let name3 = name.clone();
                                   let name4 = name.clone();
                                   let name5 = name.clone();
                                   let name6 = name.clone();
                                   view! {
                                       <div class="input-group">
                                           <label class="input-label">{name.clone()}</label>
                                           <div class="input-row">
                                               <input type="range" class="range-input" min=min max=max step=0.1
                                                   prop:value=move || inputs.with(|m| m.get(&name5).copied().unwrap_or((min+max)/2.0))
                                                   on:input=move |e| {
                                                       if let Ok(n) = event_target_value(&e).parse::<f64>() {
                                                           inputs.update(|m| { m.insert(name3.clone(), n); });
                                                       }
                                                   }/>
                                               <input type="number" class="range-number" min=min max=max step=0.1
                                                   prop:value=move || inputs.with(|m| m.get(&name6).copied().unwrap_or((min+max)/2.0))
                                                   on:input=move |e| {
                                                       if let Ok(n) = event_target_value(&e).parse::<f64>() {
                                                           let clamped = n.clamp(min, max);
                                                           inputs.update(|m| { m.insert(name4.clone(), clamped); });
                                                       }
                                                   }/>
                                           </div>
                                           <div style="font-size:9px;color:var(--text3);display:flex;justify-content:space-between">
                                               <span>{min}</span><span>{max}</span>
                                           </div>
                                       </div>
                                   }
                              }).collect_view()}
                              <button class="btn btn-primary" on:click=move |_| run_sim() style="margin-top:8px">
                                  <i class="ti ti-player-play"></i>"Executar Simulação"
                              </button>
                          }.into_any()
                      }}
                  </div>
                  <div class="panel">
                      <div class="panel-title">"Resultado"</div>
                      {move || {
                          if loading.get() {
                              return view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Simulando..."</div> }.into_any();
                          }
                          match result.get() {
                              None => view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Execute uma simulação para ver o resultado."</div> }.into_any(),
                               Some(r) => {
                                   let outputs = r["outputs"].as_object().cloned().unwrap_or_default();
                                   let items = outputs.into_iter().collect::<Vec<_>>();
                                   view! {
                                       <For each=move || items.clone() key=|(k, _)| k.clone() let:item>
                                           {
                                               let v = item.1.as_f64().unwrap_or(0.0);
                                               view! {
                                                   <div class="output-display">
                                                       <div class="output-val">{format!("{:.2}", v)}</div>
                                                       <div class="output-label">{item.0.clone()}</div>
                                                   </div>
                                               }
                                           }
                                       </For>
                                   }.into_any()
                               }
                          }
                      }}
                   </div>
               </div>
               {move || {
                   let sid = selected_sys.get();
                   if sid.is_empty() { return view! {}.into_any(); }
                   let sc_list = scenarios.get();
                   view! {
                       <div style="margin-top:20px;display:flex;gap:20px;flex-wrap:wrap">
                           <div class="panel" style="flex:1;min-width:280px">
                               <div class="panel-title">"Cenarios (UC12)"</div>
                               <div style="display:flex;gap:6px;margin-bottom:8px">
                                   <input type="text" class="text-input" style="margin-bottom:0;flex:1;font-size:11px" placeholder="Nome do cenario"
                                       prop:value=move || scenario_name.get()
                                       on:input=move |e| scenario_name.set(event_target_value(&e))/>
                                   <button class="btn btn-primary" style="font-size:10px;padding:4px 10px" on:click=move |_| save_scenario()>
                                       <i class="ti ti-device-floppy"></i>"Salvar"
                                   </button>
                               </div>
                               {move || { let m = scenario_msg.get(); if !m.is_empty() { view! { <div style="font-size:10px;color:var(--teal);margin-bottom:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                               {if sc_list.is_empty() {
                                   view! { <div style="font-size:10px;color:var(--text3)">"Nenhum cenario salvo."</div> }.into_any()
                               } else {
                                   view! {
                                       <div style="max-height:200px;overflow-y:auto">
                                           {sc_list.into_iter().map(|sc| {
                                               let sc_id = sc.id.clone();
                                               let sc_name = sc.name.clone();
                                               let load_fn = load_scenario.clone();
                                               let del_fn = delete_scenario_fn.clone();
                                               view! {
                                                   <div style="display:flex;justify-content:space-between;align-items:center;padding:4px 0;border-bottom:1px solid var(--border)">
                                                       <span style="font-size:11px">{sc_name.clone()}</span>
                                                       <div style="display:flex;gap:4px">
                                                           <button class="icon-btn" title="Carregar" on:click=move |_| load_fn(sc.clone())>
                                                               <i class="ti ti-upload"></i>
                                                           </button>
                                                           <button class="icon-btn" title="Deletar" on:click=move |_| del_fn(sc_id.clone())>
                                                               <i class="ti ti-trash"></i>
                                                           </button>
                                                       </div>
                                                   </div>
                                               }
                                           }).collect_view()}
                                       </div>
                                   }.into_any()
                               }}
                           </div>
                           <div class="panel" style="flex:1;min-width:280px">
                               <div class="panel-title">"Varredura - Sweep (UC13)"</div>
                               <div style="display:flex;gap:6px;flex-wrap:wrap;margin-bottom:8px">
                                   <select class="text-input" style="margin-bottom:0;font-size:10px;flex:1"
                                       prop:value=move || sweep_var.get()
                                       on:change=move |e| sweep_var.set(event_target_value(&e))>
                                       <option value="">"-- Variavel --"</option>
                                       {move || variables.get().iter().filter(|v| v["role"].as_str() == Some("antecedent")).map(|v| {
                                           let name = v["name"].as_str().unwrap_or("").to_string();
                                           view! { <option value={name.clone()}>{name.clone()}</option> }
                                       }).collect_view()}
                                   </select>
                               </div>
                               <div style="display:flex;gap:6px;flex-wrap:wrap">
                                   <div><label style="font-size:9px;color:var(--text3)">"Inicio"</label><input type="number" class="text-input" style="margin-bottom:0;font-size:10px;width:70px" prop:value=move || sweep_start.get() on:input=move |e| sweep_start.set(event_target_value(&e))/></div>
                                   <div><label style="font-size:9px;color:var(--text3)">"Fim"</label><input type="number" class="text-input" style="margin-bottom:0;font-size:10px;width:70px" prop:value=move || sweep_end.get() on:input=move |e| sweep_end.set(event_target_value(&e))/></div>
                                   <div><label style="font-size:9px;color:var(--text3)">"Passo"</label><input type="number" class="text-input" style="margin-bottom:0;font-size:10px;width:70px" prop:value=move || sweep_step.get() on:input=move |e| sweep_step.set(event_target_value(&e))/></div>
                                   <button class="btn btn-primary" style="font-size:10px;padding:4px 10px;align-self:flex-end" on:click=move |_| run_sweep_fn()>
                                       <i class="ti ti-wave-sine"></i>"Varrer"
                                   </button>
                               </div>
                               {move || {
                                   if sweep_loading.get() { return view! { <div style="font-size:10px;color:var(--text3);margin-top:8px">"Varrendo..."</div> }.into_any(); }
                                   match sweep_result.get() {
                                       None => view! {}.into_any(),
                                       Some(res) => {
                                           let points = res["points"].as_array().cloned().unwrap_or_default();
                                           view! {
                                               <div style="margin-top:8px;max-height:200px;overflow-y:auto">
                                                   <table style="width:100%;font-size:10px">
                                                       <thead><tr><th>"x"</th><th>"y"</th></tr></thead>
                                                       <tbody>{points.into_iter().map(|p| {
                                                           let x = p["x"].as_f64().unwrap_or(0.0);
                                                           let y = p["y"].as_f64().unwrap_or(0.0);
                                                           view! { <tr><td>{format!("{:.2}", x)}</td><td>{format!("{:.2}", y)}</td></tr> }
                                                       }).collect_view()}</tbody>
                                                   </table>
                                               </div>
                                           }.into_any()
                                       }
                                   }
                               }}
                           </div>
                       </div>
                   }.into_any()
               }}
              }.into_any() } else { view! {}.into_any() }}

              // ─── TSK Tab ───
              {move || if active_tab.get() == "tsk" { view! {
              <div class="sim-layout">
                  <div class="panel">
                      <div class="panel-title">"Entradas (TSK - UC18)"</div>
                      {move || {
                          let vars = variables.get();
                          let antecedents: Vec<&serde_json::Value> = vars.iter().filter(|v| v["role"].as_str() == Some("antecedent")).collect();
                          if antecedents.is_empty() {
                              return view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Nenhuma variável antecedente."</div> }.into_any();
                          }
                          view! {
                              {antecedents.into_iter().map(|var| {
                                  let name = var["name"].as_str().unwrap_or("?").to_string();
                                  let min = var["universe_min"].as_f64().unwrap_or(0.0);
                                  let max = var["universe_max"].as_f64().unwrap_or(100.0);
                                  let n2 = name.clone();
                                  let n3 = name.clone();
                                  view! {
                                      <div class="input-group">
                                          <label class="input-label">{name.clone()}</label>
                                          <input type="number" class="text-input" min=min max=max step=0.1
                                              prop:value=move || tsk_inputs.with(|m| m.get(&n3).copied().unwrap_or((min+max)/2.0))
                                              on:input=move |e| {
                                                  if let Ok(n) = event_target_value(&e).parse::<f64>() {
                                                      tsk_inputs.update(|m| { m.insert(n2.clone(), n.clamp(min, max)); });
                                                  }
                                              }/>
                                      </div>
                                  }
                              }).collect_view()}
                              <div style="margin-top:12px">
                                  <label class="input-label">"Coeficientes TSK (JSON, ex: {\"Risco_Alto\": [50, 0.5]})"</label>
                                  <textarea class="text-input" style="min-height:60px;font-family:monospace;font-size:10px"
                                      prop:value=move || tsk_coeffs_str.get()
                                      on:input=move |e| tsk_coeffs_str.set(event_target_value(&e))></textarea>
                              </div>
                              <button class="btn btn-primary" style="margin-top:12px" on:click={
                                  let ss = selected_sys.clone();
                                  let inp = tsk_inputs.clone();
                                  let cs = tsk_coeffs_str.clone();
                                  let r2 = tsk_result.clone();
                                  let ld = tsk_loading.clone();
                                  move |_| {
                                      let sid = ss.get();
                                      if sid.is_empty() { return; }
                                      ld.set(true);
                                      let inputs_val = serde_json::json!(inp.get());
                                      let coeffs_val: serde_json::Value = match serde_json::from_str(&cs.get()) {
                                          Ok(v) => v, _ => { ld.set(false); return; }
                                      };
                                      let r3 = r2.clone();
                                      let l2 = ld.clone();
                                      spawn_async(async move {
                                          let res = run_tsk_simulation(&sid, &inputs_val, &coeffs_val).await;
                                          r3.set(res);
                                          l2.set(false);
                                      });
                                  }
                              }>
                                  <i class="ti ti-player-play"></i>"Executar TSK"
                              </button>
                          }.into_any()
                      }}
                  </div>
                  <div class="panel">
                      <div class="panel-title">"Resultado TSK"</div>
                      {move || {
                          if tsk_loading.get() { return view! { <div style="font-size:11px;color:var(--text3);padding:16px 0">"Calculando..."</div> }.into_any(); }
                          match tsk_result.get() {
                              None => view! { <div style="font-size:11px;color:var(--text3);padding:16px 0">"Configure os coeficientes e execute."</div> }.into_any(),
                              Some(r) => {
                                  let outputs = r["outputs"].as_object().cloned().unwrap_or_default();
                                  view! {
                                      {outputs.into_iter().map(|(k, v)| {
                                          let val = v.as_f64().unwrap_or(0.0);
                                          view! {
                                              <div class="output-display">
                                                  <div class="output-val">{format!("{:.4}", val)}</div>
                                                  <div class="output-label">{k}</div>
                                              </div>
                                          }
                                      }).collect_view()}
                                  }.into_any()
                              }
                          }
                      }}
                  </div>
              </div>
              }.into_any() } else { view! {}.into_any() }}

              // ─── SVG Tab ───
              {move || if active_tab.get() == "svg" { view! {
              <div>
                  <button class="btn btn-primary" style="margin-bottom:16px" on:click={
                      let ss = selected_sys.clone();
                      let sr = svg_result.clone();
                      let sl = svg_loading.clone();
                      move |_| {
                          let sid = ss.get();
                          if sid.is_empty() { return; }
                          sl.set(true);
                          let sr2 = sr.clone();
                          let sl2 = sl.clone();
                          spawn_async(async move {
                              let res = get_svg_export(&sid).await;
                              sr2.set(res);
                              sl2.set(false);
                          });
                      }
                  }>
                      <i class="ti ti-file-type-svg"></i>"Gerar SVG"
                  </button>
                  {move || {
                      if svg_loading.get() { return view! { <div style="font-size:11px;color:var(--text3)">"Gerando..."</div> }.into_any(); }
                      match svg_result.get() {
                          None => view! { <div style="font-size:11px;color:var(--text3)">"Clique em \"Gerar SVG\" para visualizar as funções de pertinência."</div> }.into_any(),
                          Some(r) => {
                              let svgs = r["svgs"].as_array().cloned().unwrap_or_default();
                              view! {
                                  <div style="display:flex;flex-wrap:wrap;gap:16px">
                                      {svgs.into_iter().map(|item| {
                                          let name = item["variable"].as_str().unwrap_or("?").to_string();
                                          let svg_raw = item["svg"].as_str().unwrap_or("").to_string();
                                          view! {
                                              <div class="panel" style="flex:1;min-width:300px">
                                                  <div class="panel-title">{name}</div>
                                                  <div inner_html=svg_raw></div>
                                              </div>
                                          }
                                      }).collect_view()}
                                  </div>
                              }.into_any()
                          }
                      }
                  }}
              </div>
              }.into_any() } else { view! {}.into_any() }}

              // ─── Diagnostic Tab ───
              {move || if active_tab.get() == "diagnostic" { view! {
              <div class="sim-layout">
                  <div class="panel">
                      <div class="panel-title">"Entradas (Diagnóstico - UC20)"</div>
                      {move || {
                          let vars = variables.get();
                          let antecedents: Vec<&serde_json::Value> = vars.iter().filter(|v| v["role"].as_str() == Some("antecedent")).collect();
                          if antecedents.is_empty() {
                              return view! { <div style="color:var(--text3);font-size:11px;padding:16px 0">"Nenhuma variável antecedente."</div> }.into_any();
                          }
                          view! {
                              {antecedents.into_iter().map(|var| {
                                  let name = var["name"].as_str().unwrap_or("?").to_string();
                                  let min = var["universe_min"].as_f64().unwrap_or(0.0);
                                  let max = var["universe_max"].as_f64().unwrap_or(100.0);
                                  let n2 = name.clone();
                                  let n3 = name.clone();
                                  view! {
                                      <div class="input-group">
                                          <label class="input-label">{name.clone()}</label>
                                          <input type="number" class="text-input" min=min max=max step=0.1
                                              prop:value=move || inputs.with(|m| m.get(&n3).copied().unwrap_or((min+max)/2.0))
                                              on:input=move |e| {
                                                  if let Ok(n) = event_target_value(&e).parse::<f64>() {
                                                      inputs.update(|m| { m.insert(n2.clone(), n.clamp(min, max)); });
                                                  }
                                              }/>
                                      </div>
                                  }
                              }).collect_view()}
                              <button class="btn btn-primary" style="margin-top:12px" on:click={
                                  let ss = selected_sys.clone();
                                  let inp = inputs.clone();
                                  let dr = diag_result.clone();
                                  let dl = diag_loading.clone();
                                  move |_| {
                                      let sid = ss.get();
                                      if sid.is_empty() { return; }
                                      dl.set(true);
                                      let inputs_val = serde_json::json!(inp.get());
                                      let dr2 = dr.clone();
                                      let dl2 = dl.clone();
                                      spawn_async(async move {
                                          let res = get_diagnostic(&sid, &inputs_val).await;
                                          dr2.set(res);
                                          dl2.set(false);
                                      });
                                  }
                              }>
                                  <i class="ti ti-report-analytics"></i>"Gerar Diagnóstico"
                              </button>
                          }.into_any()
                      }}
                  </div>
                  <div class="panel">
                      <div class="panel-title">"Diagnóstico"</div>
                      {move || {
                          if diag_loading.get() { return view! { <div style="font-size:11px;color:var(--text3);padding:16px 0">"Analisando..."</div> }.into_any(); }
                          match diag_result.get() {
                              None => view! { <div style="font-size:11px;color:var(--text3);padding:16px 0">"Configure as entradas e gere o diagnóstico."</div> }.into_any(),
                              Some(r) => {
                                  let fuzz = r["fuzzification"].as_array().cloned().unwrap_or_default();
                                  let firings = r["rule_firings"].as_array().cloned().unwrap_or_default();
                                  view! {
                                      <details open>
                                          <summary style="cursor:pointer;font-size:11px;color:var(--teal);font-weight:600">
                                              "Fuzzificação (" {fuzz.len()} " variáveis)"
                                          </summary>
                                          {fuzz.iter().map(|fv| {
                                              let vname = fv["variable"].as_str().unwrap_or("?").to_string();
                                              let crisp = fv["crisp_input"].as_f64().unwrap_or(0.0);
                                              let terms = fv["term_degrees"].as_array().cloned().unwrap_or_default();
                                              view! {
                                                  <div style="margin:8px 0;padding:8px;background:var(--surface1);border-radius:4px;font-size:10px">
                                                      <div style="font-weight:600">{vname} <span style="color:var(--amber)">= {format!("{:.2}", crisp)}</span></div>
                                                      {terms.clone().iter().map(|t| {
                                                          let label = t["term"].as_str().unwrap_or("").to_string();
                                                          let mu = t["mu"].as_f64().unwrap_or(0.0);
                                                          view! { <div style="padding-left:12px">{label}": " {format!("{:.4}", mu)}</div> }
                                                      }).collect_view()}
                                                  </div>
                                              }
                                          }).collect_view()}
                                      </details>
                                      <details style="margin-top:12px">
                                          <summary style="cursor:pointer;font-size:11px;color:var(--teal);font-weight:600">
                                              "Regras Disparadas (" {firings.len()} ")"
                                          </summary>
                                          {firings.iter().map(|rf| {
                                              let text = rf["rule_text"].as_str().unwrap_or("?").to_string();
                                              let degree = rf["firing_degree"].as_f64().unwrap_or(0.0);
                                              let fired = rf["fired"].as_bool().unwrap_or(false);
                                              view! {
                                                  <div style="margin:4px 0;padding:4px 8px;background:var(--surface1);border-radius:4px;font-size:10px">
                                                      <span>{text}</span>
                                                      <span style="margin-left:8px;color:var(--amber)">"μ=" {format!("{:.4}", degree)}</span>
                                                      <span style=format!("margin-left:8px;color:{}", if fired { "var(--green)" } else { "var(--coral)" })>
                                                          {if fired { "✓" } else { "✗" }}
                                                      </span>
                                                  </div>
                                              }
                                          }).collect_view()}
                                      </details>
                                      <details style="margin-top:12px">
                                          <summary style="cursor:pointer;font-size:11px;color:var(--teal);font-weight:600">
                                              "Saídas"
                                          </summary>
                                          {r["outputs"].as_object().map(|outs| {
                                              outs.iter().map(|(k, v)| {
                                                  let val = v.as_f64().unwrap_or(0.0);
                                                  view! {
                                                      <div class="output-display">
                                                          <div class="output-val">{format!("{:.4}", val)}</div>
                                                          <div class="output-label">{k.clone()}</div>
                                                      </div>
                                                  }
                                              }).collect_view()
                                          }).unwrap_or_default()}
                                      </details>
                                  }.into_any()
                              }
                          }
                      }}
                  </div>
              </div>
              }.into_any() } else { view! {}.into_any() }}
          </div>
      }
  }

 #[component]
 fn Historico() -> impl IntoView {
     let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
     let selected_sys = RwSignal::new(String::new());
     let sims = RwSignal::new(Vec::<serde_json::Value>::new());
     let selected_ids = RwSignal::new(std::collections::HashSet::<String>::new());
     let compare_result = RwSignal::new(None::<Vec<serde_json::Value>>);
     let compare_loading = RwSignal::new(false);
     let export_msg = RwSignal::new(String::new());

    spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

    let load_sims = {
        let ss = selected_sys.clone();
        let s = sims.clone();
        let si = selected_ids.clone();
        let cr = compare_result.clone();
        move |sid: String| {
            ss.set(sid.clone());
            si.set(std::collections::HashSet::new());
            cr.set(None);
            let s2 = s.clone();
            spawn_async(async move { s2.set(list_simulations(&sid).await.into_iter().map(|si| serde_json::json!(si)).collect()); });
        }
    };

    let toggle_select = {
        let si = selected_ids.clone();
        move |id: String| {
            si.update(|set| {
                if set.contains(&id) { set.remove(&id); }
                else { set.insert(id); }
            });
        }
    };

    let run_compare = {
        let si = selected_ids.clone();
        let cr = compare_result.clone();
        let cl = compare_loading.clone();
        move || {
            let ids: Vec<String> = si.get().into_iter().collect();
            if ids.len() < 2 { return; }
            cl.set(true);
            let cr2 = cr.clone();
            let cl2 = cl.clone();
            spawn_async(async move {
                let res = compare_simulations(&ids).await;
                cr2.set(res.map(|v| v.into_iter().map(|si| serde_json::json!(si)).collect()));
                cl2.set(false);
            });
        }
    };

    let export_report_fn = {
        let em = export_msg.clone();
        move |id: String| {
            let em2 = em.clone();
            spawn_async(async move {
                match export_simulation_report(&id).await {
                    Some(_data) => {
                        em2.set("Relatorio copiado para area de transferencia!".into());
                        #[cfg(target_arch = "wasm32")]
                        if let Some(w) = web_sys::window() {
                            let _ = w.navigator().clipboard().write_text(&serde_json::to_string_pretty(&_data).unwrap_or_default());
                        }
                    }
                    None => em2.set("Erro ao exportar".into()),
                }
            });
        }
    };

    view! {
        <Topbar breadcrumb="Histórico"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px"><div class="section-title">"Histórico (UC06)"</div></div>
            <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:400px">
                <label class="input-label">"Sistema"</label>
                <select class="text-input" style="margin-bottom:0"
                    on:change=move |e| { load_sims(event_target_value(&e)); }>
                     <option value="">"— Selecione —"</option>
                     {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                 </select>
             </div>

             <div style="display:flex;gap:8px;margin-bottom:12px;align-items:center">
                 <button class="btn btn-primary" style="font-size:10px;padding:4px 10px" on:click=move |_| run_compare()>
                     <i class="ti ti-arrows-left-right"></i>"Comparar Selecionados (UC08)"
                 </button>
                 {move || { let m = export_msg.get(); if !m.is_empty() { view! { <span style="font-size:10px;color:var(--teal)">{m}</span> }.into_any() } else { view! {}.into_any() } }}
             </div>

             {move || {
                 if compare_loading.get() { return view! { <div style="font-size:11px;color:var(--text3);padding:8px 0">"Comparando..."</div> }.into_any(); }
                 if let Some(comp) = compare_result.get() {
                     return view! {
                         <div class="panel" style="margin-bottom:16px">
                             <div class="panel-title">"Comparacao"</div>
                             <table class="hist-table"><thead><tr><th>"#"</th><th>"Entradas"</th><th>"Saida"</th><th>"Data"</th></tr></thead>
                             <tbody>{comp.iter().enumerate().map(|(i, s)| {
                                 view! { <tr><td>{i + 1}</td><td style="font-size:10px">{s["inputs"].to_string()}</td><td>{s["outputs"].to_string()}</td><td>{s["executed_at"].as_str().unwrap_or("")[..19].replace("T"," ")}</td></tr> }
                             }).collect_view()}</tbody></table>
                         </div>
                     }.into_any();
                 }
                 let list = sims.get();
                 if list.is_empty() { return view! { <div class="empty-state">"Nenhuma simulacao encontrada."</div> }.into_any(); }
                 view! {
                     <div class="hist-wrap">
                         <table class="hist-table"><thead><tr><th style="width:30px">""</th><th>"Entradas"</th><th>"Saida"</th><th>"Data"</th><th style="width:40px">""</th></tr></thead>
                         <tbody>{list.iter().map(|s| {
                             let sim_id = s["id"].as_str().unwrap_or("").to_string();
                             let is_selected = selected_ids.with(|set| set.contains(&sim_id));
                             view! {
                                 <tr style={if is_selected { "background:var(--surface2)" } else { "" }}>
                                     <td><input type="checkbox" checked=is_selected on:click={let sid = sim_id.clone(); move |_| toggle_select(sid.clone())}/></td>
                                     <td style="font-size:10px">{s["inputs"].to_string()}</td>
                                     <td>{s["outputs"].to_string()}</td>
                                     <td>{s["executed_at"].as_str().unwrap_or("")[..19].replace("T"," ")}</td>
                                     <td>
                                         <button class="icon-btn" title="Exportar Relatorio (UC09)" on:click={let sid = sim_id.clone(); move |_| export_report_fn(sid.clone())}>
                                             <i class="ti ti-file-export"></i>
                                         </button>
                                     </td>
                                 </tr>
                             }
                         }).collect_view()}</tbody>
                     </table>
                     </div>
                 }.into_any()
             }}
         </div>
     }
 }

 #[component]
 fn BatchDashboard() -> impl IntoView {
     let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
     let selected_sys = RwSignal::new(String::new());
     let json_input = RwSignal::new(String::new());
     let batch_result = RwSignal::new(None::<BatchResponse>);
     let batch_loading = RwSignal::new(false);
     let batch_error = RwSignal::new(None::<String>);
     let history = RwSignal::new(Vec::<serde_json::Value>::new());

     spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

     let load_history = {
         let ss = selected_sys.clone();
         let h = history.clone();
         move || {
             let sid = ss.get();
             if !sid.is_empty() {
                 let h2 = h.clone();
                 spawn_async(async move { h2.set(list_batch_results(&sid).await); });
             }
         }
     };

     let run_batch = {
         let ss = selected_sys.clone();
         let ji = json_input.clone();
         let br = batch_result.clone();
         let bl = batch_loading.clone();
         let be = batch_error.clone();
         let h = history.clone();
         move || {
             let sid = ss.get();
             if sid.is_empty() { be.set(Some("Selecione um sistema".into())); return; }
             let text = ji.get();
             if text.trim().is_empty() { be.set(Some("Cole os inputs em formato JSON".into())); return; }
             let parsed: Result<serde_json::Value, _> = serde_json::from_str(&text);
             match parsed {
                 Ok(val) => {
                     bl.set(true); be.set(None);
                     let br2 = br.clone(); let bl2 = bl.clone(); let be2 = be.clone();
                     let h2 = h.clone(); let sid2 = sid.clone();
                     spawn_async(async move {
                         match process_batch(&sid2, &val).await {
                             Some(resp) => {
                                 br2.set(Some(resp));
                                 h2.set(list_batch_results(&sid2).await);
                             }
                             None => be2.set(Some("Erro ao processar batch".into())),
                         }
                         bl2.set(false);
                     });
                 }
                 Err(e) => be.set(Some(format!("JSON inválido: {}", e))),
             }
         }
     };

     view! {
         <Topbar breadcrumb="Inferência em Lote"/>
         <div class="content">
             <div class="section-header"><div class="section-title">"Dashboard Batch (UC07)"</div></div>
             <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:500px">
                 <label class="input-label">"Sistema"</label>
                 <select class="text-input" style="margin-bottom:0"
                     prop:value=move || selected_sys.get()
                     on:change=move |e| { selected_sys.set(event_target_value(&e)); load_history(); }>
                     <option value="">"— Selecione —"</option>
                     {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                 </select>
             </div>

             <div style="display:flex;gap:20px;flex-wrap:wrap">
                 <div class="panel" style="flex:1;min-width:300px">
                     <div class="panel-title">"Processar Lote"</div>
                     <div style="margin-bottom:8px">
                         <label class="input-label">"Inputs (array JSON)"</label>
                         <textarea class="text-input" style="min-height:150px;font-family:monospace;font-size:10px;width:100%;resize:vertical"
                             prop:value=move || json_input.get()
                             on:input=move |e| json_input.set(event_target_value(&e))
                             placeholder=r#"[
  {"impacto_financeiro": 70, "impacto_mercado": 10},
  {"impacto_financeiro": 30, "impacto_mercado": 80},
  {"impacto_financeiro": 90, "impacto_mercado": 90}
]"#></textarea>
                     </div>
                     <button class="btn btn-primary"
                         on:click=move |_| run_batch()
                         disabled=move || batch_loading.get()>
                         {move || if batch_loading.get() { "Processando..." } else { "Executar Lote" }}
                     </button>
                     {move || batch_error.get().map(|e| view! {
                         <div style="color:var(--red);font-size:10px;margin-top:8px">{e}</div>
                     })}
                 </div>

                 <div class="panel" style="flex:2;min-width:400px">
                     <div class="panel-title">"Resultados"</div>
                     {move || {
                         match batch_result.get() {
                             None => view! { <div style="font-size:10px;color:var(--text3);padding:8px 0">"Execute um lote para ver os resultados."</div> }.into_any(),
                             Some(res) => {
                                 let err_style = if res.errors > 0 { "color:var(--red)" } else { "color:var(--teal)" };
                                 view! {
                                     <div style="display:flex;gap:16px;margin-bottom:12px;font-size:11px">
                                         <div>"Processados: " <strong>{res.processed}</strong></div>
                                          <div>"Erros: " <strong style={err_style}>{res.errors}</strong></div>
                                         <div>"Total: " <strong>{res.total}</strong></div>
                                     </div>
                                     <div style="max-height:400px;overflow-y:auto">
                                         <table style="width:100%;font-size:10px;border-collapse:collapse">
                                             <thead><tr style="background:var(--surface2)">
                                                 <th style="padding:4px;text-align:left">"#"</th>
                                                 <th style="padding:4px;text-align:left">"Inputs"</th>
                                                 <th style="padding:4px;text-align:right">"Output"</th>
                                             </tr></thead>
                                             <tbody>{res.results.iter().map(|r| {
                                                 let inputs_str = r.inputs.to_string();
                                                 view! {
                                                     <tr style="border-bottom:1px solid var(--border)">
                                                         <td style="padding:4px">{r.row_index + 1}</td>
                                                         <td style="padding:4px;font-size:9px;max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{inputs_str}</td>
                                                         <td style="padding:4px;text-align:right;font-weight:bold">{format!("{:.2}", r.output)}</td>
                                                     </tr>
                                                 }
                                             }).collect_view()}</tbody>
                                         </table>
                                     </div>
                                 }.into_any()
                             }
                         }
                     }}
                 </div>
             </div>

             <div class="panel" style="margin-top:16px">
                 <div class="panel-title">"Histórico de Processamentos"</div>
                 {move || {
                     let items = history.get();
                     if items.is_empty() {
                         return view! { <div style="font-size:10px;color:var(--text3);padding:8px 0">"Nenhum processamento batch encontrado para este sistema."</div> }.into_any();
                     }
                     view! {
                         <div style="max-height:300px;overflow-y:auto">
                             <table style="width:100%;font-size:10px;border-collapse:collapse">
                                 <thead><tr style="background:var(--surface2)">
                                     <th style="padding:4px;text-align:left">"Linha"</th>
                                     <th style="padding:4px;text-align:left">"Inputs"</th>
                                     <th style="padding:4px;text-align:right">"Output"</th>
                                     <th style="padding:4px;text-align:right">"Data"</th>
                                 </tr></thead>
                                 <tbody>{items.iter().map(|item| {
                                     let inputs_str = item["inputs"].to_string();
                                     let output = item["output"].as_f64().unwrap_or(0.0);
                                     let idx = item["row_index"].as_i64().unwrap_or(0);
                                     let date = item["executed_at"].as_str().unwrap_or("").to_string();
                                     let short_date = if date.len() > 19 { date[..19].to_string() } else { date.clone() };
                                     view! {
                                         <tr style="border-bottom:1px solid var(--border)">
                                             <td style="padding:4px">{idx + 1}</td>
                                             <td style="padding:4px;font-size:9px;max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{inputs_str}</td>
                                             <td style="padding:4px;text-align:right;font-weight:bold">{format!("{:.2}", output)}</td>
                                             <td style="padding:4px;text-align:right;font-size:9px;color:var(--text3)">{short_date}</td>
                                         </tr>
                                     }
                                 }).collect_view()}</tbody>
                             </table>
                         </div>
                     }.into_any()
                 }}
             </div>
         </div>
     }
 }

 #[component]
 fn Analise() -> impl IntoView {
     let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
     let selected_sys = RwSignal::new(String::new());
     let variables = RwSignal::new(Vec::<serde_json::Value>::new());
     let antecedents = RwSignal::new(Vec::<serde_json::Value>::new());
     let surf_x = RwSignal::new(String::new());
     let surf_y = RwSignal::new(String::new());
     let surf_res = RwSignal::new("20".to_string());
     let surf_result = RwSignal::new(None::<serde_json::Value>);
     let surf_loading = RwSignal::new(false);
     let matrix_result = RwSignal::new(None::<serde_json::Value>);
     let matrix_loading = RwSignal::new(false);

     spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

     let load_vars = {
         let ss = selected_sys.clone();
         let v = variables.clone();
         let a = antecedents.clone();
         move || {
             let sid = ss.get();
             if !sid.is_empty() {
                 let v2 = v.clone();
                 let a2 = a.clone();
                 spawn_async(async move {
                     let vars = list_variables(&sid).await;
                     let ants: Vec<serde_json::Value> = vars.iter().filter(|x| x["role"].as_str() == Some("antecedent")).cloned().collect();
                     v2.set(vars);
                     a2.set(ants);
                 });
             }
         }
     };

     let run_surface = {
         let ss = selected_sys.clone();
         let sx = surf_x.clone();
         let sy = surf_y.clone();
         let sr = surf_res.clone();
         let sres = surf_result.clone();
         let sl = surf_loading.clone();
         move || {
             let sid = ss.get();
             let x_name = sx.get();
             let y_name = sy.get();
             if sid.is_empty() || x_name.is_empty() || y_name.is_empty() { return; }
             let res: usize = match sr.get().parse::<usize>() { Ok(v) => v.min(50).max(5), _ => 20 };
             sl.set(true);
             let sres2 = sres.clone();
             let sl2 = sl.clone();
             spawn_async(async move {
                 let data = run_surface(&sid, &x_name, &y_name, Some(res), Some(res)).await;
                 sres2.set(data);
                 sl2.set(false);
             });
         }
     };

     let run_matrix = {
         let ss = selected_sys.clone();
         let mr = matrix_result.clone();
         let ml = matrix_loading.clone();
         let ants = antecedents.clone();
         move || {
             let sid = ss.get();
             if sid.is_empty() { return; }
             let mut mid_inputs = std::collections::HashMap::new();
             for var in ants.get().iter() {
                 let name = var["name"].as_str().unwrap_or("").to_string();
                 let min = var["universe_min"].as_f64().unwrap_or(0.0);
                 let max = var["universe_max"].as_f64().unwrap_or(100.0);
                 mid_inputs.insert(name, (min + max) / 2.0);
             }
             ml.set(true);
             let mr2 = mr.clone();
             let ml2 = ml.clone();
             let inputs_val = serde_json::json!(mid_inputs);
             spawn_async(async move {
                 let data = get_rule_matrix(&sid, &inputs_val).await;
                 mr2.set(data);
                 ml2.set(false);
             });
         }
     };

     view! {
         <Topbar breadcrumb="Análise"/>
         <div class="content">
             <div class="section-header" style="margin-bottom:16px"><div class="section-title">"Superficie & Matriz de Regras"</div></div>
             <div class="panel" style="margin-bottom:16px;padding:12px 16px;max-width:500px">
                 <label class="input-label">"Sistema"</label>
                 <select class="text-input" style="margin-bottom:0"
                     prop:value=move || selected_sys.get()
                     on:change=move |e| { selected_sys.set(event_target_value(&e)); load_vars(); }>
                     <option value="">"— Selecione —"</option>
                     {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                 </select>
             </div>

             <div style="display:flex;gap:20px;flex-wrap:wrap">
                 <div class="panel" style="flex:1;min-width:300px">
                     <div class="panel-title">"Superficie de Controle (UC15)"</div>
                     <div style="display:flex;gap:6px;flex-wrap:wrap;margin-bottom:8px">
                         <select class="text-input" style="margin-bottom:0;font-size:10px;flex:1"
                             prop:value=move || surf_x.get()
                             on:change=move |e| surf_x.set(event_target_value(&e))>
                             <option value="">"-- Eixo X --"</option>
                             {move || antecedents.get().iter().map(|v| {
                                 let name = v["name"].as_str().unwrap_or("").to_string();
                                 view! { <option value={name.clone()}>{name.clone()}</option> }
                             }).collect_view()}
                         </select>
                         <select class="text-input" style="margin-bottom:0;font-size:10px;flex:1"
                             prop:value=move || surf_y.get()
                             on:change=move |e| surf_y.set(event_target_value(&e))>
                             <option value="">"-- Eixo Y --"</option>
                             {move || antecedents.get().iter().map(|v| {
                                 let name = v["name"].as_str().unwrap_or("").to_string();
                                 view! { <option value={name.clone()}>{name.clone()}</option> }
                             }).collect_view()}
                         </select>
                         <div><label style="font-size:9px;color:var(--text3)">"Res"</label><input type="number" class="text-input" style="margin-bottom:0;font-size:10px;width:60px" min=5 max=50 prop:value=move || surf_res.get() on:input=move |e| surf_res.set(event_target_value(&e))/></div>
                         <button class="btn btn-primary" style="font-size:10px;padding:4px 10px;align-self:flex-end" on:click=move |_| run_surface()>
                             <i class="ti ti-chart-grid-dots"></i>"Gerar"
                         </button>
                     </div>
                     {move || {
                         if surf_loading.get() { return view! { <div style="font-size:10px;color:var(--text3);padding:8px 0">"Calculando superficie..."</div> }.into_any(); }
                         match surf_result.get() {
                             None => view! { <div style="font-size:10px;color:var(--text3);padding:8px 0">"Selecione variaveis e gere a superficie."</div> }.into_any(),
                             Some(res) => {
                                 let grid = res["grid"].as_array().cloned().unwrap_or_default();
                                 let n = (grid.len() as f64).sqrt() as usize;
                                 if grid.is_empty() || n == 0 { return view! { <div style="font-size:10px;color:var(--text3)">"Sem dados"</div> }.into_any(); }
                                 let x_var = res["x_var"].as_str().unwrap_or("X");
                                 let y_var = res["y_var"].as_str().unwrap_or("Y");
                                 let mut min_z = f64::MAX; let mut max_z = f64::MIN;
                                 for p in &grid { if let Some(z) = p["z"].as_f64() { min_z = min_z.min(z); max_z = max_z.max(z); } }
                                 let range = (max_z - min_z).max(0.001);
                                 view! {
                                     <div style="font-size:9px;color:var(--text3);margin-bottom:6px">
                                         {format!("{x_var} x {y_var}  {n}x{n} grid  z in [{min_z:.2}, {max_z:.2}]")}
                                     </div>
                                     <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(8px,1fr));gap:1px;max-height:300px;overflow-y:auto;border:1px solid var(--border);padding:4px;border-radius:4px">
                                         {grid.iter().map(|p| {
                                             let z = p["z"].as_f64().unwrap_or(0.0);
                                             let intensity = ((z - min_z) / range) * 0.85 + 0.15;
                                             let r = (255.0 * (1.0 - intensity)) as u8;
                                             let g = (255.0 * (0.3 + 0.7 * intensity)) as u8;
                                             let b = (255.0 * (0.1 + 0.2 * intensity)) as u8;
                                             view! { <div style=format!("width:8px;height:8px;background:rgb({r},{g},{b});border-radius:1px") title=format!("{x_var}={:.1}, {y_var}={:.1}, z={:.2}", p["x"].as_f64().unwrap_or(0.0), p["y"].as_f64().unwrap_or(0.0), z)></div> }
                                         }).collect_view()}
                                     </div>
                                 }.into_any()
                             }
                         }
                     }}
                 </div>

                  <div class="panel" style="flex:1;min-width:300px">
                      <div class="panel-title">"Matriz de Regras Ativadas (UC14)"</div>
                      <div style="display:flex;gap:6px;margin-bottom:8px">
                          <button class="btn btn-primary" style="font-size:10px;padding:4px 10px;white-space:nowrap" on:click=move |_| run_matrix()>
                              <i class="ti ti-list"></i>"Calcular Ativacoes (valores medios)"
                          </button>
                      </div>
                      {move || {
                          if matrix_loading.get() { return view! { <div style="font-size:10px;color:var(--text3);padding:8px 0">"Calculando..."</div> }.into_any(); }
                          match matrix_result.get() {
                              None => view! { <div style="font-size:10px;color:var(--text3);padding:8px 0">"Clique em Calcular Ativacoes."</div> }.into_any(),
                              Some(res) => {
                                  let rules = res["rules"].as_array().cloned().unwrap_or_default();
                                  if rules.is_empty() { return view! { <div style="font-size:10px;color:var(--text3)">"Nenhuma regra."</div> }.into_any(); }
                                  let cols = (rules.len() as f64).sqrt().ceil() as usize;
                                  let cell = 20usize;
                                  view! {
                                      <div style="margin-bottom:6px;font-size:9px;color:var(--text3)">
                                          {format!("{} regras ~ grid {}x{}  celula {}px", rules.len(), cols, (rules.len() + cols - 1) / cols, cell)}
                                      </div>
                                      <div style=format!("display:grid;grid-template-columns:repeat({},{}px);gap:2px;padding:6px;border:1px solid var(--border);border-radius:4px;width:fit-content;max-height:400px;overflow-y:auto", cols, cell)>
                                          {rules.iter().map(|r| {
                                              let act = r["activation"].as_f64().unwrap_or(0.0);
                                              let pct = (act * 100.0).round();
                                              let pos = r["position"].as_i64().unwrap_or(0) + 1;
                                              let rule_text = r["rule_text"].as_str().unwrap_or("");
                                              let intensity = (act * 0.85 + 0.15).clamp(0.15, 1.0);
                                              let r_ch = (255.0 * (1.0 - intensity)) as u8;
                                              let g_ch = (255.0 * (0.3 + 0.7 * intensity)) as u8;
                                              let b_ch = (255.0 * (0.1 + 0.2 * intensity)) as u8;
                                              view! {
                                                  <div style=format!("width:{}px;height:{}px;background:rgb({r_ch},{g_ch},{b_ch});border-radius:2px;cursor:pointer", cell, cell)
                                                      title=format!("#{pos}  {pct}%  {rule_text}")></div>
                                              }
                                          }).collect_view()}
                                      </div>
                                      <div style="display:flex;gap:8px;margin-top:6px;font-size:8px;color:var(--text3);align-items:center">
                                          <span>"0%"</span>
                                          <div style="flex:1;height:6px;border-radius:3px;background:linear-gradient(to right,rgb(217,76,76),rgb(76,217,76))"></div>
                                          <span>"100%"</span>
                                      </div>
                                  }.into_any()
                              }
                          }
                      }}
                  </div>
             </div>
         </div>
     }
 }

// ─────────────────────────────────────────────────────────────
// Edit System Page
// ─────────────────────────────────────────────────────────────
#[component]
fn EditSystemPage() -> impl IntoView {
    let query = use_query_map();
    let name = RwSignal::new(String::new());
    let desc = RwSignal::new(String::new());
    let method = RwSignal::new("centroid".to_string());
    let msg = RwSignal::new(String::new());
    let loaded = RwSignal::new(false);

    let system_id = query.get().get("id").map(|s| s.to_string()).unwrap_or_default();

    spawn_async({
        let n = name.clone();
        let d = desc.clone();
        let m = method.clone();
        let l = loaded.clone();
        let sid = system_id.clone();
        async move {
            if !sid.is_empty() {
                if let Some(sys) = get_system(&sid).await {
                    n.set(sys.name);
                    d.set(sys.description.unwrap_or_default());
                    m.set(sys.defuzz_method);
                    l.set(true);
                }
            }
        }
    });

    let sid_for_submit = system_id.clone();

    view! {
        <Topbar breadcrumb="Editar Sistema"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:20px"><div class="section-title">"Editar Sistema Fuzzy"</div></div>
            <div class="panel" style="max-width:500px">
                {move || if !loaded.get() {
                    view! { <div class="loading">"Carregando..."</div> }.into_any()
                } else {
                    view! {
                        <label class="input-label">"Nome *"</label>
                        <input type="text" class="text-input" prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                        <label class="input-label">"Descrição"</label>
                        <input type="text" class="text-input" prop:value=move || desc.get() on:input=move |e| desc.set(event_target_value(&e))/>
                        <label class="input-label">"Método de Defuzzificação"</label>
                        <select class="text-input" prop:value=move || method.get() on:change=move |e| method.set(event_target_value(&e))>
                            <option value="centroid">"Centroide"</option>
                            <option value="bisector">"Bissetor"</option>
                            <option value="mom">"Mean of Maximum"</option>
                            <option value="lom">"Largest of Maximum"</option>
                            <option value="som">"Smallest of Maximum"</option>
                        </select>
                        {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                    }.into_any()
                }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| {
                        let n = name.get();
                        if n.trim().is_empty() { msg.set("Nome obrigatório".into()); return; }
                        let d = if desc.get().is_empty() { None } else { Some(desc.get()) };
                        let m = method.get();
                        let sid = sid_for_submit.clone();
                        let msg2 = msg.clone();
                        spawn_async(async move {
                            match update_system(&sid, &n, d.as_deref(), &m).await {
                                Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href("/").ok()); } }
                                None => msg2.set("Erro ao atualizar sistema".into()),
                            }
                        });
                    }>"Salvar Alterações"</button>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// AddVarPage (RUST 100%)
// ─────────────────────────────────────────────────────────────
#[component]
fn AddVarPage() -> impl IntoView {
    let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
    let sel_sys = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let role = RwSignal::new("antecedent".to_string());
    let msg = RwSignal::new(String::new());

    {
        let sl = systems_list.clone();
        let ss = sel_sys.clone();
        let _ = &ss;
        spawn_async(async move {
            sl.set(list_systems().await);
            #[cfg(target_arch = "wasm32")]
            if let Some(s) = web_sys::window().and_then(|w| w.location().search().ok()) {
                if let Some(id) = s.split("s=").nth(1).and_then(|x| x.split('&').next()) {
                    if !id.is_empty() { ss.set(id.to_string()); }
                }
            }
        });
    }
    let sel_sys_clone = sel_sys.clone();
    let name_clone = name.clone();
    let role_clone = role.clone();
    let msg_clone = msg.clone();

    let submit = move || {
        let sid = sel_sys_clone.get();
        let n = name_clone.get();
        let r = role_clone.get();
        if sid.is_empty() || n.is_empty() { msg_clone.set("Preencha todos os campos".into()); return; }
        let m = msg_clone.clone();
        spawn_async(async move {
            match create_variable(&sid, &n, &r, 0.0, 100.0).await {
                Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href(&format!("/vars?s={sid}")).ok()); } }
                None => m.set("Erro ao criar variável".into()),
            }
        });
    };

    view! {
        <Topbar breadcrumb="Adicionar Variável"/>
        <div class="content">
            <div class="section-header"><div class="section-title">"Nova Variável"</div></div>
            <div class="panel" style="max-width:500px">
                <label class="input-label">"Sistema"</label>
                <select class="text-input" prop:value=move || sel_sys.get()
                    on:change=move |e| sel_sys.set(event_target_value(&e))>
                    <option value="">"— Selecione —"</option>
                    {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                </select>
                <label class="input-label">"Nome"</label>
                <input type="text" class="text-input" prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                <label class="input-label">"Papel"</label>
                <select class="text-input" prop:value=move || role.get() on:change=move |e| role.set(event_target_value(&e))>
                    <option value="antecedent">"Antecedente"</option>
                    <option value="consequent">"Consequente"</option>
                </select>
                {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/vars" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| submit()>"Adicionar"</button>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// EditVarPage
// ─────────────────────────────────────────────────────────────
#[component]
fn EditVarPage() -> impl IntoView {
    let query = use_query_map();
    let name = RwSignal::new(String::new());
    let role = RwSignal::new("antecedent".to_string());
    let universe_min = RwSignal::new("0".to_string());
    let universe_max = RwSignal::new("100".to_string());
    let resolution = RwSignal::new("501".to_string());
    let msg = RwSignal::new(String::new());
    let loaded = RwSignal::new(false);

    let var_id = query.get().get("id").map(|s| s.to_string()).unwrap_or_default();

    spawn_async({
        let n = name.clone();
        let r = role.clone();
        let umin = universe_min.clone();
        let umax = universe_max.clone();
        let res = resolution.clone();
        let l = loaded.clone();
        let vid = var_id.clone();
        async move {
            if !vid.is_empty() {
                if let Some(v) = get_variable(&vid).await {
                    n.set(v.name);
                    r.set(v.role);
                    umin.set(v.universe_min.to_string());
                    umax.set(v.universe_max.to_string());
                    res.set(v.resolution.to_string());
                    l.set(true);
                }
            }
        }
    });

    let system_id_editvar = query.get().get("s").map(|s| s.to_string()).unwrap_or_default();
    let vid_for_submit = var_id.clone();
    let sid_for_redirect = system_id_editvar.clone();

    view! {
        <Topbar breadcrumb="Editar Variável"/>
        <div class="content">
            <div class="section-header"><div class="section-title">"Editar Variável"</div></div>
            <div class="panel" style="max-width:500px">
                {move || if !loaded.get() {
                    view! { <div class="loading">"Carregando..."</div> }.into_any()
                } else {
                    view! {
                        <label class="input-label">"Nome"</label>
                        <input type="text" class="text-input" prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                        <label class="input-label">"Papel"</label>
                        <select class="text-input" prop:value=move || role.get() on:change=move |e| role.set(event_target_value(&e))>
                            <option value="antecedent">"Antecedente"</option>
                            <option value="consequent">"Consequente"</option>
                        </select>
                        <label class="input-label">"Universo Mínimo"</label>
                        <input type="text" class="text-input" prop:value=move || universe_min.get() on:input=move |e| universe_min.set(event_target_value(&e))/>
                        <label class="input-label">"Universo Máximo"</label>
                        <input type="text" class="text-input" prop:value=move || universe_max.get() on:input=move |e| universe_max.set(event_target_value(&e))/>
                        <label class="input-label">"Resolução"</label>
                        <input type="text" class="text-input" prop:value=move || resolution.get() on:input=move |e| resolution.set(event_target_value(&e))/>
                        {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                    }.into_any()
                }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/vars" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| {
                        let n = name.get();
                        if n.trim().is_empty() { msg.set("Nome obrigatório".into()); return; }
                        let role = role.get();
                        let min: f64 = match universe_min.get().parse() { Ok(v) => v, Err(_) => { msg.set("Universo mínimo inválido".into()); return; } };
                        let max: f64 = match universe_max.get().parse() { Ok(v) => v, Err(_) => { msg.set("Universo máximo inválido".into()); return; } };
                        let res: i32 = match resolution.get().parse() { Ok(v) => v, Err(_) => { msg.set("Resolução inválida".into()); return; } };
                        let vid = vid_for_submit.clone();
                        let m = msg.clone();
                        let s = sid_for_redirect.clone();
                        let _ = &s;
                        spawn_async(async move {
                            match update_variable(&vid, &n, &role, min, max, res).await {
                                Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href(&format!("/vars?s={}", s)).ok()); } }
                                None => m.set("Erro ao atualizar variável".into()),
                            }
                        });
                    }>"Salvar"</button>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// AddTermPage (RUST 100%)
// ─────────────────────────────────────────────────────────────
#[component]
fn AddTermPage() -> impl IntoView {
    let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
    let vars_list = RwSignal::new(Vec::<serde_json::Value>::new());
    let sel_sys = RwSignal::new(String::new());
    let sel_var = RwSignal::new(String::new());
    let label = RwSignal::new(String::new());
    let mf_type = RwSignal::new("trimf".to_string());
    let params = RwSignal::new(String::new());
    let msg = RwSignal::new(String::new());

    spawn_async({ let sl = systems_list.clone(); async move { sl.set(list_systems().await); } });

    // load variables when system changes
    let load_vars = {
        let vl = vars_list.clone();
        let ss = sel_sys.clone();
        move || { let s = ss.get(); if !s.is_empty() { let v = vl.clone(); spawn_async(async move { v.set(list_variables(&s).await); }); } }
    };

    let submit = {
        let lbl = label.clone(); let mf = mf_type.clone(); let p = params.clone();
        let sv = sel_var.clone(); let ss = sel_sys.clone(); let m2 = msg.clone();
        let _ = &ss;
        move || {
            let vid = sv.get();
            if vid.is_empty() { m2.set("Selecione uma variável".into()); return; }
            let parsed: Vec<f64> = p.get().split(',').filter_map(|x| x.trim().parse().ok()).collect();
            if parsed.is_empty() { m2.set("Parâmetros inválidos. Ex: 0,10,22".into()); return; }
            let m3 = m2.clone();
            spawn_async(async move {
                match create_term(&vid, &lbl.get(), &mf.get(), parsed).await {
                    Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href(&format!("/vars?s={}", ss.get())).ok()); } }
                    None => m3.set("Erro ao criar termo".into()),
                }
            });
        }
    };

    view! {
        <Topbar breadcrumb="Adicionar Termo"/>
        <div class="content">
            <div class="section-header"><div class="section-title">"Novo Termo Linguístico"</div></div>
            <div class="panel" style="max-width:500px">
                <label class="input-label">"Sistema"</label>
                <select class="text-input" prop:value=move || sel_sys.get()
                    on:change=move |e| { sel_sys.set(event_target_value(&e)); load_vars(); }>
                    <option value="">"— Selecione —"</option>
                    {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                </select>
                <label class="input-label">"Variável"</label>
                <select class="text-input" prop:value=move || sel_var.get() on:change=move |e| sel_var.set(event_target_value(&e))>
                    <option value="">"— Selecione —"</option>
                    {move || vars_list.get().iter().map(|v| {
                        let id = v["id"].as_str().unwrap_or("").to_string();
                        let name = v["name"].as_str().unwrap_or("?").to_string();
                        view! { <option value={id}>{name}</option> }
                    }).collect_view()}
                </select>
                <label class="input-label">"Rótulo"</label>
                <input type="text" class="text-input" placeholder="Ex: Frio" prop:value=move || label.get() on:input=move |e| label.set(event_target_value(&e))/>
                <label class="input-label">"Tipo MF"</label>
                <select class="text-input" prop:value=move || mf_type.get() on:change=move |e| mf_type.set(event_target_value(&e))>
                    <option value="trimf">"trimf [a,b,c]"</option>
                    <option value="trapmf">"trapmf [a,b,c,d]"</option>
                    <option value="gaussmf">"gaussmf [mean,sigma]"</option>
                </select>
                <label class="input-label">"Parâmetros (ex: 0,10,22)"</label>
                <input type="text" class="text-input" placeholder="0, 10, 22" prop:value=move || params.get() on:input=move |e| params.set(event_target_value(&e))/>
                {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/vars" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| submit()>"Adicionar"</button>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// EditTermPage
// ─────────────────────────────────────────────────────────────
#[component]
fn EditTermPage() -> impl IntoView {
    let query = use_query_map();
    let label = RwSignal::new(String::new());
    let mf_type = RwSignal::new("trimf".to_string());
    let params = RwSignal::new(String::new());
    let msg = RwSignal::new(String::new());
    let loaded = RwSignal::new(false);

    let term_id = query.get().get("id").map(|s| s.to_string()).unwrap_or_default();

    spawn_async({
        let lbl = label.clone();
        let mf = mf_type.clone();
        let p = params.clone();
        let l = loaded.clone();
        let tid = term_id.clone();
        async move {
            if !tid.is_empty() {
                if let Some(t) = get_term(&tid).await {
                    lbl.set(t.label);
                    mf.set(t.mf_type);
                    if let Some(arr) = t.params.as_array() {
                        p.set(arr.iter().map(|v| v.as_f64().unwrap_or(0.0).to_string()).collect::<Vec<_>>().join(", "));
                    }
                    l.set(true);
                }
            }
        }
    });

    let sid_for_term = query.get().get("s").map(|s| s.to_string()).unwrap_or_default();

    view! {
        <Topbar breadcrumb="Editar Termo"/>
        <div class="content">
            <div class="section-header"><div class="section-title">"Editar Termo Linguístico"</div></div>
            <div class="panel" style="max-width:500px">
                {move || if !loaded.get() {
                    view! { <div class="loading">"Carregando..."</div> }.into_any()
                } else {
                    view! {
                        <label class="input-label">"Rótulo"</label>
                        <input type="text" class="text-input" prop:value=move || label.get() on:input=move |e| label.set(event_target_value(&e))/>
                        <label class="input-label">"Tipo MF"</label>
                        <select class="text-input" prop:value=move || mf_type.get() on:change=move |e| mf_type.set(event_target_value(&e))>
                            <option value="trimf">"trimf [a,b,c]"</option>
                            <option value="trapmf">"trapmf [a,b,c,d]"</option>
                            <option value="gaussmf">"gaussmf [mean,sigma]"</option>
                        </select>
                        <label class="input-label">"Parâmetros (ex: 0, 10, 22)"</label>
                        <input type="text" class="text-input" prop:value=move || params.get() on:input=move |e| params.set(event_target_value(&e))/>
                        {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                    }.into_any()
                }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/vars" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| {
                        let tid = term_id.clone();
                        let lbl = label.get();
                        if lbl.trim().is_empty() { msg.set("Rótulo obrigatório".into()); return; }
                        let mf = mf_type.get();
                        let parsed: Vec<f64> = params.get().split(',').filter_map(|x| x.trim().parse().ok()).collect();
                        if parsed.is_empty() { msg.set("Parâmetros inválidos. Ex: 0,10,22".into()); return; }
                        let m = msg.clone();
                        let s = sid_for_term.clone();
                        let _ = &s;
                        spawn_async(async move {
                            match update_term(&tid, &lbl, &mf, parsed).await {
                                Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href(&format!("/vars?s={}", s)).ok()); } }
                                None => m.set("Erro ao atualizar termo".into()),
                            }
                        });
                    }>"Salvar"</button>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// AddRulePage
// ─────────────────────────────────────────────────────────────
#[component]
fn AddRulePage() -> impl IntoView {
    let systems_list = RwSignal::new(Vec::<SystemInfo>::new());
    let sel_sys = RwSignal::new(String::new());
    let rule_text = RwSignal::new(String::new());
    let weight = RwSignal::new("1.0".to_string());
    let msg = RwSignal::new(String::new());

    {
        let sl = systems_list.clone();
        let ss = sel_sys.clone();
        let _ = &ss;
        spawn_async(async move {
            sl.set(list_systems().await);
            #[cfg(target_arch = "wasm32")]
            if let Some(s) = web_sys::window().and_then(|w| w.location().search().ok()) {
                if let Some(id) = s.split("s=").nth(1).and_then(|x| x.split('&').next()) {
                    if !id.is_empty() { ss.set(id.to_string()); }
                }
            }
        });
    }

    let submit = move || {
        let sid = sel_sys.get();
        let text = rule_text.get();
        let w: f64 = weight.get().parse().unwrap_or(1.0);
        if sid.is_empty() || text.is_empty() { msg.set("Preencha todos os campos".into()); return; }
        let m = msg.clone();
        spawn_async(async move {
            match create_rule(&sid, &text, w).await {
                Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href(&format!("/rules?s={sid}")).ok()); } }
                None => m.set("Erro ao criar regra".into()),
            }
        });
    };

    view! {
        <Topbar breadcrumb="Adicionar Regra"/>
        <div class="content">
            <div class="section-header"><div class="section-title">"Nova Regra Fuzzy"</div></div>
            <div class="panel" style="max-width:500px">
                <label class="input-label">"Sistema"</label>
                <select class="text-input" prop:value=move || sel_sys.get()
                    on:change=move |e| sel_sys.set(event_target_value(&e))>
                    <option value="">"— Selecione —"</option>
                    {move || systems_list.get().iter().map(|s| view! { <option value={s.id.clone()}>{s.name.clone()}</option> }).collect_view()}
                </select>
                <label class="input-label">"Regra (formato: var É termo E ... ENTÃO var É termo)"</label>
                <input type="text" class="text-input" placeholder="Ex: temperatura É Frio E umidade É Alta ENTÃO conforto É Desconfortável"
                    prop:value=move || rule_text.get() on:input=move |e| rule_text.set(event_target_value(&e))/>
                <label class="input-label">"Peso (0.0 a 1.0)"</label>
                <input type="text" class="text-input" value="1.0" prop:value=move || weight.get() on:input=move |e| weight.set(event_target_value(&e))/>
                {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/rules" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| submit()>"Adicionar"</button>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// EditRulePage
// ─────────────────────────────────────────────────────────────
#[component]
fn EditRulePage() -> impl IntoView {
    let query = use_query_map();
    let rule_text = RwSignal::new(String::new());
    let weight = RwSignal::new("1.0".to_string());
    let msg = RwSignal::new(String::new());
    let loaded = RwSignal::new(false);

    let rule_id = query.get().get("id").map(|s| s.to_string()).unwrap_or_default();

    spawn_async({
        let rt = rule_text.clone();
        let w = weight.clone();
        let l = loaded.clone();
        let rid = rule_id.clone();
        async move {
            if !rid.is_empty() {
                if let Some(r) = get_rule(&rid).await {
                    rt.set(r.rule_text);
                    w.set(r.weight.to_string());
                    l.set(true);
                }
            }
        }
    });

    let sid_for_rule = query.get().get("s").map(|s| s.to_string()).unwrap_or_default();

    view! {
        <Topbar breadcrumb="Editar Regra"/>
        <div class="content">
            <div class="section-header"><div class="section-title">"Editar Regra Fuzzy"</div></div>
            <div class="panel" style="max-width:500px">
                {move || if !loaded.get() {
                    view! { <div class="loading">"Carregando..."</div> }.into_any()
                } else {
                    view! {
                        <label class="input-label">"Regra"</label>
                        <input type="text" class="text-input"
                            placeholder="Ex: temperatura É Frio E umidade É Alta ENTÃO conforto É Desconfortável"
                            prop:value=move || rule_text.get() on:input=move |e| rule_text.set(event_target_value(&e))/>
                        <label class="input-label">"Peso (0.0 a 1.0)"</label>
                        <input type="text" class="text-input" prop:value=move || weight.get() on:input=move |e| weight.set(event_target_value(&e))/>
                        {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                    }.into_any()
                }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/rules" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| {
                        let rid = rule_id.clone();
                        let text = rule_text.get();
                        if text.trim().is_empty() { msg.set("Texto da regra obrigatório".into()); return; }
                        let w: f64 = match weight.get().parse() { Ok(v) => v, Err(_) => { msg.set("Peso inválido".into()); return; } };
                        let m = msg.clone();
                        let s = sid_for_rule.clone();
                        let _ = &s;
                        spawn_async(async move {
                            match update_rule(&rid, &text, w).await {
                                Some(_) => { #[cfg(target_arch = "wasm32")] { _ = web_sys::window().and_then(|w| w.location().set_href(&format!("/rules?s={}", s)).ok()); } }
                                None => m.set("Erro ao atualizar regra".into()),
                            }
                        });
                    }>"Salvar"</button>
                </div>
            </div>
        </div>
    }
}

// ── Import System (UC11) ──
#[component]
fn ImportPage() -> impl IntoView {
    let json_text = RwSignal::new(String::new());
    let msg = RwSignal::new(String::new());
    let do_import = move || {
        let text = json_text.get();
        if text.trim().is_empty() { msg.set("Cole o JSON do sistema".into()); return; }
        let parsed: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => { msg.set(format!("JSON invalido: {e}")); return; }
        };
        let m = msg.clone();
        spawn_async(async move {
            match import_system(&parsed).await {
                Some(_) => {
                    #[cfg(target_arch = "wasm32")]
                    { _ = web_sys::window().and_then(|w| w.location().set_href("/").ok()); }
                }
                None => m.set("Erro ao importar sistema. Verifique o formato.".into()),
            }
        });
    };
    view! {
        <Topbar breadcrumb="Importar Sistema"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px"><div class="section-title">"Importar Sistema (UC11)"</div></div>
            <div class="panel" style="max-width:600px">
                <div style="font-size:11px;color:var(--text3);margin-bottom:12px">
                    "Cole o JSON de um sistema exportado (use o botao de download no Dashboard para exportar)."
                </div>
                <textarea class="text-input" style="min-height:200px;font-family:monospace;font-size:11px;resize:vertical"
                    placeholder="{\n  name: \"Meu Sistema\",\n  variables: [...],\n  rules: [...]\n}"
                    prop:value=move || json_text.get()
                    on:input=move |e| json_text.set(event_target_value(&e))></textarea>
                {move || { let m = msg.get(); if !m.is_empty() { view! { <div style="color:var(--coral);font-size:11px;margin-top:8px">{m}</div> }.into_any() } else { view! {}.into_any() } }}
                <div style="display:flex;gap:10px;margin-top:16px">
                    <a class="btn" href="/" target="_self">"Cancelar"</a>
                    <button class="btn btn-primary" on:click=move |_| do_import()>
                        <i class="ti ti-upload"></i>"Importar Sistema"
                    </button>
                </div>
            </div>
        </div>
    }
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
