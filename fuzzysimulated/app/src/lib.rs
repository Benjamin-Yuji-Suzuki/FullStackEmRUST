use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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
                        <Route path=StaticSegment("")      view=Dashboard/>
                        <Route path=StaticSegment("vars")  view=Variaveis/>
                        <Route path=StaticSegment("rules") view=Regras/>
                        <Route path=StaticSegment("sim")   view=Simulador/>
                        <Route path=StaticSegment("hist")  view=Historico/>
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
            </nav>

            <div class="sidebar-footer">
                <span class="sprint-badge">"⬡ Sprint 1 — Estrutura"</span>
                <div class="sidebar-course">"Disciplina QPS · CESUPA"</div>
            </div>
        </aside>
    }
}

// ─────────────────────────────────────────────────────────────
// Topbar helper
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
                <button class="btn btn-primary">
                    <i class="ti ti-plus"></i>
                    "Novo Sistema"
                </button>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Dashboard
// ─────────────────────────────────────────────────────────────
#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <Topbar breadcrumb="Dashboard"/>
        <div class="content">

            // ── timeline sprints
            <div class="sprint-timeline">
                <div class="sprint-block done">
                    <div class="sprint-num">"Sprint 1"</div>
                    <div class="sprint-name">"Estrutura"</div>
                    <div class="sprint-date">"12 mai"</div>
                    <div class="sprint-status dot-green">"✓ Entregue"</div>
                </div>
                <div class="sprint-block active-sprint">
                    <div class="sprint-num">"Sprint 2"</div>
                    <div class="sprint-name">"CRUDs + API"</div>
                    <div class="sprint-date">"19 mai"</div>
                    <div class="sprint-status dot-amber">"◉ Em progresso"</div>
                </div>
                <div class="sprint-block">
                    <div class="sprint-num">"Sprint 3"</div>
                    <div class="sprint-name">"Testes + Deploy"</div>
                    <div class="sprint-date">"26 mai"</div>
                    <div class="sprint-status dot-gray">"○ Pendente"</div>
                </div>
            </div>

            // ── KPIs
            <div class="kpi-grid">
                <KpiCard label="Sistemas"   value="3"  unit="cadastrados"/>
                <KpiCard label="Variáveis"  value="14" unit="antec. + conseq."/>
                <KpiCard label="Regras"     value="27" unit="mapeadas"/>
                <KpiCard label="Simulações" value="81" unit="executadas"/>
            </div>

            // ── lista de sistemas
            <div class="section-header">
                <div class="section-title">"Sistemas Fuzzy"</div>
                <button class="btn btn-primary" style="font-size:10px;padding:5px 12px">
                    <i class="ti ti-plus"></i>"Criar Sistema"
                </button>
            </div>

            <div class="systems-grid">
                <SystemCard
                    name="Conforto Térmico Urbano"
                    desc="Classifica o conforto a partir de temperatura e umidade."
                    tag_class="tag-green"
                    tag_text="Ativo"
                    meta="centroide · 9 regras · 4 variáveis"
                    extra="OpenWeather integrado"
                />
                <SystemCard
                    name="Risco de Queimadas"
                    desc="Avalia risco a partir de umidade do ar e temperatura."
                    tag_class="tag-coral"
                    tag_text="Rascunho"
                    meta="bissetor · 12 regras · 6 variáveis"
                    extra="Sem integração"
                />
                <SystemCard
                    name="Qualidade do Ar"
                    desc="Índice de qualidade baseado em PM2.5 e CO₂."
                    tag_class="tag-teal"
                    tag_text="Completo"
                    meta="MOM · 6 regras · 4 variáveis"
                    extra="81 simulações"
                />
                <div class="system-card system-card-dashed">
                    <i class="ti ti-plus" style="font-size:20px"></i>
                    "Novo sistema fuzzy"
                </div>
            </div>
        </div>
    }
}

#[component]
fn KpiCard(label: &'static str, value: &'static str, unit: &'static str) -> impl IntoView {
    view! {
        <div class="kpi-card">
            <div class="kpi-label">{label}</div>
            <div class="kpi-value">{value}</div>
            <div class="kpi-unit">{unit}</div>
        </div>
    }
}

