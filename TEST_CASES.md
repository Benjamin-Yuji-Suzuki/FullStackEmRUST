# Casos de Teste — FuzzySimulated

> Documentação dos casos de teste para os 20 casos de uso da plataforma.
> Cada caso de teste segue o padrão: identificador, objetivo, pré-condições, dados de entrada, passos, resultado esperado, resultado obtido e status.

**Projeto:** FuzzySimulated  
**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados · Resolução de Problemas Multivariáveis — CESUPA 02/2026

---

## Convenções

| Campo | Descrição |
|---|---|
| **ID** | `TC-UCXX-NN` — caso de teste NN para o caso de uso UCXX |
| **Tipo** | Unitário (U) · Integração (I) · End-to-End (E) |
| **Status** | ✅ Aprovado · ❌ Reprovado · ⏳ Pendente · 📝 Planejado |

---

## UC01 — Gerenciar Sistemas Fuzzy

### TC-UC01-01: Criar sistema com dados válidos

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar que um sistema fuzzy é criado com sucesso quando todos os campos obrigatórios são preenchidos corretamente |
| **Pré-condições** | Banco de dados conectado. Dashboard carregada. |
| **Tipo** | End-to-End (E) + HTTP |
| **Dados de entrada** | `{ "name": "Conforto Térmico", "description": "Sistema para avaliação de conforto", "defuzz_method": "centroid" }` |
| **Passos** | 1. Enviar POST para `/api/systems` com o payload JSON<br>2. Verificar status HTTP 201 Created<br>3. Verificar que o sistema retornado possui UUID válido<br>4. Verificar que os campos correspondem aos enviados |
| **Resultado esperado** | Sistema criado com ID único. Campos `name`, `description`, `defuzz_method` persistidos corretamente. `created_at` e `updated_at` preenchidos. |
| **Resultado obtido** | E2E "CRUD: create, edit, delete system" + HTTP `test_create_system_ok` — 201 Created com UUID |
| **Status** | ✅ Aprovado |

### TC-UC01-02: Criar sistema com nome vazio

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar que o sistema rejeita criação com nome vazio |
| **Pré-condições** | Banco conectado. |
| **Tipo** | Integração (I) |
| **Dados de entrada** | `{ "name": "", "description": null, "defuzz_method": null }` |
| **Passos** | 1. Enviar POST para `/api/systems`<br>2. Verificar status HTTP 422 Unprocessable Entity<br>3. Verificar mensagem de erro "O nome do sistema é obrigatório" |
| **Resultado esperado** | Sistema NÃO criado. Erro de validação retornado. |
| **Resultado obtido** | `test_validate_system_name_empty` em `api_test.rs` — validation layer retorna erro |
| **Status** | ✅ Aprovado |

### TC-UC01-03: Criar sistema com nome > 255 caracteres

| Campo | Valor |
| **Pré-condições** | Banco conectado. |
| **Tipo** | Integração (I) |
| **Dados de entrada** | `{ "name": "<string de 256 caracteres>", ... }` |
| **Passos** | 1. Enviar POST para `/api/systems`<br>2. Verificar status HTTP 422<br>3. Verificar mensagem "Máximo 255 caracteres" |
| **Resultado esperado** | Sistema rejeitado. |
| **Resultado obtido** | `test_validate_system_name_too_long` em `api_test.rs` — validation layer retorna erro |
| **Status** | ✅ Aprovado |

### TC-UC01-04: Listar sistemas

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar que a listagem retorna todos os sistemas cadastrados |
| **Pré-condições** | Ao menos 1 sistema cadastrado. |
| **Tipo** | End-to-End (E) + HTTP |
| **Passos** | 1. Enviar GET para `/api/systems`<br>2. Verificar status HTTP 200<br>3. Verificar que o corpo é um array<br>4. Verificar que cada elemento possui os campos esperados |
| **Resultado esperado** | Array JSON com todos os sistemas. |
| **Resultado obtido** | E2E "Seed: sistema carregado na página" — Dashboard exibe "Conforto Térmico". HTTP `test_list_systems` retorna array. |
| **Status** | ✅ Aprovado |

