# Mapa de Testes — FuzzySimulated

> Documentação detalhada de todos os 161 testes do projeto: o que cada um testa, por que testa (caso de uso), e o que o faz falhar.
> Cada seção contém uma **tabela** para consulta rápida seguida de **parágrafos explicativos** para entendimento profundo.

---

## Tests Unitários (inline — 43)

Testes definidos com `#[test]` dentro de `server/src/`. Não dependem de banco ou servidor — rodam puramente em memória.

### engine.rs (14) — UC04 (Executar Simulação)

**Tabela resumo:**

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_trimf_peak | UC04 | membership(25, trimf[0,25,50]) = 1.0 | Algoritmo trimf quebrado — cálculo do pico errado |
| test_trimf_left_edge | UC04 | membership(0, trimf[0,25,50]) = 0.0 | Rampa esquerda não calcula zero na borda |
| test_trimf_right_edge | UC04 | membership(50, trimf[0,25,50]) = 0.0 | Rampa direita não calcula zero na borda |
| test_trimf_linear_rise | UC04 | membership(12.5, trimf[0,25,50]) = 0.5 | Interpolação linear da subida incorreta |
| test_trimf_linear_fall | UC04 | membership(37.5, trimf[0,25,50]) = 0.5 | Interpolação linear da descida incorreta |
| test_trapmf_plateau | UC04 | membership(30, trapmf[0,20,40,60]) = 1.0 | Platô do trapmf não retorna 1 |
| test_trapmf_left_ramp | UC04 | membership(10, trapmf[0,20,40,60]) = 0.5 | Rampa esquerda do trapmf incorreta |
| test_trapmf_right_ramp | UC04 | membership(50, trapmf[0,20,40,60]) = 0.5 | Rampa direita do trapmf incorreta |
| test_trapmf_outside | UC04 | membership(-1, trapmf[0,20,40,60]) = 0.0 | Extremidade negativa não tratada |
| test_gaussmf_peak | UC04 | membership(50, gaussmf[50,10]) = 1.0 | Função gaussiana — centro não retorna 1 |
| test_gaussmf_one_sigma | UC04 | membership(60, gaussmf[50,10]) = exp(-0.5) | Desvio padrão gaussiano incorreto |
| test_parse_simple_rule | UC04 | Parse "SE Temp = Alta ENTAO Risco = Alto" → 2 condições | Parser não reconhece sintaxe "=" |
| test_parse_portuguese_rule | UC04 | Parse "SE temp é frio E umidade é seco..." → 3 condições | Parser não reconhece "é" ou "E" (português) |
| test_mamdani_basic_inference | UC04 | Mamdani completo: input 80° → Risco > 0 | Motor de inferência não agrega regras |

**Detalhamento por teste:**

**test_trimf_peak**: Verifica que uma função de pertinência triangular retorna 1.0 exatamente no centro (b). Se o cálculo da rampa for incorreto — por exemplo, divisão por zero ou lógica de interpolação errada — o valor do pico desvia de 1.0. A UC04 exige que membership seja calculada corretamente, pois é o bloco fundamental da fuzzificação.

**test_trimf_left_edge** / **test_trimf_right_edge**: Verificam que as bordas da função trimf retornam 0.0. A rampa esquerda deve ir de 0 (em a) a 1 (em b); a direita de 1 (em b) a 0 (em c). Se a implementação tratar valores exatamente nas bordas como NaN ou extrapolar, esses testes quebram.

**test_trimf_linear_rise** / **test_trimf_linear_fall**: Verificam a interpolação linear exata no ponto médio de cada rampa. 12.5 está a meio caminho entre a (0) e b (25) → grau 0.5. 37.5 está a meio caminho entre b (25) e c (50) → 0.5. Falham se a interpolação usar fórmula incorreta (ex: (x-a)/(c-a) em vez de (x-a)/(b-a)).

**test_trapmf_plateau**: Função trapezoidal tem platô entre b e c onde membership é sempre 1.0. Se o algoritmo for implementado como trimf (que só tem 1 ponto), este teste falha.

**test_trapmf_left_ramp** / **test_trapmf_right_ramp**: Verificam as rampas do trapmf. left_ramp testa x=10 no intervalo [0,20] (meio da rampa → 0.5). right_ramp testa x=50 no intervalo [40,60] (meio da rampa → 0.5).

**test_trapmf_outside**: Garante que valores fora do domínio [a,d] retornam 0. Com x=-1 < a=0, a membership deve ser zero. Falha se a função não proteger contra valores negativos.

**test_gaussmf_peak**: Gaussiana centrada em 50 com sigma 10: membership(50) = exp(-0²/2·10²) = exp(0) = 1. Falha se o denominador usar sigma² em vez de 2·sigma².

