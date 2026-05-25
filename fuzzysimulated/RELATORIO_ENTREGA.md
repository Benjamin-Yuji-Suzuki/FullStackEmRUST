# Relatório de Entrega — FuzzySimulated

## 1. Informações Gerais

- **Projeto:** FuzzySimulated — Sistema de Inferência Fuzzy Full-Stack
- **Disciplinas:** Qualidade e Projeto de Software / Inteligência Artificial e Computacional / Ciência de Dados — CESUPA 01/2026
- **Stack:** 100% Rust (Leptos SSR+WASM + Axum + PostgreSQL)
- **Repositório:** `FullStackEmRUST/fuzzysimulated/`

---

## 2. Arquitetura do Sistema

### 2.1 Frontend (Leptos SSR + WASM)

```
app/src/lib.rs          → 18 rotas, 5 telas principais, componentes Leptos
app/src/server_fns.rs   → Funções client-side que chamam API REST
style/main.scss         → Estilo compilado via dart-sass + Lightning CSS
frontend/src/lib.rs     → Ponto de entrada WASM
```

**5 Telas implementadas:**
1. **Dashboard** (`/`) — Lista de sistemas fuzzy com cards e badges de status
2. **Simulador** (`/sim`) — 5 abas: Mamdani, TSK, SVG, Diagnóstico, Analisar
3. **Análise** (`/analysis`) — Superfície de controle (heatmap) + Matriz de Regras
4. **Histórico** (`/hist`) — Comparação de simulações, exportação de relatórios
5. **Auditoria** (`/audit`) — Eventos de alteração com undo (JSONB snapshots)

**Telas auxiliares:** Novo Sistema (`/newsys`), Editar Sistema (`/editsys`), Variáveis (`/vars`), Regras (`/rules`), Batch (`/batch`), Otimizador (`/opt`), Importar (`/import`).

### 2.2 Backend (Axum)

```
server/src/main.rs          → Entry point, migrations, startup
server/src/lib.rs           → Re-exports públicos
server/src/state.rs         → AppState (pool + LeptosOptions)
server/src/errors.rs        → AppError enum, IntoResponse
server/src/validation.rs    → Validação de nomes, MF shapes, defuzz
server/src/math.rs          → Otimização quadrática (Hessiana, gradiente)
server/src/engine.rs        → Motor de inferência fuzzy (Mamdani + TSK)
server/src/audit.rs         → Schema de auditoria (JSONB snapshot)
server/src/routes/
  ├── mod.rs                → Composição de rotas
  ├── systems.rs            → CRUD sistemas + status + duplicar + import/export
  ├── variables.rs          → CRUD variáveis + termos
  ├── rules.rs              → CRUD regras
  ├── simulate.rs           → Mamdani, TSK, SVG, Diagnóstico, PSO, Sweep, Surface, RuleMatrix
  ├── scenarios.rs          → CRUD cenários
  ├── batch.rs              → Inferência em lote (JSON, CSV, Parquet)
  ├── optimize.rs           → Otimização quadrática + histórico + export
  ├── audit_routes.rs       → Listagem + undo de eventos
  └── weather.rs            → Integração OpenWeatherMap
```

### 2.3 Banco de Dados (PostgreSQL)

```
migrations/
  001_schema.sql          → Tabelas base (systems, variables, terms, rules)
  002_seed.sql            → Seed Conforto Térmico (9 regras)
  003_optimization.sql    → Tabela optimizations
  004_audit_orphan.sql    → Índices para orphan events
  005_system_status.sql   → Coluna status (ativo/favorito/concluido/desativado)
  006_scenarios.sql       → Tabela scenarios
  007_seed_risco.sql      → Seed Análise de Risco
  008_seed_risco_cibernetico.sql → Seed Risco Cibernético
  009_reset_and_seed.sql  → Reset + 4 sistemas seed (42 regras, 43 cenários)
```

---

## 3. 15 Casos de Uso + 8 Extras