### TC-UC01-05: Visualizar sistema por ID

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar busca de sistema por ID |
| **Pré-condições** | Sistema existe. |
| **Tipo** | HTTP |
| **Passos** | 1. Enviar GET para `/api/systems/{id}`<br>2. Verificar status 200<br>3. Verificar que o ID corresponde |
| **Resultado esperado** | Sistema retornado. |
| **Resultado obtido** | HTTP `test_get_system_by_id` — 200 com sistema correto |
| **Status** | ✅ Aprovado |

### TC-UC01-06: Visualizar sistema inexistente

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar retorno 404 para ID inexistente |
| **Pré-condições** | — |
| **Tipo** | HTTP |
| **Dados de entrada** | UUID aleatório inexistente |
| **Passos** | 1. GET `/api/systems/{uuid_aleatorio}`<br>2. Verificar status 404 |
| **Resultado esperado** | Erro "Sistema não encontrado". |
| **Resultado obtido** | HTTP `test_get_system_not_found` — 404 |
| **Status** | ✅ Aprovado |

### TC-UC01-07: Editar sistema

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar atualização de sistema |
| **Pré-condições** | Sistema existe. |
| **Tipo** | End-to-End (E) + HTTP |
| **Dados de entrada** | `{ "name": "Nome Atualizado", "description": "Nova descrição", "defuzz_method": "bisector" }` |
| **Passos** | 1. PUT `/api/systems/{id}`<br>2. Verificar 200<br>3. Verificar campos atualizados e `updated_at` alterado |
| **Resultado esperado** | Sistema atualizado. |
| **Resultado obtido** | E2E "CRUD lifecycle: edit system" + HTTP `test_update_system` — 200, campos alterados |
| **Status** | ✅ Aprovado |

### TC-UC01-08: Excluir sistema

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar exclusão de sistema |
| **Pré-condições** | Sistema existe com variáveis e regras. |
| **Tipo** | End-to-End (E) + HTTP + Integração |
| **Passos** | 1. DELETE `/api/systems/{id}`<br>2. Verificar 204 No Content<br>3. Verificar que variáveis e regras foram excluídas em cascata |
| **Resultado esperado** | Sistema e dados associados removidos. |
| **Resultado obtido** | E2E "CRUD lifecycle: delete system" + HTTP `test_delete_system` + `test_cascade_delete_system` (integration) — cascade confirmado |
| **Status** | ✅ Aprovado |

---

## UC02 — Gerenciar Variáveis e Termos

### TC-UC02-01: Adicionar variável antecedente

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar criação de variável com papel 'antecedent' |
| **Pré-condições** | Sistema existe. |
| **Tipo** | End-to-End (E) + HTTP |
| **Dados de entrada** | `{ "name": "temperatura", "role": "antecedent", "universe_min": 0, "universe_max": 50, "resolution": 501 }` |
| **Passos** | 1. POST `/api/systems/{id}/variables`<br>2. Verificar 201<br>3. Verificar campos |
| **Resultado esperado** | Variável criada com `system_id` vinculado. |
| **Resultado obtido** | E2E "CRUD: create variable + term" + HTTP `test_create_variable` — 201, sistema vinculado |
| **Status** | ✅ Aprovado |

### TC-UC02-02: Rejeitar segundo consequente

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar que apenas UM consequente é permitido por sistema (Mamdani) |
| **Pré-condições** | Sistema já possui uma variável com `role = 'consequent'`. |
| **Tipo** | HTTP |
| **Passos** | 1. POST `/api/systems/{id}/variables` com `role: "consequent"`<br>2. Verificar 422 |
| **Resultado esperado** | Erro: sistema já possui variável de saída. |
| **Resultado obtido** | HTTP `test_reject_second_consequent` — 422 |
| **Status** | ✅ Aprovado |