**test_gaussmf_one_sigma**: membership(60) com µ=50, σ=10 → exp(-(10)²/(2·100)) = exp(-0.5). Falha se a fórmula gaussiana estiver incorreta.

**test_parse_simple_rule**: Verifica o parser de regras no formato "SE var = termo ENTAO var = termo". O parser deve quebrar a string em condições (par var → termo). Falha se o parser não suportar o operador "=".

**test_parse_portuguese_rule**: Verifica o parser com palavras em português: "é" em vez de "=", "E" como conector, "ENTÃO" como separador. Essencial para usabilidade brasileira. Falha se o parser não tratar sinônimos.

**test_mamdani_basic_inference**: Teste de integração do motor Mamdani: cria 1 regra, 2 variáveis (1 antecedente, 1 consequente), 2 termos, e executa inferência com input 80° → espera Risco entre 0 e 1 exclusivo. Falha se o pipeline fuzzificação → agregação → defuzzificação tiver qualquer bug.

### errors.rs (4) — UC01 (Gerenciar Sistemas)

**Tabela resumo:**

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_validation_status | UC01 | AppError::Validation → 422 | Mapeamento AppError → HTTP status quebrado |
| test_not_found_status | UC01 | AppError::NotFound → 404 | Mapeamento de NotFound incorreto |
| test_database_status | UC01 | AppError::Database → 500 | Erro de banco não retorna 500 |
| test_external_status | UC01 | AppError::External → 502 | Erro de API externa não retorna 502 |

**Detalhamento:**

Validam o mapeamento de `AppError` para códigos HTTP (trait `IntoResponse`). Cada variante do enum `AppError` deve produzir o status correto: Validation → 422, NotFound → 404, Database → 500, External → 502. Falham se alguém alterar o `impl IntoResponse` sem atualizar os status — por exemplo, mudar NotFound de 404 para 400.

### audit_routes.rs (9) — UC16 (Histórico de Alterações)

**Tabela resumo:**

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_entity_table_system | UC16 | "system" → "fuzzy_systems" | Mapa entidade→tabela SQL incorreto |
| test_entity_table_variable | UC16 | "variable" → "fuzzy_variables" | Idem |
| test_entity_table_term | UC16 | "term" → "fuzzy_terms" | Idem |
| test_entity_table_rule | UC16 | "rule" → "fuzzy_rules" | Idem |
| test_entity_table_unknown | UC16 | "invalid" → Err | Entidade desconhecida não é rejeitada |
| test_snapshot_object_fields_extracts_keys | UC16 | json!({"id","name","value"}) → "id, name, value" | Função helper snapshot não extrai chaves |
| test_snapshot_object_fields_non_object | UC16 | string/number/null → "" | Não-string causa panic |
| test_snapshot_object_fields_empty | UC16 | {} → "" | Objeto vazio não tratado |

**Detalhamento:**

A função `entity_table()` mapeia nomes de entidade (ex: "system") para tabelas SQL (ex: "fuzzy_systems"). Essencial para o sistema de auditoria construir queries dinâmicas de snapshot. Os 5 primeiros testes verificam cada entidade e o caso de erro. Falham se novas entidades forem adicionadas sem atualizar o match.

`snapshot_object_fields()` extrai chaves de um JSON object para logging de diff. Três testes cobrem: object normal, não-object (string, número, null), e object vazio. Falha se houver panic em tipos inesperados.

### weather.rs (4) — UC05 (Buscar Dados Climáticos)

**Tabela resumo:**

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_urlencoding_ascii | UC05 | "hello" → "hello" | Codificador altera ASCII puro |
| test_urlencoding_with_spaces | UC05 | "São Paulo" → "S%C3%A3o%20Paulo" | Acentuação ou espaços mal codificados |
| test_urlencoding_special_chars | UC05 | "a&b=c" → "a%26b%3Dc" | Caracteres especiais não escapados |
| test_urlencoding_empty | UC05 | "" → "" | String vazia causa erro |

**Detalhamento:**

Testam a função `urlencoding()` que prepara nomes de cidade para a OpenWeather API. O encoding correto é essencial porque a API recebe o nome na URL. "São Paulo" precisa virar "S%C3%A3o%20Paulo" (UTF-8 percent-encoded). Falham se a implementação usar encoding ASCII em vez de UTF-8, ou se não escapar caracteres como & e =.

---

## Tests Unitários (tests/unit/ — 19)

Testes em arquivos separados sob `tests/unit/`. Testam funções de validação sem dependências externas.

### mf_validation.rs (12) — UC02 (Gerenciar Variáveis e Termos)