#[component]
fn SystemCard(
    name: &'static str,
    desc: &'static str,
    tag_class: &'static str,
    tag_text: &'static str,
    meta: &'static str,
    extra: &'static str,
) -> impl IntoView {
    view! {
        <div class="system-card">
            <div class="system-card-top">
                <div>
                    <div class="system-name">{name}</div>
                    <div class="system-desc">{desc}</div>
                </div>
                <span class={format!("tag {tag_class}")}>{tag_text}</span>
            </div>
            <div style="font-size:10px;color:var(--text3)">
                "Defuzz: " <span style="color:var(--amber)">{meta}</span>
            </div>
            <div class="system-meta">
                <span>{extra}</span>
                <div class="system-actions">
                    <button class="icon-btn"><i class="ti ti-edit"></i></button>
                    <button class="icon-btn"><i class="ti ti-player-play"></i></button>
                    <button class="icon-btn"><i class="ti ti-trash"></i></button>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Variáveis & Termos
// ─────────────────────────────────────────────────────────────
#[component]
fn Variaveis() -> impl IntoView {
    let selected = RwSignal::new("Temperatura");
    let selected_term = RwSignal::new("Frio");

    view! {
        <Topbar breadcrumb="Variáveis & Termos"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px">
                <div>
                    <div class="section-title" style="margin-bottom:3px">"Variáveis & Termos Linguísticos"</div>
                    <div style="font-size:11px;color:var(--text3)">
                        "Sistema: " <span style="color:var(--amber)">"Conforto Térmico Urbano"</span>
                    </div>
                </div>
                <button class="btn btn-primary" style="font-size:10px;padding:5px 12px">
                    <i class="ti ti-plus"></i>"Variável"
                </button>
            </div>

            <div class="var-layout">
                // ── painel lateral de variáveis
                <div class="var-sidebar">
                    <div class="var-group-label">"Antecedentes"</div>
                    <VarItem name="Temperatura" color="var(--amber)" selected=selected/>
                    <VarItem name="Umidade"     color="var(--teal)"  selected=selected/>

                    <div class="var-group-label">"Consequentes"</div>
                    <VarItem name="Conforto"   color="var(--coral)" selected=selected/>
                    <VarItem name="Índice UV"  color="var(--green)" selected=selected/>

                    <div style="margin-top:16px;border-top:1px solid var(--border);padding-top:12px">
                        <button class="btn" style="width:100%;font-size:10px">
                            <i class="ti ti-plus"></i>"Adicionar variável"
                        </button>
                    </div>
                </div>

                // ── painel de edição
                <div class="var-panel">
                    <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:16px">
                        <div>
                            <div style="font-family:var(--display);font-size:15px;font-weight:700;color:var(--text)">
                                {move || selected.get()}
                            </div>
                            <div style="font-size:10px;color:var(--text3);margin-top:2px">
                                "antecedente · universo [0, 50] · resolução 501"
                            </div>
                        </div>
                        <span class="tag tag-amber">"antecedente"</span>
                    </div>

                    // ── gráfico MF
                    <div class="mf-canvas">
                        <svg viewBox="0 0 580 140" xmlns="http://www.w3.org/2000/svg">
                            <line x1="30" y1="110" x2="560" y2="110" stroke="#333" stroke-width="1"/>
                            <line x1="30" y1="20"  x2="30"  y2="115" stroke="#333" stroke-width="1"/>
                            // Frio
                            <polyline points="30,110 100,110 170,30 240,110" fill="none" stroke="#378ADD" stroke-width="2"/>
                            <text x="140" y="26" fill="#378ADD" font-size="10" font-family="monospace">"Frio"</text>
                            // Agradável
                            <polyline points="160,110 240,30 320,30 400,110" fill="none" stroke="#EF9F27" stroke-width="2"/>
                            <text x="248" y="26" fill="#EF9F27" font-size="10" font-family="monospace">"Agradável"</text>
                            // Quente
                            <polyline points="320,110 400,30 500,30 560,110" fill="none" stroke="#D85A30" stroke-width="2"/>
                            <text x="410" y="26" fill="#D85A30" font-size="10" font-family="monospace">"Quente"</text>
                            // eixo x labels
                            <text x="28"  y="128" fill="#666" font-size="9" font-family="monospace">"0"</text>
                            <text x="265" y="128" fill="#666" font-size="9" font-family="monospace">"25"</text>
                            <text x="540" y="128" fill="#666" font-size="9" font-family="monospace">"50°C"</text>
                        </svg>
                    </div>

                    // ── termos
                    <div class="section-title" style="margin-bottom:8px;font-size:11px">"Termos linguísticos"</div>
                    <div class="term-chips">
                        <TermChip label="Frio"      suffix="trimf"  selected=selected_term/>
                        <TermChip label="Agradável" suffix="trapmf" selected=selected_term/>
                        <TermChip label="Quente"    suffix="trimf"  selected=selected_term/>
                        <button class="term-chip" style="border-style:dashed;color:var(--text3)">
                            "+ Termo"
                        </button>
                    </div>

                    // ── parâmetros
                    <div class="params-box">
                        <div class="params-label">
                            "Parâmetros — " {move || selected_term.get()} " (trimf)"
                        </div>
                        <div class="params-grid">
                            <div><div class="param-key">"A"</div>       <div class="param-val">"0"</div></div>
                            <div><div class="param-key">"B (pico)"</div><div class="param-val">"10"</div></div>
                            <div><div class="param-key">"C"</div>       <div class="param-val">"22"</div></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn VarItem(
    name: &'static str,
    color: &'static str,
    selected: RwSignal<&'static str>,
) -> impl IntoView {
    view! {
        <div
            class=move || if selected.get() == name { "var-item active" } else { "var-item" }
            on:click=move |_| selected.set(name)
        >
            <span class="var-dot" style=format!("background:{color}")></span>
            {name}
        </div>
    }
}

#[component]
fn TermChip(
    label: &'static str,
    suffix: &'static str,
    selected: RwSignal<&'static str>,
) -> impl IntoView {
    view! {
        <button
            class=move || if selected.get() == label { "term-chip active" } else { "term-chip" }
            on:click=move |_| selected.set(label)
        >
            {label}" "
            <span style="font-size:9px;opacity:0.6">"["</span>
            <span style="font-size:9px;opacity:0.6">{suffix}</span>
            <span style="font-size:9px;opacity:0.6">"]"</span>
        </button>
    }
}

// ─────────────────────────────────────────────────────────────
// Editor de Regras
// ─────────────────────────────────────────────────────────────
#[component]
fn Regras() -> impl IntoView {
    view! {
        <Topbar breadcrumb="Editor de Regras"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px">
                <div>
                    <div class="section-title" style="margin-bottom:3px">"Editor de Regras Fuzzy"</div>
                    <div style="font-size:11px;color:var(--text3)">
                        "Sistema: " <span style="color:var(--amber)">"Conforto Térmico Urbano"</span>
                        " · 9 regras"
                    </div>
                </div>
                <button class="btn btn-primary" style="font-size:10px;padding:5px 12px">
                    <i class="ti ti-plus"></i>"Regra"
                </button>
            </div>

            <div class="rule-hint">
                <span style="color:var(--text3)">"Formato: "</span>
                <span class="rule-kw">"SE "</span>
                "<antecedente> "
                <span class="rule-kw">"É "</span>
                "<termo> "
                <span class="rule-kw">"E "</span>
                "... "
                <span class="rule-kw">"ENTÃO "</span>
                "<consequente> "
                <span class="rule-kw">"É "</span>
                "<termo> "
                <span style="color:var(--text3)">"[peso]"</span>
            </div>

            <RuleRow n=1  ante="Temperatura" ate="Frio"       cons="Conforto" ct="Desconfortável"      w="1.0"/>
            <RuleRow n=2  ante="Temperatura" ate="Agradável"  cons="Conforto" ct="Confortável"          w="1.0"/>
            <RuleRow n=3  ante="Temperatura" ate="Quente"     cons="Conforto" ct="Muito Desconfortável" w="0.9"/>
            <RuleRow n=4  ante="Temperatura" ate="Quente"     cons="Conforto" ct="Desconfortável"       w="0.8"/>
            <RuleRow n=5  ante="Temperatura" ate="Frio"       cons="Conforto" ct="Neutro"               w="0.7"/>

            // ── construtor visual
            <div class="rule-builder">
                <div class="section-title" style="font-size:11px">"Construtora visual de regra"</div>
                <div class="rule-builder-grid">
                    <select>
                        <option>"Temperatura"</option>
                        <option>"Umidade"</option>
                    </select>
                    <span class="rule-kw-label">"É"</span>
                    <select>
                        <option>"Frio"</option>
                        <option>"Agradável"</option>
                        <option>"Quente"</option>
                    </select>
                    <span class="rule-arrow">"→"</span>
                    <select>
                        <option>"Confortável"</option>
                        <option>"Desconfortável"</option>
                        <option>"Neutro"</option>
                    </select>
                </div>
                <button class="btn btn-primary" style="margin-top:12px;font-size:10px">
                    "Adicionar regra"
                </button>
            </div>
        </div>
    }
}

#[component]
fn RuleRow(
    n: u8,
    ante: &'static str,
    ate: &'static str,
    cons: &'static str,
    ct: &'static str,
    w: &'static str,
) -> impl IntoView {
    view! {
        <div class="rule-row">
            <div class="rule-num">{format!("{n:02}")}</div>
            <div class="rule-text">
                <span class="rule-kw">"SE "</span>
                <span class="rule-var">{ante}</span>
                <span class="rule-kw">" É "</span>
                <span class="rule-term">{ate}</span>
                <span class="rule-kw">" E "</span>
                <span class="rule-var">"Umidade"</span>
                <span class="rule-kw">" É "</span>
                <span class="rule-term">"Alta"</span>
                <span class="rule-kw">" ENTÃO "</span>
                <span class="rule-var">{cons}</span>
                <span class="rule-kw">" É "</span>
                <span class="rule-term">{ct}</span>
            </div>
            <div class="rule-weight">{format!("w={w}")}</div>
            <div class="system-actions">
                <button class="icon-btn"><i class="ti ti-edit"></i></button>
                <button class="icon-btn"><i class="ti ti-trash"></i></button>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Simulador
// ─────────────────────────────────────────────────────────────
#[component]
fn Simulador() -> impl IntoView {
    let temp = RwSignal::new(32_i32);
    let hum = RwSignal::new(88_i32);

    // inferência simplificada: disconforto = temp*0.6 + hum*0.4 (escala 0-100)
    let output = move || {
        let t = temp.get() as f64 / 50.0;
        let h = hum.get() as f64 / 100.0;
        (t * 0.6 + h * 0.4) * 100.0
    };

    let class_label = move || {
        let v = output();
        if v < 30.0 {
            ("Confortável", "var(--green)")
        } else if v < 55.0 {
            ("Neutro", "var(--amber)")
        } else if v < 75.0 {
            ("Desconfortável", "var(--coral)")
        } else {
            ("Muito Desconfortável", "var(--red)")
        }
    };

    view! {
        <Topbar breadcrumb="Simulador"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px">
                <div>
                    <div class="section-title" style="margin-bottom:3px">"Simulador Mamdani"</div>
                    <div style="font-size:11px;color:var(--text3)">
                        "Sistema: " <span style="color:var(--amber)">"Conforto Térmico Urbano"</span>
                    </div>
                </div>
            </div>

            <div class="sim-layout">
                // ── coluna esquerda: entradas
                <div>
                    <div class="panel">
                        <div class="panel-title">"Entradas climáticas"</div>

                        <div class="weather-card">
                            <div class="weather-icon">"🌦"</div>
                            <div>
                                <div class="weather-city">"Belém, PA — Brasil"</div>
                                <div class="weather-vals">
                                    "Temp: " <span>"32.4°C"</span>
                                    " · Umidade: " <span>"88%"</span>
                                    " · via OpenWeather"
                                </div>
                            </div>
                            <button class="btn" style="margin-left:auto;font-size:10px;padding:5px 10px">
                                <i class="ti ti-refresh"></i>
                            </button>
                        </div>

                        // slider temperatura
                        <div class="input-group">
                            <label class="input-label">"Temperatura"</label>
                            <div class="input-row">
                                <input
                                    type="range" class="range-input"
                                    min="0" max="50"
                                    prop:value=move || temp.get()
                                    on:input=move |e| {
                                        let v = event_target_value(&e).parse::<i32>().unwrap_or(25);
                                        temp.set(v);
                                    }
                                />
                                <div class="range-val">{move || format!("{}°C", temp.get())}</div>
                            </div>
                        </div>

                        // slider umidade
                        <div class="input-group">
                            <label class="input-label">"Umidade Relativa"</label>
                            <div class="input-row">
                                <input
                                    type="range" class="range-input"
                                    min="0" max="100"
                                    prop:value=move || hum.get()
                                    on:input=move |e| {
                                        let v = event_target_value(&e).parse::<i32>().unwrap_or(50);
                                        hum.set(v);
                                    }
                                />
                                <div class="range-val">{move || format!("{}%", hum.get())}</div>
                            </div>
                        </div>

                        <button class="btn btn-primary" style="width:100%;margin-top:8px">
                            <i class="ti ti-player-play"></i>
                            "Executar Simulação"
                        </button>
                    </div>
                </div>

                // ── coluna direita: resultado
                <div>
                    <div class="panel">
                        <div class="panel-title">"Resultado da Inferência"</div>

                        <div class="output-display">
                            <div class="output-val">
                                {move || format!("{:.1}", output())}
                            </div>
                            <div class="output-label">"Índice de Conforto · Defuzz: centroide"</div>
                        </div>

                        <div class="output-stats" style="margin-top:12px">
                            <div class="stat-box">
                                <div class="stat-key">"Classificação"</div>
                                <div class="stat-val" style=move || format!("color:{}", class_label().1)>
                                    {move || class_label().0}
                                </div>
                            </div>
                            <div class="stat-box">
                                <div class="stat-key">"Regras ativas"</div>
                                <div class="stat-val" style="color:var(--amber)">"3 / 9"</div>
                            </div>
                            <div class="stat-box">
                                <div class="stat-key">"Latência"</div>
                                <div class="stat-val" style="color:var(--green)">"1.2 ms"</div>
                            </div>
                        </div>
                    </div>

                    <div class="panel">
                        <div class="panel-title">"Pipeline de Inferência"</div>
                        <div class="pipeline-step">
                            <span class="pipeline-num" style="color:var(--green)">"①"</span>
                            "Fuzzificação das entradas"
                        </div>
                        <div class="pipeline-step">
                            <span class="pipeline-num" style="color:var(--green)">"②"</span>
                            "Avaliação das regras (min-operador)"
                        </div>
                        <div class="pipeline-step">
                            <span class="pipeline-num" style="color:var(--amber)">"③"</span>
                            "Agregação das saídas (max-operador)"
                        </div>
                        <div class="pipeline-step">
                            <span class="pipeline-num" style="color:var(--amber)">"④"</span>
                            "Defuzzificação — centroide → "
                            <span style="color:var(--amber);font-weight:700">
                                {move || format!("{:.1}", output())}
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────
// Histórico
// ─────────────────────────────────────────────────────────────
#[component]
fn Historico() -> impl IntoView {
    view! {
        <Topbar breadcrumb="Histórico"/>
        <div class="content">
            <div class="section-header" style="margin-bottom:16px">
                <div class="section-title">"Histórico de Simulações"</div>
                <button class="btn" style="font-size:10px;padding:5px 12px">
                    <i class="ti ti-download"></i>"Exportar CSV"
                </button>
            </div>

            <div class="hist-wrap">
                <table class="hist-table">
                    <thead>
                        <tr>
                            <th>"Sistema"</th>
                            <th>"Entradas"</th>
                            <th>"Saída"</th>
                            <th>"Cidade"</th>
                            <th>"Executado em"</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        <HistRow
                            tag_class="tag-green" tag="Conforto Térmico"
                            inputs="32.4°C / 88%"
                            val="17.3" val_class="hist-val-mid" classification="Desc."
                            cidade="Belém, PA"
                            date="12 mai 15:32"
                        />
                        <HistRow
                            tag_class="tag-teal" tag="Qualidade do Ar"
                            inputs="PM2.5: 45 / CO₂: 410"
                            val="72.1" val_class="hist-val-good" classification="Boa"
                            cidade="—"
                            date="12 mai 14:10"
                        />
                        <HistRow
                            tag_class="tag-coral" tag="Risco Queimada"
                            inputs="38.1°C / 22%"
                            val="91.4" val_class="hist-val-bad" classification="Alto"
                            cidade="Santarém, PA"
                            date="11 mai 09:55"
                        />
                        <HistRow
                            tag_class="tag-green" tag="Conforto Térmico"
                            inputs="26.0°C / 60%"
                            val="62.8" val_class="hist-val-good" classification="Conf."
                            cidade="Marabá, PA"
                            date="10 mai 18:22"
                        />
                        <HistRow
                            tag_class="tag-teal" tag="Qualidade do Ar"
                            inputs="PM2.5: 120 / CO₂: 680"
                            val="18.5" val_class="hist-val-bad" classification="Ruim"
                            cidade="—"
                            date="09 mai 11:00"
                        />
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn HistRow(
    tag_class: &'static str,
    tag: &'static str,
    inputs: &'static str,
    val: &'static str,
    val_class: &'static str,
    classification: &'static str,
    cidade: &'static str,
    date: &'static str,
) -> impl IntoView {
    view! {
        <tr>
            <td><span class={format!("tag {tag_class}")} style="font-size:9px">{tag}</span></td>
            <td>{inputs}</td>
            <td>
                <span class={val_class}>{val}</span>
                " " <span style="color:var(--text3);font-size:10px">"("{classification}")"</span>
            </td>
            <td>{cidade}</td>
            <td>{date}</td>
            <td><button class="icon-btn"><i class="ti ti-eye"></i></button></td>
        </tr>
    }
}

// ─────────────────────────────────────────────────────────────
// 404
// ─────────────────────────────────────────────────────────────
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