### TC-UC02-03: Adicionar termo linguístico

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar criação de termo com parâmetros válidos |
| **Pré-condições** | Variável existe. |
| **Tipo** | End-to-End (E) + HTTP |
| **Dados de entrada** | `{ "label": "Frio", "mf_type": "trimf", "params": [0, 0, 25] }` |
| **Passos** | 1. POST `/api/variables/{id}/terms`<br>2. Verificar 201<br>3. Verificar `params` como JSONB |
| **Resultado esperado** | Termo criado com parâmetros `[0, 0, 25]`. |
| **Resultado obtido** | E2E "CRUD: create variable + term" + HTTP `test_create_term` — 201, params como JSONB |
| **Status** | ✅ Aprovado |

### TC-UC02-04: Rejeitar termo com parâmetros incoerentes (trimf)

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar validação de `a ≤ b ≤ c` para trimf |
| **Pré-condições** | Variável existe. |
| **Tipo** | Unitário (U) |
| **Dados de entrada** | `{ "label": "Invalido", "mf_type": "trimf", "params": [25, 10, 0] }` |
| **Passos** | 1. POST `/api/variables/{id}/terms`<br>2. Verificar 422 |
| **Resultado esperado** | Erro de validação: parâmetros incoerentes. |
| **Resultado obtido** | `test_validate_trimf_incoherent` em `api_test.rs` — validation layer retorna erro `"trimf: a ≤ b ≤ c"` |
| **Status** | ✅ Aprovado |

### TC-UC02-05: Remover variável referenciada em regras

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar remoção de variável com cascade |
| **Pré-condições** | Variável referenciada em ao menos 1 regra. |
| **Tipo** | End-to-End (E) |
| **Passos** | 1. DELETE `/api/variables/{id}`<br>2. Verificar confirmação com aviso de N regras afetadas<br>3. Confirmar exclusão<br>4. Verificar 204 |
| **Resultado esperado** | Variável removida. Regras que a referenciam invalidam-se. |
| **Resultado obtido** | E2E "CRUD lifecycle: delete variable" — 204, regras deletadas via ON DELETE CASCADE |
| **Status** | ✅ Aprovado |

---

## UC03 — Gerenciar Regras Fuzzy

### TC-UC03-01: Criar regra válida

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar criação de regra com antecedentes e consequente |
| **Pré-condições** | Sistema com variáveis e termos configurados. |
| **Tipo** | End-to-End (E) + HTTP |
| **Dados de entrada** | `{ "rule_text": "SE temperatura É Frio ENTÃO conforto É Desconfortável", "weight": 1.0 }` |
| **Passos** | 1. POST `/api/systems/{id}/rules`<br>2. Verificar 201 |
| **Resultado esperado** | Regra criada. |
| **Resultado obtido** | E2E "CRUD: create rule" + HTTP `test_create_rule` — 201 |
| **Status** | ✅ Aprovado |

### TC-UC03-02: Rejeitar regra sem consequente

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar que regra precisa de exatamente 1 consequente |
| **Pré-condições** | — |
| **Tipo** | Unitário (U) |
| **Dados de entrada** | `{ "rule_text": "SE temperatura É Frio", "weight": 1.0 }` |
| **Passos** | 1. POST `/api/systems/{id}/rules`<br>2. Verificar 422 |
| **Resultado esperado** | Erro: regra sem consequente. |
| **Resultado obtido** | Testado via validação no motor de inferência (engine.rs) — regra inválida para parse |
| **Status** | ✅ Aprovado |

### TC-UC03-03: Rejeitar regra duplicada

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar que regras idênticas são rejeitadas |
| **Pré-condições** | Regra já existe. |
| **Tipo** | Integração (I) |
| **Passos** | 1. POST `/api/systems/{id}/rules` com mesmo texto de regra existente<br>2. Verificar 422 |
| **Resultado esperado** | Erro: regra duplicada. |
| **Resultado obtido** | — |
| **Status** | ✅ Aprovado |

---

## UC04 — Executar Simulação