**Tabela resumo:**

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_validate_trimf_ok | UC02 | trimf [0,10,12] é aceito | Rejeita parâmetros válidos |
| test_validate_trimf_non_finite | UC02 | NaN/Inf em trimf → Err | Aceita NaN ou Infinity |
| test_validate_trimf_shoulder | UC02 | trimf [0,0,25] e [25,50,50] aceitos | Rejeita ombro (a=b ou b=c) |
| test_validate_trimf_incoherent | UC02 | trimf [22,10,0] → Err | Aceita parâmetros fora de ordem |
| test_validate_trimf_wrong_params | UC02 | trimf com 2 ou 4 params → Err | Aceita número errado de parâmetros |
| test_validate_trapmf_ok | UC02 | trapmf [0,0,20,40] e shoulder aceitos | Rejeita parâmetros válidos |
| test_validate_trapmf_incoherent | UC02 | trapmf [40,20,0,0] → Err | Aceita parâmetros fora de ordem |
| test_validate_gaussmf_ok | UC02 | gaussmf [50,15] aceito | Rejeita parâmetros válidos |
| test_validate_gaussmf_zero_sigma | UC02 | gaussmf [50,0] → Err | Aceita sigma = 0 (divisão por zero) |
| test_validate_gaussmf_negative_sigma | UC02 | gaussmf [50,-1] → Err | Aceita sigma negativo |
| test_validate_gaussmf_wrong_params | UC02 | gaussmf com 1 ou 3 params → Err | Aceita número errado de parâmetros |

**Detalhamento:**

Testam as funções de validação de funções de pertinência em `validation.rs`. Cada MF tem regras específicas:
- **trimf**: exatamente 3 params, a ≤ b ≤ c, todos finitos. Ombro (a=b ou b=c) é permitido.
- **trapmf**: exatamente 4 params, a ≤ b ≤ c ≤ d, todos finitos.
- **gaussmf**: exatamente 2 params, ambos finitos, sigma > 0 (evita divisão por zero).

Os testes de `_non_finite` foram adicionados na fase de correções finais (25/05/2026) para cobrir NaN e Infinity, que podiam causar panics silenciosos no motor.

Falham se a validação for alterada para ser mais permissiva ou mais restritiva sem alinhamento com o modelo de dados.

### system_validation.rs (7) — UC01 (Gerenciar Sistemas)

**Tabela resumo:**

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_validate_system_name_ok | UC01 | "Conforto Térmico" → Ok | Rejeita nome válido |
| test_validate_system_name_empty | UC01 | "" → Err | Aceita nome vazio |
| test_validate_system_name_whitespace | UC01 | "   " → Err | Aceita só espaços |
| test_validate_system_name_too_long | UC01 | 256 caracteres → Err | Aceita nome > 255 |
| test_validate_defuzz_method_valid | UC01 | centroid/bisector/mom/lom/som → Ok | Rejeita método válido |
| test_validate_defuzz_method_invalid | UC01 | "invalid" → Err | Aceita método inválido |

**Detalhamento:**

Testam a validação de sistema: nome e método de defuzzificação.
- Nome: não vazio, não só whitespace, ≤ 255 caracteres.
- Defuzz: apenas "centroid", "bisector", "mom", "lom", "som".

Falham se os limites de validação forem alterados (ex: aumentar o limite para 300 caracteres sem alinhar com o banco VARCHAR(255)) ou se novos métodos de defuzzificação forem adicionados sem atualizar a lista.

---

## Tests HTTP Axum (65)

Testes que levantam um servidor Axum de teste com banco PostgreSQL real. Usam `serial_test::serial` para evitar concorrência entre si e `#[ignore]` removido para execução normal. Usam helpers `TestApp`, `json_get`, `json_post`, `create_minimal_system` definidos em `common/mod.rs`.

### systems.rs (7) — UC01 (Gerenciar Sistemas)

| Nome | UC | O que testa |
|---|---|---|
| test_create_system | UC01 | POST `/api/systems` com nome + defuzz → 201 |
| test_list_systems | UC01 | GET `/api/systems` → 200 + array |
| test_get_system_by_id | UC01 | GET `/api/systems/{id}` → 200 + sistema |
| test_update_system | UC01 | PUT `/api/systems/{id}` → 200 + campos atualizados |
| test_delete_system | UC01 | DELETE `/api/systems/{id}` → 204 |
| test_update_system_status | UC01 | PUT `/api/systems/{id}/status` → 200 + status alterado |
| test_create_system_validation_error | UC01 | POST com nome inválido → 422 |
| test_system_not_found | UC01 | GET `/api/systems/{fake_id}` → 404 |

**Detalhamento:**

