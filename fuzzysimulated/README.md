# FuzzySimulated — Inference Platform

Plataforma full-stack **100% Rust** para construção e simulação de sistemas de inferência fuzzy (Mamdani), com otimização multivariável e cálculo de ponto ótimo de funções objetivo.

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
| **Testes** | Rust unit + Axum HTTP + Playwright E2E |

## Arquitetura

```
fuzzysimulated/
├── server/          # Axum API + engine fuzzy + otimizador
│   ├── src/
│   │   ├── engine.rs      # Motor Mamdani (fuzzificação → agregação → defuzz centroide)
│   │   ├── math.rs        # Otimização quadrática (Hessiana, gradiente, classificação)
│   │   ├── audit.rs       # Trilha de auditoria com snapshots JSONB
│   │   ├── validation.rs  # Validação de MF, sistema, defuzz method
│   │   └── routes/        # Systems, Variables, Terms, Rules, Simulate, Optimize, Audit, Weather
│   ├── tests/
│   │   ├── axum_api.rs    # 39 testes HTTP (serializados)
│   │   ├── api_test.rs    # 8 testes de integração (transaction rollback)
│   │   ├── unit/          # 22 testes unitários (mf_validation, system_validation, optimization)
│   │   └── common/        # TestApp helper
│   └── migrations/        # SQLx migrations (schema + seed + audit + status)
├── app/             # Leptos components + server_fns
│   └── src/
│       └── lib.rs         # ~1800 linhas de UI reativa
├── frontend/        # WASM entry point
└── end2end/         # Playwright E2E tests
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
| Simulador | `/sim?s=` | ✅ |
| Histórico de simulações | `/hist` | ✅ |
| Superfície de resposta | `/analysis` | ✅ |
| Auditoria (com undo) | `/audit?id=` | ✅ |
| Otimizador | `/opt` | ✅ |

## Motor Mamdani

Pipeline de inferência real (não mock):

1. **Fuzzificação** — membership functions: `trimf`, `trapmf`, `gaussmf`
2. **Agregação** — operador `min` para AND, `max` para OR, clipping com weight da regra
3. **Defuzzificação** — centroide discreto com resolução configurável (default 501 pontos)

## Comandos

```bash
# Watch mode (porta 3000)
cargo leptos watch

# Unit tests (41, sem DB)
cargo test -p server --lib -- --skip ignored

# HTTP tests (39, requer DB fuzzysimulated_test)
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test axum_api

# Integration tests (8, requer DB)
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test api_test -- --ignored

# Todos os testes
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server

# End-to-end (requer servidor rodando)
cd end2end && npx playwright test

# Cobertura (requer cargo-llvm-cov)
./coverage.sh

# Análise estática
cargo clippy -p server
cargo audit
```

## Testes

| Suite | Qtde | DB | Como rodar |
|---|---|---|---|
| Unit (inline) | 41 | ❌ | `cargo test -p server --lib` |
| Unit (tests/) | 22 | ❌ | `cargo test -p server --test api_test -- unit::` |
| HTTP Axum | 39 | ✅ | `cargo test -p server --test axum_api` (serial) |
| Integration | 8 | ✅ | `cargo test -p server --test api_test -- --ignored` |
| E2E Playwright | 3 | ✅ | `cd end2end && npx playwright test` |
| **Total** | **113** | | |

Todos os 39 testes HTTP usam `#[serial_test::serial]` para evitar deadlocks do `TRUNCATE CASCADE` concorrente.

## Funcionalidades

- **Motor Mamdani real** — fuzzificação, agregação min, defuzz centroide discreto
- **Estados do sistema:** Ativo, Favorito (protege de deleção), Concluído (só simular), Desativado (oculto)
- **Auditoria com undo real:** restore completo de sistema + variáveis + termos + regras via snapshots JSONB
- **Otimizador:** Hessiana, gradiente, classificação de ponto crítico (mínimo/máximo/sela), busca por intervalo
- **Seed demo:** Sistema "Conforto Térmico" com 3 variáveis, 9 termos, 9 regras