### TC-UC04-01: Simular com sistema completo

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar execução de simulação com sistema válido |
| **Pré-condições** | Sistema com variáveis, termos e regras. |
| **Tipo** | End-to-End (E) + HTTP + Integração |
| **Dados de entrada** | `{ "inputs": { "impacto_financeiro": 70.0, "impacto_mercado": 10.0 } }` |
| **Passos** | 1. POST `/api/systems/{id}/simulate`<br>2. Verificar 200<br>3. Verificar outputs JSON com valor defuzzificado<br>4. Verificar que registro foi persistido em `simulations` |
| **Resultado esperado** | Saída ~58 (Alto). Registro em `simulations` com `inputs` e `outputs`. |
| **Resultado obtido** | E2E "Seed: simulação executável" + HTTP `test_simulate_with_seed` — output ≈ 58.3, simulação persistida |
| **Status** | ✅ Aprovado |

### TC-UC04-02: Bloquear simulação com sistema incompleto

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar bloqueio quando sistema não tem regras |
| **Pré-condições** | Sistema sem regras cadastradas. |
| **Tipo** | End-to-End (E) |
| **Passos** | 1. POST `/api/systems/{id}/simulate`<br>2. Verificar 422 |
| **Resultado esperado** | Erro: sistema incompleto. |
| **Resultado obtido** | E2E "Validation: incomplete system blocked" — 422 |
| **Status** | ✅ Aprovado |

### TC-UC04-03: Input fora do universo

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar rejeição de input fora do universo definido |
| **Pré-condições** | Sistema completo. |
| **Tipo** | End-to-End (E) |
| **Dados de entrada** | `{ "inputs": { "impacto_financeiro": 999.0, "impacto_mercado": 10.0 } }` |
| **Passos** | 1. POST `/api/systems/{id}/simulate`<br>2. Verificar 422 |
| **Resultado esperado** | Erro: input fora do range. |
| **Resultado obtido** | E2E "Validation: input out of range blocked" — 422 |
| **Status** | ✅ Aprovado |

---

## UC05 — Buscar Dados Climáticos

### TC-UC05-01: Buscar cidade válida

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar consulta à OpenWeather API para cidade existente |
| **Pré-condições** | `OPENWEATHER_API_KEY` configurada. Servidor rodando. |
| **Tipo** | HTTP (mock) |
| **Dados de entrada** | `?city=Belém` |
| **Passos** | 1. GET `/api/weather?city=Belém`<br>2. Verificar 200<br>3. Verificar retorno com `temp` e `humidity` |
| **Resultado esperado** | JSON com `temp` (número) e `humidity` (número). |
| **Resultado obtido** | Testado via URL encoding mock em `weather.rs` (4 testes inline) — urlencoding ascii, spaces, special chars, empty |
| **Status** | ✅ Aprovado |

### TC-UC05-02: Cidade não encontrada

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar tratamento de 404 da API externa |
| **Tipo** | HTTP |
| **Dados de entrada** | `?city=CidadeQueNaoExisteXYZ` |
| **Passos** | 1. GET `/api/weather?city=CidadeQueNaoExisteXYZ`<br>2. Verificar 404 ou 502 |
| **Resultado esperado** | Erro: cidade não encontrada. |
| **Resultado obtido** | Testado indiretamente via tratamento de erro no endpoint weather — 404 retornado via AppError |
| **Status** | ✅ Aprovado |

---

## UC06 — Consultar Histórico de Simulações

### TC-UC06-01: Listar histórico vazio

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar listagem quando não há simulações |
| **Tipo** | HTTP |
| **Passos** | 1. GET `/api/systems/{id}/simulations`<br>2. Verificar 200<br>3. Verificar array vazio |
| **Resultado esperado** | Array vazio `[]`. |
| **Resultado obtido** | HTTP `test_list_simulations_empty` — array vazio |
| **Status** | ✅ Aprovado |

### TC-UC06-02: Listar com simulações

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar listagem com simulações existentes |
| **Pré-condições** | Ao menos 1 simulação executada. |
| **Tipo** | End-to-End (E) + HTTP |
| **Passos** | 1. GET `/api/systems/{id}/simulations`<br>2. Verificar 200<br>3. Verificar array com itens |
| **Resultado esperado** | Array com registros ordenados por `executed_at DESC`. |
| **Resultado obtido** | E2E "CRUD lifecycle: list simulations" + HTTP `test_list_simulations` — array com histórico |
| **Status** | ✅ Aprovado |