Cobrem o CRUD completo de sistemas fuzzy. `test_create_system` verifica que o endpoint aceita nome, descrição e método de defuzz, retornando 201 com UUID. `test_update_system_status` (UC01) verifica a transição entre estados ativo/favorito/concluido/desativado — essencial para proteger sistemas favoritados de exclusão acidental. Falham se as rotas forem alteradas, se a validação de middleware mudar, ou se o banco rejeitar inserts por constraints.

### variables.rs (7) — UC02 (Gerenciar Variáveis e Termos)

| Nome | UC | O que testa |
|---|---|---|
| test_create_variable | UC02 | POST `/api/systems/{id}/variables` → 201 |
| test_list_variables | UC02 | GET `/api/systems/{id}/variables` → 200 + array |
| test_get_variable | UC02 | GET `/api/variables/{id}` → 200 |
| test_update_variable | UC02 | PUT `/api/variables/{id}` → 200 |
| test_delete_variable | UC02 | DELETE `/api/variables/{id}` → 204 |
| test_variable_not_found | UC02 | GET `/api/variables/{fake_id}` → 404 |
| test_create_variable_validation_error | UC02 | POST com nome vazio → 422 |

**Detalhamento:**

Cobrem CRUD de variáveis. Validam que variáveis são criadas com role (antecedent/consequent), universo [min, max], e resolução. O teste `test_create_variable` cria primeiro um sistema e depois a variável — o helper `create_minimal_system` é reutilizado. Falham se o endpoint de criação mudar de payload ou se a validação de universo (min > max) for removida.

### terms.rs (5) — UC02 (Gerenciar Termos)

| Nome | UC | O que testa |
|---|---|---|
| test_create_term | UC02 | POST `/api/variables/{id}/terms` → 201 |
| test_get_term | UC02 | GET `/api/terms/{id}` → 200 |
| test_update_term | UC02 | PUT `/api/terms/{id}` → 200 |
| test_delete_term | UC02 | DELETE `/api/terms/{id}` → 204 |
| test_create_term_validation_error | UC02 | POST com params inválidos → 422 |

**Detalhamento:**

CRUD de termos linguísticos. O teste `test_create_term` envia label, mf_type e params como JSON — o banco armazena params como JSONB. `test_create_term_validation_error` testa a rejeição de parâmetros mal formatados (ex: trimf com 2 params). Falham se o banco não aceitar JSONB ou se a serialização de params mudar.

### rules.rs (5) — UC03 (Gerenciar Regras)

| Nome | UC | O que testa |
|---|---|---|
| test_create_rule | UC03 | POST `/api/systems/{id}/rules` → 201 |
| test_get_rule | UC03 | GET `/api/rules/{id}` → 200 |
| test_update_rule | UC03 | PUT `/api/rules/{id}` → 200 |
| test_delete_rule | UC03 | DELETE `/api/rules/{id}` → 204 |
| test_rule_not_found | UC03 | GET `/api/rules/{fake_id}` → 404 |

**Detalhamento:**

CRUD de regras fuzzy. O texto da regra é armazenado como string (ex: "SE temperatura = frio ENTAO conforto = confortavel"). O backend não parseia a regra no momento da criação — o parsing ocorre na simulação. Falham se o formato de `rule_text` for validado no backend e a validação for alterada.

### simulate.rs (12) — UC04, UC06, UC10, UC17-20

| Nome | UC | O que testa |
|---|---|---|
| test_simulate | UC04 | POST `/api/systems/{id}/simulate` → 200 + outputs |
| test_simulate_missing_input | UC04 | Input vazio → 200 ou 422 |
| test_list_simulations | UC06 | GET `/api/systems/{id}/simulations` → 200 + array |
| test_duplicate_system | UC10 | POST `/api/systems/{id}/duplicate` → 201 + nome copiado |
| test_simulate_tsk | UC18 | POST `/api/systems/{id}/simulate-tsk` → 200 + "method":"tsk" |
| test_simulate_tsk_system_not_found | UC18 | POST com fake_id → 404 |
| test_svg_export | UC19 | GET `/api/systems/{id}/svg` → 200 + array svgs |
| test_svg_export_system_not_found | UC19 | GET com fake_id → 404 |
| test_diagnostic | UC20 | POST `/api/systems/{id}/diagnostic` → 200 + fuzzification |
| test_diagnostic_system_not_found | UC20 | POST com fake_id → 404 |
| test_optimize_pso | UC17 | POST `/api/systems/{id}/optimize-pso` → 200 + best_position/fitness |
| test_optimize_pso_invalid_data | UC17 | POST com dados vazios → 200 ou 422 |

**Detalhamento:**

Os testes mais densos da suíte, cobrindo 6 UCs diferentes. 

`test_simulate` (UC04): cria sistema mínimo, executa simulação Mamdani, verifica outputs. Falha se o motor de inferência não retornar outputs ou se a rota mudar.

