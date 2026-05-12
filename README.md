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

| # | Tela | Entidade principal | Operações |
|---|---|---|---|
| 1 | Dashboard | `fuzzy_systems` | Listar, criar, editar, excluir sistemas |
| 2 | Editor de Variáveis | `fuzzy_variables` + `fuzzy_terms` | Adicionar/remover variáveis e termos linguísticos |
| 3 | Editor de Regras | `fuzzy_rules` | Criar, editar, reordenar, excluir regras |
| 4 | Simulador | `simulations` | Executar simulação manual, buscar dados OpenWeather |
| 5 | Histórico | `simulations` | Listar, expandir detalhes, excluir registros |
| 6 | Dashboard Batch | `batch_results` | Upload de Parquet, mapeamento de colunas, execução em lote, visualização de resultados |
| 7 | Gerenciamento de Variáveis do Dataset | `parquet_columns` (frontend) | Renomear colunas do Parquet para variáveis fuzzy, normalizar nomes com caracteres especiais |

---

## Casos de Uso (17)

Especificação completa em **[USE_CASES.md](./USE_CASES.md)**.

| ID | Nome | Tela |
|---|---|---|
| UC01 | Criar novo sistema fuzzy | Dashboard |
| UC02 | Editar metadados de um sistema | Dashboard |
| UC03 | Excluir sistema fuzzy | Dashboard |
| UC04 | Adicionar variável antecedente | Editor de Variáveis |
| UC05 | Adicionar variável consequente | Editor de Variáveis |
| UC06 | Adicionar termo linguístico a uma variável | Editor de Variáveis |
| UC07 | Remover variável ou termo | Editor de Variáveis |
| UC08 | Criar regra fuzzy via interface visual | Editor de Regras |
| UC09 | Editar regra existente | Editor de Regras |
| UC10 | Remover regra | Editor de Regras |
| UC11 | Executar simulação com inputs manuais | Simulador |
| UC12 | Buscar dados climáticos reais via OpenWeather | Simulador |
| UC13 | Visualizar pipeline completo da simulação | Simulador |
| UC14 | Consultar histórico de simulações | Histórico |
| UC15 | Validar sistema antes de executar | Sistema (automático) |
| UC16 | Carregar dataset Parquet e executar inferência em lote | Dashboard Batch |
| UC17 | Renomear colunas do Parquet via dashboard para normalizar nomes | Gerenciamento de Variáveis do Dataset |

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
| [USE_CASES.md](./USE_CASES.md) | Especificação completa dos 17 casos de uso |
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