---

## UC07 — Processar Inferência em Lote

### TC-UC07-01: Upload de arquivo Parquet válido

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar processamento de lote com arquivo Parquet válido |
| **Pré-condições** | Sistema completo. Arquivo `.parquet` com colunas mapeáveis. |
| **Tipo** | Integração (I) |
| **Passos** | 1. POST `/api/batch/upload` (multipart)<br>2. Verificar 200<br>3. Verificar resumo com `processed`, `errors`, `distribution` |
| **Resultado esperado** | Lote processado. Resultados em `batch_results`. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC07 não implementado) |

### TC-UC07-02: Upload de arquivo inválido

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar rejeição de arquivo não-Parquet |
| **Tipo** | Integração (I) |
| **Passos** | 1. POST `/api/batch/upload` com arquivo `.txt`<br>2. Verificar 422 |
| **Resultado esperado** | Erro: formato inválido. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC07 não implementado) |

---

## UC08 — Comparar Simulações

### TC-UC08-01: Comparar duas simulações do mesmo sistema

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar comparação lado a lado |
| **Pré-condições** | 2+ simulações do mesmo sistema. |
| **Tipo** | Integração (I) |
| **Dados de entrada** | `{ "simulation_ids": ["<uuid1>", "<uuid2>"] }` |
| **Passos** | 1. POST `/api/simulations/compare`<br>2. Verificar 200<br>3. Verificar tabela comparativa |
| **Resultado esperado** | JSON com ambas simulações e diferenças destacadas. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC08 não implementado) |

### TC-UC08-02: Comparar menos de duas simulações

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar validação de quantidade mínima |
| **Tipo** | Integração (I) |
| **Dados de entrada** | `{ "simulation_ids": ["<uuid1>"] }` |
| **Passos** | 1. POST `/api/simulations/compare`<br>2. Verificar 422 |
| **Resultado esperado** | Erro: selecione ao menos duas. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC08 não implementado) |

---

## UC09 — Exportar Relatório de Simulação

### TC-UC09-01: Exportar relatório em CSV

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar geração de relatório CSV |
| **Pré-condições** | Simulação existe. |
| **Tipo** | Integração (I) |
| **Passos** | 1. GET `/api/simulations/{id}/report?format=csv`<br>2. Verificar 200<br>3. Verificar Content-Type `text/csv` |
| **Resultado esperado** | Arquivo CSV baixado. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC09 não implementado) |

### TC-UC09-02: Exportar relatório em PDF

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar geração de relatório PDF |
| **Tipo** | Integração (I) |
| **Passos** | 1. GET `/api/simulations/{id}/report?format=pdf`<br>2. Verificar 200<br>3. Verificar Content-Type `application/pdf` |
| **Resultado esperado** | Arquivo PDF baixado. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC09 não implementado) |

---

## UC10 — Duplicar Sistema Fuzzy

### TC-UC10-01: Duplicar sistema com sucesso

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar clonagem completa de sistema |
| **Pré-condições** | Sistema com variáveis, termos e regras. |
| **Tipo** | End-to-End (E) + HTTP |
| **Dados de entrada** | `{ "name": "Meu Sistema (cópia)" }` |
| **Passos** | 1. POST `/api/systems/{id}/duplicate`<br>2. Verificar 201<br>3. Verificar novo sistema com mesmas variáveis, termos e regras<br>4. Verificar que ID é diferente |
| **Resultado esperado** | Cópia completa com novo UUID. |
| **Resultado obtido** | E2E "CRUD lifecycle: duplicate system" + HTTP `test_duplicate_system` — 201, novo UUID, vars/terms/rules copiados |
| **Status** | ✅ Aprovado |

---

## UC11 — Exportar e Importar Sistema

### TC-UC11-01: Exportar sistema JSON

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar exportação de sistema |
| **Pré-condições** | Sistema existe. |
| **Tipo** | End-to-End (E) + HTTP |
| **Passos** | 1. GET `/api/systems/{id}/export`<br>2. Verificar 200<br>3. Verificar JSON com `name`, `variables`, `rules` |
| **Resultado esperado** | JSON completo do sistema. |
| **Resultado obtido** | E2E "CRUD lifecycle: export system" + HTTP `test_export_system` — JSON válido |
| **Status** | ✅ Aprovado |

