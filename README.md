# FuzzySimulated

> Plataforma web full-stack para construção, persistência e simulação de Sistemas de Inferência Fuzzy Mamdani — com visualização interativa e análise de datasets de cibersegurança via upload de Parquet.

**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados — CESUPA 01/2026  
**Modalidade (IA):** Opção B — Aplicação/Produto baseado em Controle Fuzzy  
**Equipe:** Benjamin Yuji Suzuki  
**Repositório principal:** https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST

---

## Resumo

**FuzzySimulated** é uma plataforma web desenvolvida inteiramente em Rust que permite montar sistemas de lógica fuzzy Mamdani de forma visual, persistir configurações em banco relacional, executar simulações e visualizar o pipeline completo de inferência — fuzzificação, avaliação de regras, agregação e defuzzificação.

O projeto é composto por dois repositórios:

| Repositório | Função | Tecnologias |
|---|---|---|
| [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) | Motor de inferência Mamdani puro, publicado como crate open-source | Rust puro |
| [`FullStackEmRUST`](https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST) ← **este repo** | Plataforma web que consome o motor fuzzy | Leptos · Axum · PostgreSQL |

---

## Stack Tecnológica

| Camada | Tecnologia | Justificativa |
|---|---|---|
| Frontend | [Leptos 0.8](https://leptos.dev/) (Rust → WASM) | Framework reativo em Rust; SSR + hydration; consistência total de linguagem |
| Backend | [Axum 0.8](https://github.com/tokio-rs/axum) (Rust) | Framework assíncrono; integração nativa com Tokio |
| Banco de dados | PostgreSQL | Relacional robusto; JSONB para termos fuzzy flexíveis |
| ORM | [SQLx](https://github.com/launchbadge/sqlx) | Queries verificadas em tempo de compilação |
| Motor Fuzzy | [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) | Implementação Mamdani pura em Rust, publicada no crates.io |
| Pipeline de dados | [Polars](https://github.com/pola-rs/polars) (Rust) | Processamento de DataFrames em Rust; leitura de Parquet no backend |
| API Externa | [OpenWeather API](https://openweathermap.org/api) | Temperatura e umidade reais por cidade |
| Build | [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) | Gerencia WASM + servidor em um único comando |
| Qualidade | [SonarQube Cloud](https://sonarcloud.io) | Análise estática: complexidade, duplicação, code smells, vulnerabilidades |

---

## Estrutura do Repositório

```
FullStackEmRUST/
├── USE_CASES.md              # 20 casos de uso
├── TEST_CASES.md             # 43 casos de teste
├── FUZZY_MODEL.md            # Modelo fuzzy de demonstração
├── ARCHITECTURE.md           # Arquitetura técnica
├── README.md
└── fuzzysimulated/
    ├── Cargo.toml             # workspace Rust
    ├── Cargo.lock             # versões fixadas (commitado)
    ├── app/                   # crate compartilhada Leptos (SSR + CSR)
    │   └── src/
    │       ├── lib.rs         # componentes e páginas
    │       └── server_fns.rs  # chamadas à REST API (gloo-net/reqwest)
    ├── server/                # crate Axum — REST API + SSR
    │   ├── Cargo.toml
    │   ├── src/
    │   │   ├── main.rs        # entry point, router, static files
    │   │   ├── audit.rs       # helper de auditoria
    │   │   ├── errors.rs      # AppError
    │   │   ├── models/        # FuzzySystem, Variable, Term, Rule, etc.
    │   │   ├── routes/        # systems, variables, rules, simulate, weather, audit
    │   │   └── state.rs       # AppState (PgPool + LeptosOptions)
    │   ├── migrations/
    │   │   └── 001_schema.sql # 7 tabelas + índices
    │   └── tests/
    │       └── api_test.rs    # 16 unit + 6 integration tests
    ├── frontend/              # crate WASM — entry point hydrate
    │   └── src/lib.rs
    ├── end2end/               # testes Playwright (E2E)
    ├── style/
    │   └── main.scss          # SCSS global (tema escuro Catppuccin)
    └── public/                # assets estáticos
```

---

## Telas da Aplicação

| # | Tela | UCs | Status | Operações |
|---|---|---|---|---|
| 1 | Dashboard | UC01, UC10, UC11 | ✅ Funcional | CRUD sistemas, criar via formulário, excluir |
| 2 | Editor de Variáveis | UC02 | ✅ Funcional | Lista vars/termos, add var, add termo |
| 3 | Editor de Regras | UC03 | ✅ Funcional | Lista regras por sistema |
| 4 | Simulador | UC04, UC05, UC12, UC13 | ⚡ Esboço | Seleciona sistema |
| 5 | Histórico | UC06, UC08, UC09 | ✅ Funcional | Lista simulações por sistema |
| 6 | Dashboard Batch | UC07 | ❌ Placeholder | — |
| 7 | Análise | UC14, UC15 | ❌ Placeholder | — |
| 8 | Auditoria | UC16 | ✅ Funcional | Timeline de alterações no banco |

---

## Casos de Uso (20)

Especificação completa em **[USE_CASES.md](./USE_CASES.md)**.

| ID | Nome | Ator(es) |
|---|---|---|
| UC01 | Gerenciar Sistemas Fuzzy | Usuário |
| UC02 | Gerenciar Variáveis e Termos | Usuário |
| UC03 | Gerenciar Regras Fuzzy | Usuário |
| UC04 | Executar Simulação | Usuário |
| UC05 | Buscar Dados Climáticos | Usuário, OpenWeather API |
| UC06 | Consultar Histórico de Simulações | Usuário |
| UC07 | Processar Inferência em Lote | Usuário |
| UC08 | Comparar Simulações | Usuário |
| UC09 | Exportar Relatório de Simulação | Usuário |
| UC10 | Duplicar Sistema Fuzzy | Usuário |
| UC11 | Exportar e Importar Sistema | Usuário |
| UC12 | Salvar Cenário de Simulação | Usuário |
| UC13 | Executar Varredura de Entrada | Usuário |
| UC14 | Visualizar Matriz de Regras Ativadas | Usuário |
| UC15 | Visualizar Superfície de Controle | Usuário |
| UC16 | Gerenciar Histórico de Alterações | Usuário |
| UC17 | Otimizar Parâmetros com PSO | Usuário |
| UC18 | Executar Inferência TSK | Usuário |
| UC19 | Exportar Visualizações SVG | Usuário |
| UC20 | Visualizar Relatório de Diagnóstico | Usuário |

---

## Instalação e Execução

### Pré-requisitos

- Rust estável com target WASM: `rustup target add wasm32-unknown-unknown`
- `cargo-leptos`: `cargo install cargo-leptos --locked`
- PostgreSQL rodando localmente (porta 5432)
- Chave gratuita da [OpenWeather API](https://home.openweathermap.org/api_keys)

### 1. Clonar o repositório

```bash
git clone https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST
cd FullStackEmRUST/fuzzysimulated
```

### 2. Configurar variáveis de ambiente

```bash
cp .env.example .env
# Edite .env:
# DATABASE_URL=postgres://user:password@localhost/fuzzysimulated
# OPENWEATHER_API_KEY=sua_chave_aqui
```

### 3. Criar o banco de dados

```bash
psql -U postgres -c "CREATE DATABASE fuzzysimulated;"
psql -U postgres -d fuzzysimulated -f server/migrations/001_schema.sql
```

### 4. Rodar

```bash
cargo leptos watch
# Acesse http://127.0.0.1:3000
```

---

## Testes

### Unitários (16)

```bash
cargo test -p server -- --skip ignored
```

Cobrem: validação de nome de sistema (vazio, tamanho), método de defuzzificação, parâmetros de MF (trimf, trapmf, gaussmf).

### Integração (6 — esboçados)

```bash
# Requer banco 'fuzzysimulated_test' com migrations aplicadas
DATABASE_URL=postgres://postgres@localhost/fuzzysimulated_test cargo test -p server -- --ignored
```

Cobrem: CRUD de sistema, variável, termo, cascade delete, simulação.

### End-to-End (Playwright)

```bash
cd end2end
npx playwright install
npx playwright test
```

> ⚠️ Testes E2E ainda não implementados (esboço em `end2end/tests/example.spec.ts`).

### Qualidade estática — SonarQube Cloud

Projeto integrado ao [SonarQube Cloud](https://sonarcloud.io/project/overview?id=Benjamin-Yuji-Suzuki_FullStackEmRUST). Métricas monitoradas: complexidade ciclomática, duplicação de código, code smells e vulnerabilidades de segurança.

---

## Documentação Complementar

| Documento | Conteúdo |
|---|---|---|
| [USE_CASES.md](./USE_CASES.md) | Especificação completa dos 20 casos de uso |
| [TEST_CASES.md](./TEST_CASES.md) | Casos de teste documentados para os 20 UCs (43 casos, 16 ✅) |
| [FUZZY_MODEL.md](./FUZZY_MODEL.md) | Variáveis, universos, funções de pertinência, base de regras e cenários de teste |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Modelagem do banco, integrações (OpenWeather + Parquet) e fluxo de dados |

---

## Declaração de Uso de IA

| Ferramenta | Finalidade | Revisão da equipe |
|---|---|---|
| Claude (Anthropic) | Revisão de documentação, estruturação de README/USE_CASES/FUZZY_MODEL, dúvidas sobre Rust/Leptos/SonarQube/Polars, avaliação de arquitetura | Todo conteúdo revisado, ajustado e validado pelo autor antes de incorporar ao repositório |
| Gemini Pro (Google) | Ideação de arquitetura — sugestão de integrar pipeline de dados ao ecossistema fullstack | Ideia avaliada criticamente e adaptada pelo autor |

---

## Licença

MIT © Benjamin Yuji Suzuki
