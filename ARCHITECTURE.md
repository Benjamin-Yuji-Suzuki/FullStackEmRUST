# ARCHITECTURE.md — FuzzySimulated

> Detalhamento técnico da arquitetura do sistema: modelagem do banco de dados, integrações externas e fluxo de dados.

---

## Modelagem do Banco de Dados

O schema é versionado em `server/migrations/001_schema.sql`. Todas as tabelas usam UUID como chave primária e campos JSONB para armazenar estruturas flexíveis (inputs, outputs, termos fuzzy).

```sql
-- Sistemas fuzzy (UC01)
CREATE TABLE fuzzy_systems (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    defuzz_method TEXT NOT NULL DEFAULT 'centroid',
    status TEXT NOT NULL DEFAULT 'ativo' CHECK (status IN ('ativo','favorito','concluido','desativado')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Variáveis (UC02)
CREATE TABLE fuzzy_variables (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('antecedent', 'consequent')),
    universe_min FLOAT NOT NULL,
    universe_max FLOAT NOT NULL,
    resolution INT NOT NULL DEFAULT 501
);

-- Termos linguísticos (UC02)
CREATE TABLE fuzzy_terms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    variable_id UUID NOT NULL REFERENCES fuzzy_variables(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    mf_type TEXT NOT NULL CHECK (mf_type IN ('trimf', 'trapmf', 'gaussmf')),
    params JSONB NOT NULL
);

-- Regras fuzzy (UC03)
CREATE TABLE fuzzy_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    rule_text TEXT NOT NULL,
    weight FLOAT NOT NULL DEFAULT 1.0,
    position INT NOT NULL DEFAULT 0
);

-- Simulações (UC04, UC06)
CREATE TABLE simulations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    inputs JSONB NOT NULL,
    outputs JSONB NOT NULL,
    weather_data JSONB,
    city TEXT,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Resultados de inferência em lote (UC07)
CREATE TABLE batch_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    source_file TEXT NOT NULL,
    row_index INT NOT NULL,
    inputs JSONB NOT NULL,
    output FLOAT NOT NULL,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Eventos de auditoria (UC16)
CREATE TABLE audit_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID REFERENCES fuzzy_systems(id) ON DELETE SET NULL,
    action_type TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id UUID,
    description TEXT NOT NULL,
    snapshot_before JSONB,
    snapshot_after JSONB,
    redo_stack BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Cenários de simulação (UC12)
CREATE TABLE scenarios (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    inputs JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

```

### Índices

```sql
CREATE INDEX idx_variables_system ON fuzzy_variables(system_id);
CREATE INDEX idx_terms_variable ON fuzzy_terms(variable_id);
CREATE INDEX idx_rules_system ON fuzzy_rules(system_id);
CREATE INDEX idx_simulations_system ON simulations(system_id);
CREATE INDEX idx_simulations_executed ON simulations(executed_at DESC);
CREATE INDEX idx_batch_system ON batch_results(system_id);
CREATE INDEX idx_audit_system ON audit_events(system_id);
CREATE INDEX idx_audit_created ON audit_events(created_at DESC);
CREATE INDEX idx_scenarios_system ON scenarios(system_id);
```

---

## Integração com API Externa — OpenWeather