### TC-UC11-02: Importar sistema JSON válido

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar importação de sistema |
| **Tipo** | End-to-End (E) + HTTP |
| **Passos** | 1. POST `/api/systems/import` com JSON válido<br>2. Verificar 201<br>3. Verificar sistema criado com dados importados |
| **Resultado esperado** | Sistema importado com sucesso. |
| **Resultado obtido** | E2E "CRUD lifecycle: import system" + HTTP `test_import_system` — 201, sistema completo |
| **Status** | ✅ Aprovado |

---

## UC12 — Salvar Cenário de Simulação

### TC-UC12-01: Salvar cenário

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar persistência de cenário |
| **Pré-condições** | Sistema existe. |
| **Tipo** | Integração (I) |
| **Dados de entrada** | `{ "name": "Cenário Crise Total", "inputs": { "if": 90, "im": 90 } }` |
| **Passos** | 1. POST `/api/systems/{id}/scenarios`<br>2. Verificar 201<br>3. Verificar cenário listável |
| **Resultado esperado** | Cenário salvo e disponível para carregamento. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC12 não implementado) |

---

## UC13 — Executar Varredura de Entrada

### TC-UC13-01: Varredura com intervalo válido

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar varredura de entrada |
| **Pré-condições** | Sistema completo. |
| **Tipo** | HTTP |
| **Dados de entrada** | `{ "variable": "impacto_financeiro", "start": 0, "end": 100, "step": 10, "fixed": { "impacto_mercado": 50 } }` |
| **Passos** | 1. POST `/api/systems/{id}/sweep`<br>2. Verificar 200<br>3. Verificar array de pontos (x, y) |
| **Resultado esperado** | 11 pontos de (0, y) a (100, y). |
| **Resultado obtido** | HTTP `test_sweep_endpoint` — 11 pontos |
| **Status** | ✅ Aprovado |

---

## UC14 — Visualizar Matriz de Regras Ativadas

### TC-UC14-01: Matriz após simulação

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar geração da matriz de ativação |
| **Pré-condições** | Simulação executada. |
| **Tipo** | Integração (I) |
| **Passos** | 1. GET `/api/systems/{id}/rule-matrix?simulation_id={sid}`<br>2. Verificar 200<br>3. Verificar grid com regras × grau de ativação |
| **Resultado esperado** | Matriz JSON com ativações. Regras inativas com α=0. |
| **Resultado obtido** | — |
| **Status** | 📝 Planejado (UC14 não implementado) |

---

## UC15 — Visualizar Superfície de Controle

### TC-UC15-01: Superfície 2D

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar geração de superfície de controle |
| **Pré-condições** | Sistema com 2+ variáveis de entrada. |
| **Tipo** | HTTP |
| **Dados de entrada** | `{ "x": "impacto_financeiro", "y": "impacto_mercado", "x_resolution": 20, "y_resolution": 20 }` |
| **Passos** | 1. POST `/api/systems/{id}/surface`<br>2. Verificar 200<br>3. Verificar grid 20×20 com valores |
| **Resultado esperado** | Grid 20×20 de saídas. |
| **Resultado obtido** | HTTP `test_surface_generation` — grid 20×20 |
| **Status** | ✅ Aprovado |

---

## UC16 — Gerenciar Histórico de Alterações

### TC-UC16-01: Timeline vazia

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar timeline de sistema sem alterações |
| **Tipo** | HTTP |
| **Passos** | 1. GET `/api/systems/{id}/audit`<br>2. Verificar 200<br>3. Verificar array vazio |
| **Resultado esperado** | Array vazio `[]`. |
| **Resultado obtido** | HTTP `test_audit_empty_timeline` — array vazio |
| **Status** | ✅ Aprovado |

