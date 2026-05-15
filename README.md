# FuzzySimulated

> Plataforma web full-stack para construção, persistência e simulação de Sistemas de Inferência Fuzzy Mamdani e TSK — com visualização interativa, histórico, auditoria e otimização por PSO.

**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados — CESUPA 01/2026  
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
| [`FullStackEmRUST`](https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST) ← **este repo** | Plataforma web full-stack (simulação atualmente via mock) | Leptos · Axum · PostgreSQL |

---

## Stack Tecnológica

| Camada | Tecnologia | Justificativa |
|---|---|---|
| Frontend | [Leptos 0.8](https://leptos.dev/) (Rust → WASM) | Framework reativo em Rust; SSR + hydration; `gloo-net` para HTTP no WASM |
| Backend | [Axum 0.8](https://github.com/tokio-rs/axum) (Rust) | Framework assíncrono; integração nativa com Tokio |
| Banco de dados | PostgreSQL | Relacional robusto; JSONB para termos fuzzy flexíveis |
| ORM | [SQLx](https://github.com/launchbadge/sqlx) | Queries verificadas em tempo de compilação |
| Motor Fuzzy | [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) | Implementação Mamdani/TSK/PSO (pendente de integração — atualmente mock) |
| API Externa | [OpenWeather API](https://openweathermap.org/api) | Temperatura e umidade reais por cidade |
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

### Qualidade estática — SonarQube Cloud (planejado)

> ⚠️ Integração com SonarQube Cloud pendente para Sprint 3. Atualmente sem análise automática.

---

## Documentação Complementar

| Documento | Conteúdo |
|---|---|---|
| [USE_CASES.md](./USE_CASES.md) | Especificação completa dos 20 casos de uso |
| [TEST_CASES.md](./TEST_CASES.md) | Casos de teste documentados (43 casos, 16 aprovados) |
| [FUZZY_MODEL.md](./FUZZY_MODEL.md) | Modelos Mamdani e TSK + Otimização PSO |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Modelagem do banco, integrações e fluxo de dados |

---

## Declaração de Uso de IA (Obrigatória — Lauda IA §9)

Conforme exigido pela disciplina de Inteligência Artificial e Computacional, declaramos abaixo o uso de ferramentas de IA generativa e agentes de desenvolvimento, com transparência sobre finalidade, prompts resumidos e revisão humana.

| Ferramenta | Finalidade | Prompt/Comando resumido | Revisão da equipe |
|---|---|---|---|
| Claude (Anthropic) / opencode | Desenvolvimento de código (componentes Leptos, rotas Axum, testes Rust), estruturação de documentação, correção de bugs de compilação, debug de WASM/SSR | "Implementar página de variáveis", "corrigir erro Pool not initialized", "criar rotas REST para rules", "debug MIME types WASM", "converter HTML/JS para 100% Rust" | Todo código gerado foi revisado, testado (cargo check + cargo test) e validado quanto à correção funcional antes de incorporar ao repositório |
| Gemini Pro (Google) | Ideação de arquitetura — sugestão de integrar pipeline de dados ao ecossistema fullstack | "Como estruturar um projeto fullstack Rust com Leptos + Axum + fuzzy inference?" | Ideia avaliada criticamente e adaptada pelo autor conforme viabilidade técnica |
| ChatGPT (OpenAI) | Esclarecimento de conceitos de lógica fuzzy (Mamdani, TSK, PSO) e revisão de documentação | "Diferença entre Mamdani e TSK", "como documentar casos de uso UML" | Conteúdo revisado e adaptado ao contexto do projeto |

**Nota:** O uso de IA não substitui o domínio conceitual da equipe. Todos os integrantes compreendem e são capazes de explicar cada parte do código, modelo fuzzy e decisões de arquitetura.

---

## Licença

MIT © Benjamin Yuji Suzuki