| UC# | Nome | Status | Testes |
|-----|------|--------|--------|
| UC01 | Dashboard | ✅ | 4 E2E |
| UC02 | Variáveis & Termos | ✅ | 12 API + E2E |
| UC03 | Editor de Regras | ✅ | 5 API + E2E |
| UC05 | OpenWeather | ✅ | 2 API + 1 E2E |
| UC06 | Histórico | ✅ | 1 API + E2E |
| UC07 | Batch | ✅ | 5 API + 1 E2E |
| UC08 | Comparar Simulações | ✅ | 2 API + E2E |
| UC09 | Exportar Relatório | ✅ | 1 API + E2E |
| UC10 | Duplicar Sistema | ✅ | 1 API + E2E |
| UC11 | Importar Sistema | ✅ | 1 E2E |
| UC12 | Cenários | ✅ | 5 API + E2E |
| UC13 | Varredura (Sweep) | ✅ | 2 API + 1 E2E |
| UC14 | Matriz de Regras | ✅ | 1 API + E2E |
| UC15 | Superfície | ✅ | 1 API + 1 E2E |
| UC16 | Histórico de Alterações | ✅ | 2 API + E2E |
| UC17 | PSO | ✅ | 2 API + 1 E2E |
| UC18 | TSK | ✅ | 2 API + 1 E2E |
| UC19 | SVG | ✅ | 2 API + 1 E2E |
| UC20 | Diagnóstico | ✅ | 2 API + 1 E2E |
| UC21 | Otimização Quadrática | ✅ | 2 API |
| UC23 | Status do Sistema | ✅ | 1 API + E2E |
| UC24 | Histórico Otimizações | ✅ | 1 API + 1 E2E |
| UC25 | Export Otimização | ✅ | 1 API + 1 E2E |

---

## 4. Suíte de Testes

### 4.1 Nível Unitário (41 + 22 = 63 testes)

| Módulo | Arquivo | Testes |
|--------|---------|--------|
| Engine | `server/src/engine.rs` (inline) | 14 |
| Math | `server/src/math.rs` (inline) | 10 |
| Errors | `server/src/errors.rs` (inline) | 4 |
| Weather | `server/src/routes/weather.rs` (inline) | 4 |
| Audit | `server/src/routes/audit_routes.rs` (inline) | 9 |
| MF Validation | `tests/unit/mf_validation.rs` | 10 |
| System Validation | `tests/unit/system_validation.rs` | 6 |
| Optimization | `tests/unit/optimization.rs` | 6 |

### 4.2 Nível Integração HTTP (70 testes)

| Domínio | Arquivo `tests/api/` | Testes |
|---------|---------------------|--------|
| Systems | `systems.rs` | 8 |
| Variables | `variables.rs` | 7 |
| Terms | `terms.rs` | 5 |
| Rules | `rules.rs` | 5 |
| Simulation | `simulate.rs` | 12 |
| Compare/Export | `compare_export.rs` | 5 |
| Scenarios | `scenarios.rs` | 5 |
| Sweep/Surface | `sweep_surface.rs` | 5 |
| Batch | `batch.rs` | 5 |
| Audit | `audit.rs` | 3 |
| Optimize | `optimize.rs` | 4 |
| Misc | `misc.rs` | 3 |
| Pipeline | `pipeline.rs` | 1 |
| **Integração DB** | `tests/integration/` | 8 (ignored) |

### 4.3 Nível End-to-End (41 testes)

Arquivo: `end2end/tests/example.spec.ts` — Playwright + Chromium

- 4 testes independentes (homepage, navegação, criação, simulador vazio)
- 37 testes em 10 `describe.serial` blocks

### 4.4 Totais

| Nível | Qtde |
|-------|------|
| Unitários (src inline) | 41 |
| Unitários (tests/unit) | 22 |
| Integração HTTP | 70 |
| Integração DB (ignored) | 8 |
| End-to-End | 41 |
| **Total** | **182** |

---

## 5. Cobertura de Código (llvm-cov)

