# FuzzySimulated

> Plataforma web full-stack para construção, persistência e simulação de Sistemas de Inferência Fuzzy Mamdani — com visualização interativa e dados climáticos em tempo real.

**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional — CESUPA 01/2026  
**Modalidade (IA):** Opção B — Aplicação/Produto baseado em Controle Fuzzy  
**Equipe:** Benjamin Yuji Suzuki  
**Repositório:** https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST

---

## 📌 Resumo

**FuzzySimulated** é uma plataforma web full-stack desenvolvida inteiramente em Rust que permite ao usuário montar sistemas de lógica fuzzy Mamdani de forma visual, persistir configurações em banco de dados relacional, executar simulações e visualizar o pipeline completo de inferência — fuzzificação, avaliação de regras, agregação e defuzzificação.

A plataforma integra a biblioteca [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) — implementação pura em Rust do algoritmo Mamdani — com dados climáticos reais via **OpenWeather API**, possibilitando usar temperatura e umidade de qualquer cidade como entradas automáticas do sistema fuzzy.

---

## 🎯 Problema e Justificativa para Lógica Fuzzy

Sistemas de controle fuzzy operam sobre variáveis linguísticas imprecisas ("temperatura alta", "umidade baixa") que não podem ser representadas adequadamente por lógica booleana clássica. A plataforma resolve dois problemas concretos:

1. **Barreira de entrada**: configurar um sistema Mamdani exige conhecimento de bibliotecas científicas (scikit-fuzzy, etc.) e programação. A plataforma oferece uma interface visual sem código.
2. **Desconexão com dados reais**: simulações geralmente usam valores hipotéticos. A integração com OpenWeather permite validar o sistema com condições climáticas reais.

A lógica fuzzy é adequada porque o domínio central — conforto térmico, decisões de controle ambiental, recomendações com base em clima — envolve gradação, imprecisão e julgamento linguístico que a lógica clássica não captura.

---

## 🧱 Stack Tecnológica