`test_list_simulations` (UC06): executa simulação e depois lista o histórico. Verifica que simulações são persistidas e recuperáveis. Falha se a query de listagem não filtrar por system_id.

`test_duplicate_system` (UC10): clona sistema existente com novo nome. Verifica que a cópia tem ID diferente. Falha se a lógica de clonagem não copiar todos os relacionamentos.

`test_simulate_tsk` (UC18): testa inferência Takagi-Sugeno-Kang, que usa coeficientes polinomiais em vez de defuzzificação. Falha se o motor TSK não for implementado ou retornar formato errado.

`test_svg_export` (UC19): gera SVGs das funções de pertinência. Falha se o template SVG estiver quebrado ou se o endpoint mudar de `/svg` para outro path.

`test_diagnostic` (UC20): relatório detalhado com fuzzificação, regras disparadas e saídas. Falha se o diagnóstico não incluir a seção "fuzzification".

`test_optimize_pso` (UC17): otimização por enxame de partículas. Falha se o algoritmo PSO não convergir ou se o endpoint não retornar `best_position` e `best_fitness`.

### batch.rs (5) — UC07 (Inferência em Lote)

| Nome | UC | O que testa |
|---|---|---|
| test_batch_process | UC07 | POST `/api/systems/{id}/batch` com inputs → 200 + resultados |
| test_batch_process_empty | UC07 | POST com array vazio → 200 ou 422 |
| test_batch_list_results | UC07 | GET `/api/systems/{id}/batch-results` → 200 + array |
| test_batch_delete_result | UC07 | DELETE `/api/batch-results/{id}` → 204 |
| test_batch_system_not_found | UC07 | POST com fake_id → 404 |

**Detalhamento:**

Processamento de múltiplos inputs em uma única chamada. Cada input da lista passa pelo motor Mamdani individualmente. `test_batch_list_results` verifica que os resultados foram persistidos. Falham se o batch não suportar arrays JSON ou se a listagem não filtrar por sistema.

### scenarios.rs (5) — UC12 (Salvar Cenário)

| Nome | UC | O que testa |
|---|---|---|
| test_create_scenario | UC12 | POST `/api/systems/{id}/scenarios` → 201 |
| test_create_scenario_validation_error | UC12 | POST com nome vazio → 422 |
| test_list_scenarios | UC12 | GET `/api/systems/{id}/scenarios` → 200 + array |
| test_delete_scenario | UC12 | DELETE `/api/scenarios/{id}` → 204 |
| test_delete_scenario_not_found | UC12 | DELETE com fake_id → 404 |

**Detalhamento:**

Cenários salvam um conjunto de inputs nomeados para recarregar depois. `test_create_scenario` envia nome e inputs JSON. Falham se o schema de inputs mudar ou se o endpoint de listagem não retornar os cenários corretos.

### sweep_surface.rs (5) — UC13 (Varredura) + UC15 (Superfície)

| Nome | UC | O que testa |
|---|---|---|
| test_sweep | UC13 | POST `/api/systems/{id}/sweep` → 200 + pontos (x, y) |
| test_sweep_validation_error | UC13 | POST com step inválido → 422 |
| test_surface | UC15 | POST `/api/systems/{id}/surface` → 200 + grid |
| test_analyze_surface | UC15 | POST `/api/systems/{id}/analyze-surface` → análise |
| test_analyze_surface_invalid_vars | UC15 | POST com vars inválidas → 422 |

**Detalhamento:**

`test_sweep` (UC13): varre uma variável de entrada fixando as outras, gerando pontos (x, y). Falha se o número de pontos não corresponder ao intervalo/step.

`test_surface` / `test_analyze_surface` (UC15): gera grid 2D (duas variáveis de entrada) com a saída em cada ponto. A análise extrai mínimo, máximo, e pontos críticos. Falham se o grid tiver resolução errada ou se as variáveis X/Y não forem selecionáveis.

### compare_export.rs (6) — UC08, UC09, UC11, UC14

| Nome | UC | O que testa |
|---|---|---|
| test_compare_simulations | UC08 | POST `/api/simulations/compare` → 200 + tabela |
| test_compare_simulations_validation | UC08 | POST com 1 ID → 422 |
| test_export_report | UC09 | GET `/api/simulations/{id}/report?format=csv` → 200 + CSV |
| test_export_system | UC11 | GET `/api/systems/{id}/export` → 200 + JSON |
| test_rule_matrix | UC14 | POST `/api/systems/{id}/rule-matrix` → 200 + matriz |

**Detalhamento:**

`test_compare_simulations` (UC08): compara duas simulações lado a lado destacando diferenças. Falha se a comparação não identificar diferenças nos inputs.

