# FuzzySimulated — Inference Platform

Plataforma full-stack **100% Rust** para construção e simulação de sistemas de inferência fuzzy (Mamdani + TSK), com otimização PSO de parâmetros de MF, diagnóstico explicativo e exportação SVG.

**Projeto acadêmico** — CESUPA 02/2026  
**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados · Resolução de Problemas Multivariáveis

---

## Stack

| Camada | Tecnologia |
|---|---|
| **Frontend** | Leptos 0.8 (SSR + WASM hydration) |
| **Backend** | Axum 0.8 (REST API) |
| **Banco** | PostgreSQL via SQLx (queries compile-checked) |
| **Build** | `cargo-leptos` |
| **Testes** | 115 testes (41 unit + 66 HTTP + 8 integration) |

## Arquitetura

```
fuzzysimulated/
├── server/          # Axum API + engine fuzzy (Mamdani/TSK/PSO)
│   ├── src/
│   │   ├── engine.rs      # 599 linhas — membership, parser, Mamdani, TSK, SVG, Diagnóstico, PSO
│   │   ├── math.rs        # Otimização quadrática (Hessiana, gradiente, classificação)
│   │   ├── audit.rs       # Trilha de auditoria com snapshots JSONB
│   │   ├── validation.rs  # Validação de MF, sistema, defuzz method
│   │   └── routes/        # systems, variables, terms, rules, simulate, optimize, audit, weather
│   ├── tests/
│   │   ├── axum_api.rs    # 66 testes HTTP (serializados) + E2E pipeline
│   │   ├── api_test.rs    # 8 testes de integração (transaction rollback)
│   │   └── common/        # TestApp helper
│   └── migrations/        # SQLx migrations (schema + seed + audit + status)
├── app/             # Leptos components + server_fns
│   └── src/
│       ├── lib.rs         # UI reativa (~1800 linhas)
│       └── server_fns.rs  # API client (WASM reqwest/gloo + server reqwest)
├── frontend/        # WASM entry point
└── style/main.scss  # Tema escuro Catppuccin
```

## Telas

| Tela | Rota | Status |
|---|---|---|
| Dashboard (lista sistemas + KPIs) | `/` | ✅ |
| Criar sistema | `/newsys` | ✅ |
| Editar sistema | `/editsys?id=` | ✅ |
| Variáveis & Termos | `/vars?s=` | ✅ |
| Adicionar variável | `/add-var?s=` | ✅ |
| Adicionar termo | `/add-term?id=&v=` | ✅ |
| Editor de Regras | `/rules?s=` | ✅ |
| Simulador (Mamdani / TSK / SVG / Diagnóstico) | `/sim?s=` | ✅ |
| Histórico de simulações | `/hist` | ✅ |
| Superfície de resposta | `/analysis` | ✅ |
| Otimizador (função quadrática + PSO) | `/opt` | ✅ |
| Auditoria (com undo) | `/audit?id=` | ✅ |

## Motor Fuzzy (`server/src/engine.rs` — 599 linhas, 6 funções públicas)

| Função | Descrição | UC |
|---|---|---|
| `evaluate_mamdani()` | Pipeline completo: fuzzificação → agregação min → defuzz centroide | UC06 |
| `evaluate_tsk()` | Takagi-Sugeno-Kang: firing strengths + polinômios → média ponderada | UC18 |
| `generate_diagnostic()` | Explicação detalhada via `ExplainReport` (fuzzificação, ativações, outputs) | UC20 |
| `generate_svg()` | Gera SVG individual por variável usando `var_svg!` macro | UC19 |
| `optimize_with_pso()` | PSO (enxame) para otimizar parâmetros de MF | UC17 |
| `parse_rule_conditions()` | Parser de regras: "SE var é termo E ... ENTÃO var é termo" | — |

### Membership Functions
- `trimf` (triangular, 3 params: a ≤ b ≤ c)
- `trapmf` (trapezoidal, 4 params: a ≤ b ≤ c ≤ d)
- `gaussmf` (gaussiana, 2 params: mean, sigma > 0)

## Comandos

```bash
# Watch mode (porta 3000)
cargo leptos watch

# Unit tests (41, sem DB)
cargo test -p server --lib

# HTTP tests (66, requer DB fuzzysimulated_test)
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test axum_api

# Integration tests (8, requer DB)
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test api_test -- --ignored

# Todos os testes
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server

# Check compilação
cargo check -p server && cargo check -p app && cargo check -p frontend
```

## Testes

| Suite | Qtde | DB | Como rodar |
|---|---|---|---|
| Unit (inline) | 41 | ❌ | `cargo test -p server --lib` |
| HTTP Axum | 66 | ✅ | `cargo test -p server --test axum_api` (serial) |
| Integration | 8 | ✅ | `cargo test -p server --test api_test -- --ignored` |
| **Total** | **115** | | |

Todos os 66 testes HTTP usam `#[serial_test::serial]` para evitar deadlocks do `TRUNCATE CASCADE` concorrente. Inclui teste E2E `test_e2e_full_pipeline` que percorre 22 operações: criar sistema → variáveis → termos → regras → simular Mamdani → diagnóstico → SVG → TSK → batch → sweep → surface → cenários → comparar → duplicar → import/export → status → otimização quadrática → export → PSO → auditoria.

## Funcionalidades

- **Motor Mamdani real** — fuzzificação, agregação min, defuzz centroide discreto
- **TSK** — Takagi-Sugeno-Kang com coeficientes polinomiais por regra
- **PSO** — Particle Swarm Optimization para ajuste de parâmetros MF
- **SVG Export** — gráficos individuais por variável via `var_svg!`
- **Diagnóstico** — explicação da inferência (fuzzificação, ativação, outputs)
- **Estados do sistema:** ativo, favorito (protege deleção), concluído, desativado
- **Auditoria com undo real:** restore completo via snapshots JSONB
- **Otimizador:** Hessiana + gradiente + classificação de ponto crítico
- **Seed demo:** Sistema "Conforto Térmico" com 3 variáveis, 9 termos, 9 regras