| Camada | Tecnologia | Justificativa |
|---|---|---|
| Frontend | [Leptos 0.8](https://leptos.dev/) (Rust → WASM) | Framework reativo em Rust; SSR + hydration; consistência total de linguagem |
| Backend | [Axum 0.8](https://github.com/tokio-rs/axum) (Rust) | Framework assíncrono de alta performance; integração nativa com Tokio |
| Banco de dados | PostgreSQL | Relacional robusto; suporte a JSONB para armazenar termos fuzzy flexíveis |
| ORM | [SQLx](https://github.com/launchbadge/sqlx) | Queries verificadas em tempo de compilação; zero overhead |
| Motor Fuzzy | [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) | Implementação Mamdani pura em Rust; sem dependências externas |
| API Externa | [OpenWeather API](https://openweathermap.org/api) | Temperatura e umidade reais por cidade; enriquece as simulações |
| Build | [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) | Build tool oficial do Leptos; gerencia WASM + servidor em um único comando |
| Qualidade | [SonarQube Cloud](https://sonarcloud.io) | Análise estática: complexidade, duplicação, code smells, vulnerabilidades |

---

## 📁 Estrutura do Repositório

```
FullStackEmRUST/
└── fuzzysimulated/
    ├── Cargo.toml          # workspace Rust
    ├── Cargo.lock          # versões fixadas (binário — deve ser commitado)
    ├── .env.example        # template de variáveis de ambiente
    ├── app/                # crate compartilhada Leptos (SSR + CSR)
    │   └── src/lib.rs
    ├── server/             # crate Axum — lógica de negócio e rotas
    │   └── src/main.rs
    ├── frontend/           # crate WASM — entry point client-side
    │   └── src/lib.rs
    ├── end2end/            # testes Playwright (E2E)
    ├── style/              # SCSS global
    └── public/             # assets estáticos
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

## 🔀 Modelo Fuzzy — Visão Geral

### Entradas e Saída (exemplo padrão da plataforma)

| Variável | Papel | Universo | Termos linguísticos |
|---|---|---|---|
| Temperatura | Antecedente | [0, 50] °C | Fria · Agradável · Quente |
| Umidade | Antecedente | [0, 100] % | Baixa · Média · Alta |
| Conforto | Consequente | [0, 100] | Desconfortável · Regular · Confortável |

> O usuário pode definir qualquer conjunto de variáveis e termos via interface — o exemplo acima é o sistema-padrão de demonstração.

### Funções de Pertinência suportadas

| Tipo | Parâmetros | Uso típico |
|---|---|---|
| `trimf` | [a, b, c] | Termos centrais triangulares |
| `trapmf` | [a, b, c, d] | Termos com platô (extremos) |
| `gaussmf` | [mean, sigma] | Transições suaves |

### Inferência Mamdani

1. **Fuzzificação** — cada input crisp é mapeado para graus de pertinência em cada termo.
2. **Avaliação de regras** — operador AND (mínimo) entre antecedentes; o grau de ativação corta a pertinência do consequente (implicação mínimo).
3. **Agregação** — união (máximo) de todos os consequentes ativados.
4. **Defuzzificação** — método centroide (padrão) ou outros métodos configuráveis.

### Base de Regras (exemplo padrão — ≥ 12 regras)

| # | Se Temperatura é… | E Umidade é… | Então Conforto é… |
|---|---|---|---|
| R01 | Fria | Baixa | Regular |
| R02 | Fria | Média | Desconfortável |
| R03 | Fria | Alta | Desconfortável |
| R04 | Agradável | Baixa | Confortável |
| R05 | Agradável | Média | Confortável |
| R06 | Agradável | Alta | Regular |
| R07 | Quente | Baixa | Regular |
| R08 | Quente | Média | Desconfortável |
| R09 | Quente | Alta | Desconfortável |
| R10 | Fria | Baixa | Desconfortável |
| R11 | Agradável | Baixa | Confortável |
| R12 | Quente | Alta | Desconfortável |

> A plataforma armazena qualquer base de regras definida pelo usuário na tabela `fuzzy_rules`.

---

## 🖥️ Telas da Aplicação (5 CRUDs)

| # | Tela | Entidade | Operações |
|---|---|---|---|
| 1 | Dashboard | `fuzzy_systems` | Listar, criar, editar, excluir sistemas |
| 2 | Editor de Variáveis | `fuzzy_variables` + `fuzzy_terms` | Adicionar/remover variáveis e termos linguísticos |
| 3 | Editor de Regras | `fuzzy_rules` | Criar, editar, reordenar, excluir regras |
| 4 | Simulador | `simulations` | Executar simulação, buscar dados OpenWeather |
| 5 | Histórico | `simulations` | Listar, visualizar detalhes, excluir registros |

---

## 📋 Casos de Uso (15)

A especificação completa dos 15 casos de uso — com ator, pré-condições, fluxo principal, fluxos alternativos e pós-condições — está em **[USE_CASES.md](./USE_CASES.md)**.

Resumo:

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

---

## 🌐 Integração com API Externa — OpenWeather

A [OpenWeather Current Weather API](https://openweathermap.org/current) fornece temperatura (°C) e umidade (%) de qualquer cidade em tempo real, usadas como inputs automáticos do sistema fuzzy.

```
GET https://api.openweathermap.org/data/2.5/weather?q=Belém&appid={API_KEY}&units=metric
→ { "main": { "temp": 32.4, "humidity": 88 } }
```

O backend consome esse endpoint, extrai `temp` e `humidity`, persiste em `simulations.weather_data` (JSONB) e os retorna ao frontend para preenchimento automático dos campos de input.

---

## 🚀 Instalação e Execução

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
# Edite .env com suas credenciais:
# DATABASE_URL=postgres://user:password@localhost/fuzzysimulated
# OPENWEATHER_API_KEY=sua_chave_aqui
```

### 3. Criar o banco de dados

```bash
# Execute o schema SQL disponível em server/migrations/
psql -U postgres -c "CREATE DATABASE fuzzysimulated;"
psql -U postgres -d fuzzysimulated -f server/migrations/001_schema.sql
```

### 4. Rodar

```bash
cargo leptos watch
# Acesse http://127.0.0.1:3000
```

---

## 🧪 Testes

### Unitários e integração

```bash
cargo test
```

Cobrem: funções de pertinência (trimf, trapmf, gaussmf), pipeline de inferência Mamdani, defuzzificação por centroide, validação de regras, e camada de serviço do backend.

### End-to-End (Playwright)

```bash
cd end2end
npx playwright install
npx playwright test
```

Cobrem os fluxos críticos: criação de sistema, adição de variáveis/termos/regras, execução de simulação e consulta ao histórico.

### Qualidade estática — SonarQube Cloud

O projeto está integrado ao [SonarQube Cloud](https://sonarcloud.io/project/overview?id=Benjamin-Yuji-Suzuki_FullStackEmRUST). Métricas monitoradas: complexidade ciclomática, duplicação de código, code smells e vulnerabilidades de segurança.

---

## 📅 Cronograma

| Sprint | Data | Entregáveis |
|---|---|---|
| Sprint 1 | 12/05 | Escopo, modelagem, casos de uso, estrutura do repositório, esqueleto funcional |
| Sprint 2 | 19/05 | CRUDs funcionais, integração OpenWeather, testes unitários |
| Sprint 3 | 26/05 | Sistema completo, testes nos 3 níveis, cobertura, apresentação |

---

## 🤖 Declaração de Uso de IA

| Ferramenta | Finalidade | Resumo do uso | Revisão da equipe |
|---|---|---|---|
| Claude (Anthropic) | Revisão de documentação, estruturação do README e USE_CASES, esclarecimento de dúvidas sobre Rust/Leptos/SonarQube | Perguntas sobre boas práticas, geração de rascunhos de documentação, revisão de trechos de texto | Todo conteúdo foi revisado, ajustado e validado pelo autor antes de ser incorporado ao repositório |

---

## 📄 Licença

MIT © Benjamin Yuji Suzuki