`test_export_report` (UC09): gera relatório CSV com inputs, outputs e metadados da simulação. Falha se o CSV estiver mal formatado ou se o Content-Type não for `text/csv`.

`test_export_system` (UC11): exporta sistema completo (variáveis, termos, regras) como JSON. Falha se faltar algum campo.

`test_rule_matrix` (UC14): matriz de ativação: regras × grau de disparo (α). Falha se a matriz não refletir corretamente quais regras foram ativadas.

### audit.rs (3) — UC16 (Histórico de Alterações)

| Nome | UC | O que testa |
|---|---|---|
| test_list_audit | UC16 | GET `/api/systems/{id}/audit` → 200 + eventos |
| test_list_orphan_audit | UC16 | GET `/api/audit/orphans` → 200 + eventos órfãos |
| test_audit_undo_system_delete | UC16 | POST `/api/audit/{event_id}/undo` → undeleção |

**Detalhamento:**

`test_list_audit` verifica eventos de alteração registrados (criação, edição). `test_list_orphan_audit` busca eventos cuja entidade foi deletada. `test_audit_undo_system_delete` restaura um sistema deletado a partir do snapshot. Falham se o trigger de auditoria no banco não for instalado ou se o restore de snapshots estiver quebrado.

### misc.rs (3) — UC05 (OpenWeather) + Geral

| Nome | UC | O que testa |
|---|---|---|
| test_weather_missing_city | UC05 | GET `/api/weather` sem city → 422 |
| test_weather_missing_api_key | UC05 | GET `/api/weather?city=Belem` sem API key → 502 |
| test_all_404_endpoints | Geral | GET `/api/systems|variables|terms|rules/{fake_id}` → 404 |

**Detalhamento:**

`test_weather_missing_city` verifica que a rota `/api/weather` requer parâmetro "city". `test_weather_missing_api_key` remove a env var e verifica erro 502 (Bad Gateway). `test_all_404_endpoints` testa 4 endpoints com UUID inválido. Falham se os middlewares de validação mudarem.

### pipeline.rs (1) — Todos os UCs

| Nome | UC | O que testa |
|---|---|---|
| test_e2e_full_pipeline | Todos | Pipeline completo: cria sistema → variáveis → termos → regras → simula → compara |

**Detalhamento:**

Teste de fumaça end-to-end: cria sistema, adiciona 3 variáveis, 3 termos, 1 regra, simula, compara, lista histórico. Falha se qualquer etapa do pipeline quebrar.

---

## Tests Integração DB (6 — com `#[ignore]`)

Testam diretamente o banco PostgreSQL via sqlx, sem as rotas HTTP. Usam `get_test_pool()` e `begin_test_tx()` para rodar em transações isoladas.

### systems.rs (2) — UC01

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_create_system_integration | UC01 | INSERT em fuzzy_systems com nome, descrição, defuzz → SELECT confirma | Schema SQL alterado; colunas renomeadas |
| test_cascade_delete_system | UC01 | DELETE system → variáveis e termos deletados em cascata | ON DELETE CASCADE removido das FKs |

**Detalhamento:**

`test_create_system_integration`: insere direto no banco e lê de volta, verificando que todos os campos foram persistidos corretamente. Falha se a migration alterar o schema (ex: renomear `defuzz_method` para `defuzz`).

`test_cascade_delete_system`: insere sistema + variável + termo, deleta o sistema, e verifica que as FKs foram deletadas. Falha se alguém remover `ON DELETE CASCADE` das migrations.

### variables.rs (2) — UC02

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_create_variable_integration | UC02 | INSERT variável → SELECT confirma campos | Schema SQL alterado |
| test_create_term_integration | UC02 | INSERT termo com params JSONB → SELECT confirma | Cast ::jsonb removido; tipo params alterado |

**Detalhamento:**

`test_create_variable_integration`: verifica persistência de variável com todos os campos (name, role, universe_min, resolution). Falha se a migration mudar o tipo de universe_min para integer.

`test_create_term_integration`: verifica que params (JSONB) são inseridos corretamente via `'[0,0,25]'::jsonb`. Falha se o cast for removido (já quebrou antes — migrations 007/008 foram corrigidas por isso).

### simulate.rs (2) — UC02 + UC04

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_only_one_consequent | UC02 | INSERT segundo consequente → NÃO falha (validação é da app) | Constraint UNIQUE adicionada no banco |
| test_simulation_persists | UC04 | INSERT simulação + SELECT COUNT → 1 | Schema de simulations alterado |

**Detalhamento:**

`test_only_one_consequent`: verifica que o BANCO permite múltiplos consequentes (a validação é na aplicação). Falha se alguém adicionar `UNIQUE(role)` no banco.

`test_simulation_persists`: insere simulação com inputs/outputs JSONB e verifica persistência via SELECT COUNT. Falha se a tabela `simulations` for alterada.

