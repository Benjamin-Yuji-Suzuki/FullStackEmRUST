# ARCHITECTURE.md — FuzzySimulated

> Detalhamento técnico da arquitetura do sistema: modelagem do banco de dados, integrações externas e fluxo de dados.

---

## Modelagem do Banco de Dados

O schema é versionado em `server/migrations/001_schema.sql`. Todas as tabelas usam UUID como chave primária e campos JSONB para armazenar estruturas flexíveis (inputs, outputs, termos fuzzy).

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

CREATE TABLE batch_results (
  id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  system_id    UUID NOT NULL REFERENCES fuzzy_systems(id) ON DELETE CASCADE,
  source_file  TEXT NOT NULL,
  row_index    INT NOT NULL,
  inputs       JSONB NOT NULL,
  output       FLOAT NOT NULL,
  executed_at  TIMESTAMP DEFAULT NOW()
);
```

---

## Integração com API Externa — OpenWeather

A [OpenWeather Current Weather API](https://openweathermap.org/current) fornece temperatura (°C) e umidade (%) de qualquer cidade em tempo real, usadas como inputs automáticos do sistema fuzzy no Simulador (UC12).

```
GET https://api.openweathermap.org/data/2.5/weather?q=Belém&appid={API_KEY}&units=metric
→ { "main": { "temp": 32.4, "humidity": 88 } }
```

O backend extrai `temp` e `humidity`, persiste em `simulations.weather_data` (JSONB) e os retorna ao frontend para preenchimento automático dos inputs.

---

## Upload de Dataset Parquet — Dashboard Batch

O usuário pode carregar um arquivo Parquet diretamente pelo Dashboard Batch. O backend processa o arquivo via Polars em thread pool (sem bloquear o runtime Tokio) e executa a inferência fuzzy linha a linha usando o `logicfuzzy_academic`. Os resultados são persistidos em `batch_results` e exibidos no dashboard.

```
Frontend (Leptos)
  multipart upload do arquivo .parquet
          ↓
Axum backend
  POST /api/batch/upload
  spawn_blocking → Polars lê o Parquet
  mapeamento de colunas → variáveis fuzzy do sistema selecionado
  logicfuzzy_academic → MamdaniEngine por linha
  batch_results → persistência no PostgreSQL
          ↓
Dashboard Batch
  Leptos renderiza distribuição dos outputs
```

### Mapeamento e Renomeação de Colunas

Datasets de cibersegurança frequentemente contêm colunas com caracteres especiais, espaços ou nomes incompatíveis. A interface permite ao usuário renomear qualquer coluna do Parquet antes de mapeá-la para as variáveis fuzzy do sistema, sem alterar o arquivo original.

---

## Estrutura dos Repositórios

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

> O motor de inferência [`logicfuzzy-academic`](https://crates.io/crates/logicfuzzy_academic) é uma dependência externa publicada no crates.io — adicionada via `cargo add logicfuzzy-academic`, não faz parte deste repositório.

---

## Modelo Fuzzy — Visão Geral

O sistema-padrão pré-carregado para demonstração avalia **risco crítico de incidentes de cibersegurança** com base em impacto financeiro e impacto de mercado. O usuário pode criar qualquer sistema via interface — as tabelas abaixo descrevem apenas o exemplo de demonstração, e a base de regras é inteiramente configurável pelo usuário na tela Editor de Regras.

A especificação completa (parâmetros de pertinência, cenários de teste, análise de sensibilidade) está em **[FUZZY_MODEL.md](./FUZZY_MODEL.md)**.

### Variáveis

| Variável | Papel | Universo | Termos linguísticos |
|---|---|---|---|
| Impacto Financeiro | Antecedente | [0, 100] | Baixo · Médio · Alto |
| Impacto de Mercado | Antecedente | [0, 100] | Baixo · Médio · Alto |
| Risco Crítico | Consequente | [0, 100] | Tolerável · Moderado · Alto · Crítico · Severo |

### Funções de Pertinência Suportadas

| Tipo | Parâmetros | Uso típico |
|---|---|---|
| `trimf` | [a, b, c] | Termos centrais com transição triangular |
| `trapmf` | [a, b, c, d] | Termos extremos com platô de pertinência máxima |
| `gaussmf` | [mean, σ] | Transições suaves entre termos adjacentes |

### Inferência Mamdani

1. **Fuzzificação** — cada input crisp é mapeado para graus de pertinência em cada termo.
2. **Avaliação de regras** — operador AND (mínimo) entre antecedentes; grau de ativação corta a pertinência do consequente (implicação mínimo).
3. **Agregação** — união (máximo) de todos os consequentes ativados.
4. **Defuzzificação** — método centroide (padrão) ou outros métodos configuráveis pelo usuário.

### Base de Regras (sistema-padrão — 9 regras)

A base de regras abaixo é o ponto de partida do sistema de demonstração. O usuário pode editar, adicionar ou remover regras livremente pela interface do Editor de Regras.

| # | Se Impacto Financeiro é… | E Impacto de Mercado é… | Então Risco Crítico é… | Observação |
|---|---|---|---|---|
| R01 | Baixo | Baixo | Tolerável | Incidente isolado; monitoramento padrão é suficiente |
| R02 | Baixo | Médio | Moderado | Operação normal, mas requer ação de relações públicas |
| R03 | Baixo | Alto | Crítico | Risco alto de perda de clientes, mesmo sem grande custo direto |
| R04 | Médio | Baixo | Moderado | Custo operacional absorvível, sem alarde público |
| R05 | Médio | Médio | Alto | Prejuízo considerável e danos à reputação simultâneos |
| R06 | Médio | Alto | Crítico | Necessita acionamento imediato do comitê de crise |
| R07 | Alto | Baixo | Alto | Grande evasão de caixa, mesmo que o mercado não tenha precificado |
| R08 | Alto | Médio | Crítico | Perdas financeiras severas vazando para a percepção pública |
| R09 | Alto | Alto | Severo | Colapso simultâneo de caixa e reputação — resposta emergencial |