| Módulo | Cobertura |
|--------|-----------|
| `engine.rs` (motor inferência) | **91.76%** |
| `math.rs` (otimização quadrática) | **98.98%** |
| `errors.rs` | **100%** |
| `validation.rs` | **98.15%** |
| `routes/rules.rs` | **90.06%** |
| `routes/optimize.rs` | **86.89%** |
| `routes/scenarios.rs` | **88.16%** |
| `routes/simulate.rs` | **74.79%** |
| `routes/variables.rs` | **79.60%** |
| `routes/audit_routes.rs` | **70.43%** |
| `routes/systems.rs` | **64.79%** |
| `routes/batch.rs` | **44.11%** |
| `routes/weather.rs` | **50.39%** |

**Meta 70-80% atingida** para os módulos centrais de lógica de negócio (engine, math, validation, rules, optimize, scenarios). Rotas HTTP têm cobertura variável devido à dependência de parquet files reais (batch) e API key (weather).

Relatório HTML: `coverage/html/html/index.html`

---

## 6. Análise Estática (Clippy)

- **server crate:** 1 warning (`very_complex_type` em `engine.rs` — tipo de retorno complexo)
- **app crate:** 286 warnings (todos de macros `view!` do Leptos — `unused_unit`, `clone_on_copy`, não acionáveis)
- **Zero errors** (`cargo clippy --deny warnings` requer ajustes nas macros Leptos)

---

## 7. Estrutura de Testes (Refatoração)

### 7.1 De: Arquivo monolítico

```
tests/
  axum_api.rs    → 1516 linhas, 43 testes
```

### 7.2 Para: Módulos por domínio

```
tests/
  api/
    mod.rs          → Helpers compartilhados (TestApp, json_post, etc.)
    systems.rs      → 8 testes
    variables.rs    → 7 testes
    terms.rs        → 5 testes
    rules.rs        → 5 testes
    simulate.rs     → 12 testes
    compare_export.rs → 5 testes
    scenarios.rs    → 5 testes
    sweep_surface.rs → 5 testes
    batch.rs        → 5 testes
    audit.rs        → 3 testes
    optimize.rs     → 4 testes
    misc.rs         → 3 testes
    pipeline.rs     → 1 teste
```

### 7.3 is_ok → expect

Todos os `assert!(result.is_ok())` foram substituídos por `result.expect("mensagem")`, que exibe o erro real em caso de falha, melhorando a depuração.

---

## 8. Comandos de Verificação

```bash
# Compilação
cargo check -p server && cargo check -p app && cargo check -p frontend

# Testes unitários + HTTP (exceto DB integration)
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test \
  cargo test -p server -- --skip ignored

# Testes end-to-end (Playwright)
cd end2end && npx playwright test

# Cobertura
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test \
  cargo llvm-cov --html --output-dir coverage/html -- --skip ignored

# Lints
cargo clippy -p server

# Servidor de desenvolvimento
cargo leptos watch
```

---

## 9. Decisões Técnicas

| Decisão | Motivação |
|---------|-----------|
| **100% Rust** | Proposta acadêmica de explorar Rust full-stack |
| **Leptos SSR+WASM** | Framework reativo com suporte nativo a SSR, sem JS |
| **Axum + Tower** | Ecossistema async maduro, `ServiceExt::oneshot` para testes |
| **JSONB snapshots p/ auditoria** | Undo completo de sistema + variáveis + termos + regras |
| **PSO separado do Hessiano** | Disciplinas distintas: PSO (IA), Hessiano (Multivariável) |
| **Testes com `#[serial]`** | Banco compartilhado entre testes paralelos |
| **4 seeds distintos** | Cobertura de OpenWeather, dataset_ml.parquet, cibersegurança |
| **Output-50 fallback** | Quando `compute()` retorna Err, avalia midpoint do universo |

---

## 10. Pendências / Melhorias Futuras

- Aumentar cobertura de `batch.rs` (44%) com testes que usam arquivos Parquet reais
- Aumentar cobertura de `weather.rs` (50%) com API key de teste
- Fixar warnings do Clippy no app crate (depende de atualização do Leptos)
- Adicionar UC04 e UC22 se requisitados