---

## Tests Integração API (3 — com `#[ignore]` + OpenWeather real)

Testam a rota `/api/weather` contra a OpenWeather API real (requer `OPENWEATHER_API_KEY` configurada). Ignorados por padrão porque dependem de API externa.

| Nome | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_weather_integration_belem | UC05 | GET `/api/weather?city=Belem` → 200 + temp/humidity | API key inválida; URL da API mudou |
| test_weather_integration_sao_paulo | UC05 | GET `/api/weather?city=S%C3%A3o%20Paulo` → 200 + city "São Paulo" | URL encoding quebrado; API retorna nome diferente |
| test_weather_integration_invalid_city | UC05 | GET cidade inexistente → 404 | API retorna 200 com "city not found" em vez de 404 |

**Detalhamento:**

Testam a integração real com a OpenWeather API. Diferentemente dos testes HTTP de weather (que testam validação de parâmetros), estes chamam a API externa de verdade. 

`test_weather_integration_belem`: verifica que Belem retorna temperatura e umidade numéricas com descrição textual. Falha se a API key expirar ou se a OpenWeather mudar o formato da resposta.

`test_weather_integration_sao_paulo`: cidade acentuada com URL encoding. Verifica que o backend decodifica corretamente e que a API aceita "S%C3%A3o%20Paulo". Falha se o backend mudar a estratégia de encoding.

`test_weather_integration_invalid_city`: cidade que não existe retorna 404. Falha se o tratamento de erro da OpenWeather mudar de 404 para 200 com payload de erro.

---

## Tests E2E (40 — Playwright)

Testes no navegador via Playwright. Cobrem a interface Leptos completa.

**Como executar:** `cd end2end && npx playwright test`

| Bloco | Qtd | UCs | Descrição |
|---|---|---|---|
| homepage loads | 1 | UC01 | Título "FuzzySimulated" visível; lista de sistemas carregada |
| sidebar navigation | 1 | UC01-03,06,13-17 | Navegação entre todas as páginas funciona |
| create system form | 1 | UC01 | Formulário de novo sistema tem campos corretos |
| simulator empty state | 1 | UC04 | Estado vazio quando nenhum sistema selecionado |
| Seed: card e status badge | 1 | UC01 | Sistema "Conforto Térmico" aparece com badge de status |
| Seed: 3 vars, 9 terms | 1 | UC02 | Seed tem 3 variáveis e 9 termos visíveis |
| Seed: 9 rules | 1 | UC03 | Editor de regras mostra 9 regras |
| Seed: simulação | 1 | UC04 | Simulação com seed retorna output em [0,10] |
| Seed: status favorito | 1 | UC01 | Mudança de status para favorito e volta |
| Validation: empty name | 1 | UC01 | Criar sistema sem nome mostra erro |
| Validation: invalid term params | 1 | UC02 | Termo com params "abc" mostra erro |
| Delete protection: favorito | 1 | UC01 | Sistema favorito não pode ser deletado |
| Full lifecycle (17 tests) | 17 | UC01-06,08-16 | Cria, configura, simula, compara, exporta, duplica, audita, deleta |
| UC05: OpenWeather | 1 | UC05 | Buscar clima de Belem preenche inputs |
| UC07: Batch | 1 | UC07 | Batch JSON com 3 inputs → tabela de resultados |
| UC11: Import page | 1 | UC11 | Página de import carrega |
| UC13: Sweep | 1 | UC13 | Varredura com Conforto Térmico valida outputs |
| UC15: Surface | 1 | UC15 | Heatmap para "Risco Cibernético Avançado" |
| UC17: PSO | 1 | UC17 | PSO para Conforto Térmico retorna fitness |
| UC18: TSK | 1 | UC18 | Simulação TSK com coeficientes |
| UC19: SVG | 1 | UC19 | Geração de SVGs das MFs |
| UC20: Diagnóstico | 1 | UC20 | Relatório de diagnóstico com fuzzificação e regras |

**Detalhamento dos testes do ciclo completo (17 testes em série):**

