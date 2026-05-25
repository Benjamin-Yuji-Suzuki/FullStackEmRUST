# FuzzySimulated

> Plataforma web full-stack para construção, persistência e simulação de Sistemas de Inferência Fuzzy Mamdani e TSK — com visualização interativa, histórico, auditoria e otimização por PSO.

[![codecov](https://codecov.io/gh/Benjamin-Yuji-Suzuki/FullStackEmRUST/branch/main/graph/badge.svg)](https://codecov.io/gh/Benjamin-Yuji-Suzuki/FullStackEmRUST)

**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados · Resolução de Problemas Multivariáveis — CESUPA 02/2026  
**Modalidade (IA):** Opção B + Opção C-B (TSK) + Pontuação Extra (PSO)  
**Equipe:** Benjamin Yuji Suzuki  
**Repositório principal:** https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST

---

## Resumo

**FuzzySimulated** é uma plataforma web desenvolvida inteiramente em Rust que permite montar sistemas de inferência fuzzy (Mamdani e TSK) de forma visual, persistir configurações em banco relacional, executar simulações com ambos os motores, visualizar o pipeline completo e otimizar parâmetros via Particle Swarm Optimization (PSO).

O projeto é composto por dois repositórios:

| Repositório | Função | Tecnologias |
|---|---|---|
| [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) | Motor de inferência Mamdani/TSK/PSO, publicado como crate | Rust puro |
| [`FullStackEmRUST`](https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST) ← **este repo** | Plataforma web full-stack | Leptos · Axum · PostgreSQL |

---

## Stack Tecnológica

| Camada | Tecnologia | Justificativa |
|---|---|---|
| Frontend | [Leptos 0.8](https://leptos.dev/) (Rust → WASM) | Framework reativo em Rust; SSR + hydration; `gloo-net` para HTTP no WASM |
| Backend | [Axum 0.8](https://github.com/tokio-rs/axum) (Rust) | Framework assíncrono; integração nativa com Tokio |
| Banco de dados | PostgreSQL | Relacional robusto; JSONB para termos fuzzy flexíveis |
| ORM | [SQLx](https://github.com/launchbadge/sqlx) | Queries verificadas em tempo de compilação |
| Motor Fuzzy | [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) | Implementação Mamdani/TSK/PSO |
| Pipeline de dados | [Polars](https://pola.rs/) (Rust) | Leitura de Parquet + inferência em lote (planejado) |
| API Externa | [OpenWeather API](https://openweathermap.org/api) | Temperatura e umidade reais por cidade |
| Build | [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) | Gerencia WASM + servidor em um único comando |
| Qualidade | [SonarQube Cloud](https://sonarcloud.io) | Análise estática: complexidade, duplicação, code smells, vulnerabilidades |

---

## Estrutura do Repositório

```
FullStackEmRUST/
├── USE_CASES.md              # 25 casos de uso
├── TEST_CASES.md             # 55 casos de teste
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
    │   │   ├── 001_schema.sql
    │   │   ├── 002_seed.sql
    │   │   ├── 004_audit_orphan.sql
    │   │   ├── 005_system_status.sql
    │   │   ├── 006_scenarios.sql
    │   │   ├── 007_seed_risco.sql
    │   │   ├── 008_seed_risco_cibernetico.sql
    │   │   └── 009_reset_and_seed.sql
    │   └── tests/
    │       ├── all.rs           # 80 testes (16 unit + 64 HTTP)
    │       ├── common/          # TestApp helper
    │       ├── unit/            # 16 unit tests sem DB
    │       └── integration/     # 6 integration tests com DB
    ├── frontend/              # crate WASM — entry point hydrate
    │   └── src/lib.rs
    ├── end2end/               # 41 testes Playwright E2E
    ├── style/
    │   └── main.scss          # SCSS global (tema escuro Catppuccin)
    └── public/                # assets estáticos
```

---

## Telas da Aplicação

| # | Tela | UCs | Status | Operações |
|---|---|---|---|---|
| 1 | Dashboard | UC01, UC10, UC11 | ✅ Funcional | CRUD sistemas, criar via formulário, excluir |
| 2 | Editor de Variáveis | UC02 | ✅ Funcional | Lista vars/termos, add var, add termo, editar var/termo |
| 3 | Editor de Regras | UC03 | ✅ Funcional | Lista regras, add regra, editar regra |
| 4 | Simulador | UC04, UC05, UC12, UC13 | ✅ Funcional | Sliders por variável, busca clima OpenWeather, executa simulação |
| 5 | Histórico | UC06, UC08, UC09 | ✅ Funcional | Lista simulações por sistema |
| 6 | Dashboard Batch | UC07 | ✅ Funcional | Upload CSV/Parquet, mapear colunas, inferência em lote |
| 7 | Análise | UC13, UC14, UC15 | ✅ Funcional | Varredura, matriz de regras ativadas, superfície de controle |
| 8 | Auditoria | UC16 | ✅ Funcional | Timeline + undo real com snapshots + recuperação de sistemas deletados |
| 9 | Estados do Sistema | — | ✅ Funcional | Ativo/Favorito/Concluído/Desativado com proteção e filtros |
| 10 | Otimizador | UC17 | ✅ Funcional | PSO para ajuste de parâmetros de MF |
| 11 | Importar Sistema | UC11 | ✅ Funcional | Upload JSON com validação de estrutura |

---

## Casos de Uso (20)

Especificação completa em **[USE_CASES.md](./USE_CASES.md)**.

| ID | Nome | Ator(es) |
|---|---|---|
| UC01 | Gerenciar Sistemas Fuzzy | Usuário |
| UC02 | Gerenciar Variáveis e Termos | Usuário |
| UC03 | Gerenciar Regras Fuzzy | Usuário |
| UC04 | Executar Inferência Mamdani | Usuário |
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

Crie o arquivo `.env` em `fuzzysimulated/`:

```env
DATABASE_URL=postgres://user:password@localhost/fuzzysimulated
OPENWEATHER_API_KEY=sua_chave_aqui
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

| Suite | Qtde | DB | Como rodar |
|---|---|---|---|---|---|
| Unit (inline) | 30 | ❌ | `cargo test -p server --lib` |
| Unit (tests/) | 16 | ❌ | `cargo test -p server --test all -- unit::` |
| HTTP Axum | 64 | ✅ | `DATABASE_URL=... cargo test -p server --test all -- --skip ignored` |
| Integration | 6 | ✅ | `DATABASE_URL=... cargo test -p server --test all -- --ignored` |
| **Total server** | **116** | | `DATABASE_URL=... cargo test -p server` |

Todos os testes HTTP usam `#[serial_test::serial]` para evitar deadlocks do `TRUNCATE CASCADE`.

---

## Documentação Complementar

| Documento | Conteúdo |
|---|---|
| [USE_CASES.md](./USE_CASES.md) | Especificação dos 20 casos de uso |
| [TEST_CASES.md](./TEST_CASES.md) | 44 casos de teste (116 testes server) |
| [FUZZY_MODEL.md](./FUZZY_MODEL.md) | Modelos Mamdani, TSK e Otimização PSO |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Banco de dados, integrações e fluxos |

---

## Declaração de Uso de IA

Conforme exigido pela disciplina de Inteligência Artificial e Computacional, declaramos abaixo o uso de ferramentas de IA generativa e agentes de desenvolvimento, com transparência sobre finalidade, prompts resumidos e revisão humana.

| Ferramenta | Finalidade | Prompt/Comando resumido | Revisão da equipe |
|---|---|---|---|
| opencode (DeepSeek V4) | Desenvolvimento de código (componentes Leptos, rotas Axum, testes Rust), estruturação de documentação, correção de bugs de compilação, debug de WASM/SSR | "Implementar página de variáveis", "corrigir erro Pool not initialized", "criar rotas REST para rules", "debug MIME types WASM", "converter HTML/JS para 100% Rust" | Todo código gerado foi revisado, testado (cargo check + cargo test) e validado quanto à correção funcional antes de incorporar ao repositório |
| Claude (Anthropic) | Revisão inicial de documentação e estruturação do projeto | "Revisar USE_CASES.md", "sugerir estrutura de projeto Rust fullstack" | Contribuições iniciais revisadas e adaptadas |
| Gemini Pro (Google) | Ideação de arquitetura — sugestão de integrar pipeline de dados ao ecossistema fullstack | "Como estruturar um projeto fullstack Rust com Leptos + Axum?" | Ideia avaliada criticamente e adaptada pelo autor |

**Nota:** O uso de IA não substitui o domínio conceitual da equipe. Todos os integrantes compreendem e são capazes de explicar cada parte do código, modelo fuzzy e decisões de arquitetura.

---

## Licença

MIT © Benjamin Yuji Suzuki