A [OpenWeather Current Weather API](https://openweathermap.org/current) fornece temperatura (°C) e umidade (%) de qualquer cidade em tempo real (UC05).

```
GET https://api.openweathermap.org/data/2.5/weather?q=Belém&appid={API_KEY}&units=metric
→ { "main": { "temp": 32.4, "humidity": 88 } }
```

**Endpoint:** `GET /api/weather?city=Belém`  
**Chave:** `OPENWEATHER_API_KEY` no `.env`  
**Tratamento de erros:** cidade não encontrada (404), chave inválida (401), timeout (502)  
**TLS:** reqwest com `rustls-tls` (sem dependência de OpenSSL do sistema)

### Seed Data

Na primeira execução, `server/migrations/002_seed.sql` insere automaticamente o sistema **"Conforto Térmico"** (3 variáveis, 9 termos, 9 regras) se o banco estiver vazio. Disponível para testes imediatos no Simulador.

---



## Estrutura dos Repositórios

```
FullStackEmRUST/
├── USE_CASES.md         # 25 casos de uso
├── TEST_CASES.md # 44 casos de teste (30 inline + 16 unit + 64 HTTP + 6 integração)
├── FUZZY_MODEL.md       # Modelos Mamdani + TSK + PSO
├── ARCHITECTURE.md      # Este documento
├── README.md
├── roteiro-apresentacao.md  # Slides de apresentação
├── coverage.sh              # Script de cobertura (cargo-llvm-cov)
└── fuzzysimulated/
    ├── Cargo.toml        # workspace Rust (serial_test, Leptos, Axum, SQLx, reqwest)
    ├── app/              # crate compartilhada Leptos (SSR + CSR)
    │   └── src/
    │       ├── lib.rs         # ~1800 linhas, 17 componentes Leptos
    │       └── server_fns.rs  # API client (gloo-net WASM / reqwest SSR)
    ├── server/            # crate Axum — REST API
    │   ├── Cargo.toml
    │   ├── src/
    │   │   ├── main.rs        # entry point, router, static files
    │   │   ├── engine.rs      # Motor Mamdani (fuzzificação → agregação → defuzz centroide)
    │   │   ├── validation.rs  # Validação de MF (trimf/trapmf/gaussmf) e sistema
    │   │   ├── audit.rs       # Helper de registro de auditoria
    │   │   ├── errors.rs      # AppError (422/404/500/502)
    │   │   ├── models/        # FuzzySystem, Variable, Term, Rule, Simulation
    │   │   ├── routes/        # systems, variables, rules, simulate, weather, audit
    │   │   └── state.rs       # AppState { pool, leptos_options }
│   ├── migrations/
│   │   ├── 001_schema.sql              # Schema: 8 tabelas + índices
│   │   ├── 002_seed.sql                # Seed: sistema Conforto Térmico
    │   │   ├── 004_audit_orphan.sql        # FK audit_events ON DELETE SET NULL
│   │   ├── 005_system_status.sql       # Coluna status
│   │   ├── 006_scenarios.sql           # Tabela scenarios
│   │   ├── 007_seed_risco.sql          # Seed Análise de Risco
│   │   ├── 008_seed_risco_cibernetico.sql # Seed Risco Cibernético
│   │   └── 009_reset_and_seed.sql      # Reset + 4 sistemas seed (JSONB constante)
│   └── tests/
│       ├── all.rs               # 80 testes (16 unit + 64 HTTP)
│       ├── common/mod.rs        # TestApp helper (pool + router)
│       ├── unit/                # 16 unit tests
│       │   ├── mf_validation.rs
│       │   ├── system_validation.rs
    │   └── integration/         # 6 integration tests (ignored, transaction rollback)
    ├── frontend/          # crate WASM — entry point hydrate
    │   └── src/lib.rs
    ├── end2end/           # Playwright E2E (41 testes: CRUD, seed, validação, lifecycle, status, simulação)
    ├── style/main.scss    # Tema escuro Catppuccin
    ├── coverage/          # Relatórios HTML de cobertura
    └── public/            # assets estáticos
```

---

## Visão Geral da Arquitetura

```
Navegador (WASM)
  ├── Leptos (SSR + hydrate)
  │     └── Componentes: Dashboard, Variáveis, Regras, Simulador, etc.
  │
  ├── HTTP (gloo-net via fetch)
  │     ↓
  └── Servidor Axum (porta 3000)
        ├── REST API (/api/*)
        │     ├── systems.rs      → CRUD sistemas
        │     ├── variables.rs    → CRUD variáveis e termos
        │     ├── rules.rs        → CRUD regras
│         ├── simulate.rs     → simulação (Mamdani real), sweep, surface, duplicate, import/export
│         ├── weather.rs      → OpenWeather API
│         ├── audit_routes.rs → auditoria

        │
        ├── PostgreSQL via SQLx
        │     └── migrations, queries compile-checked, JSONB
        │
        └── Static files (ServeDir)
              └── /pkg/ (WASM, JS, CSS gerados pelo wasm-bindgen)
```