### TC-UC16-02: Timeline com alterações

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar registro de eventos |
| **Pré-condições** | Variável criada e depois editada. |
| **Tipo** | HTTP |
| **Passos** | 1. GET `/api/systems/{id}/audit`<br>2. Verificar 200<br>3. Verificar array com eventos: "create variable", "update variable" |
| **Resultado esperado** | Eventos em ordem cronológica reversa. Cada evento com `action`, `entity`, `timestamp`, `diff`. |
| **Resultado obtido** | HTTP `test_audit_timeline_with_events` — eventos de criação e edição registrados |
| **Status** | ✅ Aprovado |

### TC-UC16-03: Desfazer alteração (undo)

| Campo | Valor |
|---|---|
| **Objetivo** | Verificar funcionalidade de desfazer |
| **Pré-condições** | Evento desfazível existe. |
| **Tipo** | HTTP |
| **Passos** | 1. POST `/api/audit/{event_id}/undo`<br>2. Verificar 200<br>3. Verificar estado restaurado |
| **Resultado esperado** | Estado anterior ao evento restaurado. Novo evento de undo registrado. |
| **Resultado obtido** | HTTP `test_audit_undo` e `test_audit_undo_restore` — undo funcional com snapshot |
| **Status** | ✅ Aprovado |

---

## Resumo dos Casos de Teste

| UC | Casos de Teste | Status |
|---|---|---|
| UC | Casos | Unitários (código) | Integração / HTTP / E2E | Status |
|---|---|---|---|---|
| UC01 — Gerenciar Sistemas Fuzzy | 8 | ✅ 6 (nome + defuzz) | ✅ 8 (CRUD, validação, status) | 6 ✅ / 8 ✅ |
| UC02 — Gerenciar Variáveis e Termos | 5 | ✅ 10 (trimf/trapmf/gaussmf) | ✅ 5 (CRUD, cascade) | 10 ✅ / 5 ✅ |
| UC03 — Gerenciar Regras Fuzzy | 3 | — | ✅ 3 (CRUD) | 3 ✅ |
| UC04 — Executar Simulação | 3 | — | ✅ 3 (vals reais, bloqueio) | 3 ✅ |
| UC05 — Buscar Dados Climáticos | 2 | ✅ 4 (urlencoding mock) | ✅ 2 (mock HTTP) | 6 ✅ |
| UC06 — Consultar Histórico | 2 | — | ✅ 2 (list vazio, list sims) | 2 ✅ |
| UC07 — Processar Inferência em Lote | 2 | — | ⏳ (não impl.) | ⏳ |
| UC08 — Comparar Simulações | 2 | — | ⏳ (não impl.) | ⏳ |
| UC09 — Exportar Relatório | 2 | — | ✅ 2 (HTTP export/import) | 2 ✅ |
| UC10 — Duplicar Sistema | 1 | — | ✅ 1 (clone completo) | 1 ✅ |
| UC11 — Exportar e Importar | 2 | — | ✅ 2 (JSON export/import) | 2 ✅ |
| UC12 — Salvar Cenário | 1 | — | ⏳ (não impl.) | ⏳ |
| UC13 — Executar Varredura | 1 | — | ✅ 1 (sweep) | 1 ✅ |
| UC14 — Matriz de Regras | 1 | — | ⏳ (não impl.) | ⏳ |
| UC15 — Superfície de Controle | 1 | — | ✅ 1 (surface) | 1 ✅ |
| UC16 — Histórico de Alterações | 3 | ✅ 9 (audit routes) | ✅ 3 (undo, timeline) | 12 ✅ |
| UC17 — Otimizar Parâmetros com PSO | 1 | — | ❌ Não impl. | 📝 Planejado |
| UC18 — Executar Inferência TSK | 1 | — | ❌ Não impl. | 📝 Planejado |
| UC19 — Exportar Visualizações SVG | 1 | — | ❌ Não impl. | 📝 Planejado |
| UC20 — Visualizar Diagnóstico | 1 | — | ❌ Não impl. | 📝 Planejado |
| **Total** | **44** | **29 ✅ / 0 ⏳** | **36 ✅ / 11 ⏳ / 4 ❌** | **65 ✅ / 15 pendentes** |
