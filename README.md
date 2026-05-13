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
└── fuzzysimulated/
    ├── Cargo.toml            # workspace Rust
    ├── Cargo.lock            # versões fixadas (commitado)
    ├── .env.example          # template de variáveis de ambiente
    ├── app/                  # crate compartilhada Leptos (SSR + CSR)
    │   └── src/lib.rs
    ├── server/               # crate Axum — rotas e lógica de negócio
    │   ├── src/main.rs
    │   └── migrations/
    │       └── 001_schema.sql
    ├── frontend/             # crate WASM — entry point client-side
    │   └── src/lib.rs
    ├── end2end/              # testes Playwright (E2E)
    ├── style/                # SCSS global
    └── public/               # assets estáticos
```

---

## Telas da Aplicação

| # | Tela | UCs | Operações |
|---|---|---|---|
| 1 | Dashboard | UC01, UC10, UC11 | CRUD sistemas, duplicar, exportar/importar |
| 2 | Editor de Variáveis | UC02 | CRUD variáveis e termos linguísticos |
| 3 | Editor de Regras | UC03 | CRUD regras fuzzy |
| 4 | Simulador | UC04, UC05, UC12, UC13 | Simular, clima, cenários, varredura |
| 5 | Histórico | UC06, UC08, UC09 | Listar, comparar, exportar, excluir |
| 6 | Dashboard Batch | UC07 | Upload Parquet, mapeamento, inferência em lote |
| 7 | Análise | UC14, UC15 | Matriz de regras, superfície de controle |
| 8 | Timeline do Sistema | UC16 | Histórico de alterações, desfazer/refazer |

---

## Casos de Uso (16)

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

### Unitários e de integração

```bash
cargo test
```

Cobrem: funções de pertinência (trimf, trapmf, gaussmf), pipeline de inferência Mamdani, defuzzificação por centroide, validação de regras e camada de serviço do backend.

### End-to-End (Playwright)

```bash
cd end2end
npx playwright install
npx playwright test
```

Cobrem os fluxos críticos: criação de sistema, adição de variáveis/termos/regras, execução de simulação e consulta ao histórico.

### Qualidade estática — SonarQube Cloud

Projeto integrado ao [SonarQube Cloud](https://sonarcloud.io/project/overview?id=Benjamin-Yuji-Suzuki_FullStackEmRUST). Métricas monitoradas: complexidade ciclomática, duplicação de código, code smells e vulnerabilidades de segurança.

---

## Documentação Complementar

| Documento | Conteúdo |
|---|---|
| [USE_CASES.md](./USE_CASES.md) | Especificação completa dos 16 casos de uso |
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