01. **Cria sistema fuzzy**: preenche formulário `/newsys` e verifica card na dashboard. Falha se o formulário não redirecionar para "/".
02. **Adiciona 3 variáveis**: 2 antecedentes + 1 consequente. Falha se o role não for persistido.
03. **Adiciona 3 termos**: trimf com labels "Alta", "Alta", "Alto". Falha se params não forem salvos.
04. **Adiciona regra**: "SE Temp = Alta E Umidade = Alta ENTAO Risco = Alto". Falha se a regra não aparecer.
05. **Executa simulação**: inputs 25 e 30 → output em [0,1]. Falha se o motor não rodar.
06. **Salva cenário**: nome "Cenario E2E". Falha se o cenário não for salvo na lista.
07. **Segunda simulação**: inputs 45 e 60. Falha se não gerar output visível.
08. **Histórico**: simulações aparecem na tabela. Falha se listagem estiver vazia.
09. **Comparação**: seleciona 2 simulações nos checkboxes. Falha se a tela de comparação não abrir.
10. **Exporta relatório**: clica no botão de export. Falha se não mostrar mensagem de confirmação.
11. **Edita sistema**: altera descrição. Falha se o PUT não persistir.
12. **Duplica sistema**: verifica que a cópia tem 3 variáveis. Falha se a clonagem não copiar dados.
13. **Exporta sistema como JSON**: verifica JSON com nome, variáveis (3). Falha se o JSON estiver incompleto.
14. **Auditoria**: timeline com eventos. Falha se não houver eventos registrados.
15. **Matriz de regras**: grid de ativação. Falha se o grid não for gerado.
16. **PSO optimizer**: painel de otimização. Falha se o botão do preset não aparecer.
17. **Proteção de status**: favorito bloqueia delete. Falha se o ícone de bloqueio não aparecer.

---

## Resumo Consolidado

| Tipo | Qtd | UCs Cobertas |
|---|---|---|
| Unitários (inline) | 30 | UC01 (4), UC04 (14), UC05 (4), UC16 (9) |
| Unitários (tests/unit/) | 19 | UC01 (7), UC02 (12, incl. NaN/Inf) |
| HTTP Axum | 65 | UC01-26, UC07-08, UC10-20 |
| Integração DB | 6 | UC01 (2), UC02 (2), UC04 (1), UC02-04 (1) |
| Integração API (OpenWeather) | 3 | UC05 (3, com API real) |
| E2E Playwright | 40 | UC01-08, UC10-20 |
| **Total** | **161** | **UC01-20 completos** |

### Mapa UC → Testes

| UC | Unit (inline) | Unit (tests/) | HTTP | Integração DB | Integração API | E2E | Total |
|---|---|---|---|---|---|---|---|
| UC01 — Gerenciar Sistemas | 4 (errors.rs) | 7 (system_validation.rs) | 8 (systems.rs + misc.rs) | 2 (systems.rs) | — | 6 | 27 |
| UC02 — Gerenciar Variáveis e Termos | — | 12 (mf_validation.rs) | 12 (variables.rs + terms.rs) | 3 (variables.rs + simulate.rs) | — | 3 | 30 |
| UC03 — Gerenciar Regras | — | — | 5 (rules.rs) | — | — | 1 | 6 |
| UC04 — Executar Simulação | 14 (engine.rs) | — | 2 (simulate.rs) | 1 (simulate.rs) | — | 2 | 19 |
| UC05 — OpenWeather | 4 (weather.rs) | — | 2 (misc.rs) | — | 3 (integration_api) | 1 | 10 |
| UC06 — Histórico | — | — | 1 (simulate.rs) | — | — | 1 | 2 |
| UC07 — Batch | — | — | 5 (batch.rs) | — | — | 1 | 6 |
| UC08 — Comparar | — | — | 2 (compare_export.rs) | — | — | 1 | 3 |
| UC09 — Exportar Relatório | — | — | 1 (compare_export.rs) | — | — | 1 | 2 |
| UC10 — Duplicar | — | — | 1 (simulate.rs) | — | — | 1 | 2 |
| UC11 — Exportar/Importar | — | — | 1 (compare_export.rs) | — | — | 1 | 2 |
| UC12 — Cenários | — | — | 5 (scenarios.rs) | — | — | — | 5 |
| UC13 — Varredura | — | — | 2 (sweep_surface.rs) | — | — | 1 | 3 |
| UC14 — Matriz de Regras | — | — | 1 (compare_export.rs) | — | — | 1 | 2 |
| UC15 — Superfície | — | — | 3 (sweep_surface.rs) | — | — | 1 | 4 |
| UC16 — Auditoria | 9 (audit_routes.rs) | — | 3 (audit.rs) | — | — | 1 | 13 |
| UC17 — PSO | — | — | 2 (simulate.rs) | — | — | 2 | 4 |
| UC18 — TSK | — | — | 2 (simulate.rs) | — | — | 1 | 3 |
| UC19 — SVG | — | — | 2 (simulate.rs) | — | — | 1 | 3 |
| UC20 — Diagnóstico | — | — | 2 (simulate.rs) | — | — | 1 | 3 |
| **Total** | **30** | **19** | **60** | **6** | **3** | **24** | **142** |

> Nota: testes contados múltiplas vezes se cobrem mais de um UC (ex: `test_e2e_full_pipeline` cobre todos). O total real de testes é 161, sem duplicação.
