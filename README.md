# FuzzySimulated

> Plataforma web para construção, persistência e simulação de Sistemas de Inferência Fuzzy Mamdani — com visualização interativa e dados climáticos em tempo real.

---

## 📌 Sobre o Projeto

**FuzzySimulated** é uma plataforma web full-stack desenvolvida em Rust puro, que permite ao usuário montar sistemas de lógica fuzzy de forma visual, salvar configurações no banco de dados, executar simulações e visualizar o pipeline completo de inferência.

A plataforma integra a biblioteca [`logicfuzzy-academic`](https://github.com/Benjamin-Yuji-Suzuki/logicfuzzy-academic) — implementação pura em Rust do algoritmo Mamdani — com dados climáticos reais via **OpenWeather API**, permitindo simular sistemas fuzzy com entradas do mundo real (temperatura, umidade, etc.).

**Desenvolvido como projeto prático da disciplina de Qualidade e Projeto de Software — CESUPA.**

---

## 🧱 Stack Tecnológica

| Camada | Tecnologia | Justificativa |
|---|---|---|
| Frontend | [Leptos 0.8](https://leptos.dev/) (Rust → WASM) | Framework reativo em Rust; SSR + hydration; consistência total de linguagem |
| Backend | [Axum 0.8](https://github.com/tokio-rs/axum) (Rust) | Framework assíncrono de alta performance; integração nativa com Tokio |
| Banco de dados | PostgreSQL | Relacional robusto; suporte a JSONB para armazenar termos fuzzy flexíveis |
| ORM | [SQLx](https://github.com/launchbadge/sqlx) | Queries verificadas em tempo de compilação; zero overhead |
| Motor Fuzzy | [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) | Implementação Mamdani pura em Rust; sem dependências externas |
| API Externa | [OpenWeather API](https://openweathermap.org/api) | Fornece temperatura e umidade reais por cidade; enriquece as simulações |
| Build | [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) | Build tool oficial do Leptos; gerencia WASM + servidor em um comando |

---

## 📁 Estrutura do Repositório

```
FullStackEmRUST/
└── fuzzysimulated/
    ├── Cargo.toml        # workspace
    ├── app/              # componentes Leptos compartilhados (SSR + CSR)
    │   └── src/lib.rs
    ├── server/           # crate Axum — lógica de servidor
    │   └── src/main.rs
    ├── frontend/         # crate WASM — entry point client-side
    │   └── src/lib.rs
    ├── end2end/          # testes Playwright (Sprint 3)
    ├── style/            # SCSS global
    └── public/           # assets estáticos
```

---

## 🗄️ Modelagem do Banco de Dados

```sql
CREATE TABLE fuzzy_systems (
  id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name          TEXT NOT NULL,
  description   TEXT,
  defuzz_method TEXT NOT NULL DEFAULT 'centroid',
  created_at    TIMESTAMP DEFAULT NOW(),
  updated_at    TIMESTAMP DEFAULT NOW()
);

CREATE TABLE fuzzy_variables (
  id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  system_id     UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
  name          TEXT NOT NULL,
  role          TEXT NOT NULL CHECK (role IN ('antecedent', 'consequent')),
  universe_min  FLOAT NOT NULL,
  universe_max  FLOAT NOT NULL,
  resolution    INT NOT NULL DEFAULT 501
);

CREATE TABLE fuzzy_terms (
  id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  variable_id UUID NOT NULL REFERENCES fuzzy_variables(id) ON DELETE CASCADE,
  label       TEXT NOT NULL,
  mf_type     TEXT NOT NULL CHECK (mf_type IN ('trimf', 'trapmf', 'gaussmf')),
  params      JSONB NOT NULL
);

CREATE TABLE fuzzy_rules (
  id        UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  system_id UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
  rule_text TEXT NOT NULL,
  weight    FLOAT NOT NULL DEFAULT 1.0,
  position  INT NOT NULL DEFAULT 0
);

CREATE TABLE simulations (
  id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  system_id    UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
  inputs       JSONB NOT NULL,
  outputs      JSONB NOT NULL,
  weather_data JSONB,
  city         TEXT,
  executed_at  TIMESTAMP DEFAULT NOW()
);
```

---

## 🖥️ Telas da Aplicação (5 CRUDs)

| # | Tela | Entidade | Operações |
|---|---|---|---|
| 1 | Dashboard | `fuzzy_systems` | Listar, criar, editar, excluir sistemas |
| 2 | Editor de Variáveis | `fuzzy_variables` + `fuzzy_terms` | Adicionar/remover variáveis e termos |
| 3 | Editor de Regras | `fuzzy_rules` | Criar, editar, reordenar, excluir regras |
| 4 | Simulador | `simulations` | Executar simulação, buscar dados OpenWeather |
| 5 | Histórico | `simulations` | Listar, visualizar detalhes, excluir |

---

## 📋 Casos de Uso (15)

| ID | Nome | Ator |
|---|---|---|
| UC01 | Criar novo sistema fuzzy | Usuário |
| UC02 | Editar metadados de um sistema | Usuário |
| UC03 | Excluir sistema fuzzy | Usuário |
| UC04 | Adicionar variável antecedente | Usuário |
| UC05 | Adicionar variável consequente | Usuário |
| UC06 | Adicionar termo linguístico a uma variável | Usuário |
| UC07 | Remover variável ou termo | Usuário |
| UC08 | Criar regra fuzzy via interface visual | Usuário |
| UC09 | Editar regra existente | Usuário |
| UC10 | Remover regra | Usuário |
| UC11 | Executar simulação com inputs manuais | Usuário |
| UC12 | Buscar dados climáticos reais via OpenWeather | Usuário |
| UC13 | Visualizar pipeline completo da simulação | Usuário |
| UC14 | Consultar histórico de simulações | Usuário |
| UC15 | Validar sistema antes de executar | Sistema |

---

## 🌐 Integração com API Externa — OpenWeather

A [OpenWeather Current Weather API](https://openweathermap.org/current) fornece temperatura (°C) e umidade (%) de qualquer cidade em tempo real, usadas como inputs automáticos do sistema fuzzy.

```
GET https://api.openweathermap.org/data/2.5/weather?q=Belém&appid={API_KEY}&units=metric
→ { "temp": 32.4, "humidity": 88 }
```

---

## 🚀 Instalação e Execução

### Pré-requisitos

- Rust com target `wasm32-unknown-unknown`
- `cargo-leptos` — `cargo install cargo-leptos --locked`
- PostgreSQL rodando localmente
- Chave da [OpenWeather API](https://home.openweathermap.org/api_keys) (gratuita)

### 1. Clonar o repositório

```bash
git clone https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST
cd FullStackEmRUST/fuzzysimulated
```

### 2. Configurar variáveis de ambiente

```bash
cp .env.example .env
# edite .env com suas credenciais
```

### 3. Rodar

```bash
cargo leptos watch
# acesse http://127.0.0.1:3000
```

---

## 🧪 Testes

```bash
# Unitários e integração
cargo test

# E2E (Sprint 3)
cd end2end && npx playwright test
```

---

## 📅 Cronograma

| Sprint | Data | Entregáveis |
|---|---|---|
| Sprint 1 | 12/05 | Escopo, modelagem, casos de uso, estrutura do repositório, esqueleto funcional |
| Sprint 2 | 19/05 | CRUDs funcionais, integração OpenWeather, testes unitários |
| Sprint 3 | 26/05 | Sistema completo, testes nos 3 níveis, cobertura, apresentação |

---

## 📄 Licença

MIT © Benjamin Yuji Suzuki
