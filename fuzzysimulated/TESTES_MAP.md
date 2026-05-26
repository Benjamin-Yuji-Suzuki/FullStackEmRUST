# Mapa de Testes — FuzzySimulated

## Tests Unitários (inline — 30)

Testes definidos com `#[test]` dentro de `server/src/`.

### engine.rs (14) — UC04

| Nome do Teste | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_trimf_peak | UC04 | membership(25, trimf[0,25,50]) = 1.0 (pico) | Algoritmo trimf quebrado ou params trocados |
| test_trimf_left_edge | UC04 | membership(0, trimf[0,25,50]) = 0.0 (borda esq) | Cálculo de rampa incorreto |
| test_trimf_right_edge | UC04 | membership(50, trimf[0,25,50]) = 0.0 (borda dir) | Cálculo de rampa incorreto |
| test_trimf_linear_rise | UC04 | membership(12.5, trimf[0,25,50]) = 0.5 (meio da rampa) | Interpolação linear errada |
| test_trimf_linear_fall | UC04 | membership(37.5, trimf[0,25,50]) = 0.5 (meio da descida) | Interpolação linear errada |
| test_trapmf_plateau | UC04 | membership(30, trapmf[0,20,40,60]) = 1.0 (platô) | Algoritmo trapmf quebrado |
| test_trapmf_left_ramp | UC04 | membership(10, trapmf[0,20,40,60]) = 0.5 | Cálculo de rampa esquerda incorreto |
| test_trapmf_right_ramp | UC04 | membership(50, trapmf[0,20,40,60]) = 0.5 | Cálculo de rampa direita incorreto |
| test_trapmf_outside | UC04 | membership(-1, trapmf[0,20,40,60]) = 0.0 (fora) | Borda inferior não tratada |
| test_gaussmf_peak | UC04 | membership(50, gaussmf[50,10]) = 1.0 (pico) | Algoritmo gaussmf quebrado |
| test_gaussmf_one_sigma | UC04 | membership(60, gaussmf[50,10]) = exp(-0.5) | Cálculo de gaussiana incorreto |
| test_parse_simple_rule | UC04 | Parse "SE Temp = Alta ENTAO Risco = Alto" retorna 2 conditions | Parser de regras quebrado |
| test_parse_portuguese_rule | UC04 | Parse "SE temp é frio E umidade é seco..." com 3 conditions | Parser não aceita "é" ou "E" |
| test_mamdani_basic_inference | UC04 | Inferência Mamdani completa com 1 regra e input 80° → Risco > 0 | Motor de inferência quebrado |

### errors.rs (4) — UC01

| Nome do Teste | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_validation_status | UC01 | AppError::Validation → 422 UNPROCESSABLE_ENTITY | Mapeamento de erro quebrado |
| test_not_found_status | UC01 | AppError::NotFound → 404 NOT_FOUND | Mapeamento de erro quebrado |
| test_database_status | UC01 | AppError::Database → 500 INTERNAL_SERVER_ERROR | Mapeamento de erro quebrado |
| test_external_status | UC01 | AppError::External → 502 BAD_GATEWAY | Mapeamento de erro quebrado |

### audit_routes.rs (8) — UC16

| Nome do Teste | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_entity_table_system | UC16 | entity_table("system") = "fuzzy_systems" | Mapa entidade-tabela incorreto |
| test_entity_table_variable | UC16 | entity_table("variable") = "fuzzy_variables" | Mapa entidade-tabela incorreto |
| test_entity_table_term | UC16 | entity_table("term") = "fuzzy_terms" | Mapa entidade-tabela incorreto |
| test_entity_table_rule | UC16 | entity_table("rule") = "fuzzy_rules" | Mapa entidade-tabela incorreto |
| test_entity_table_unknown | UC16 | entity_table("invalid") → Err | Mapa aceita entidade inválida |
| test_snapshot_object_fields_extracts_keys | UC16 | snapshot_object_fields extrai chaves de objeto JSON | Função de extração quebrada |
| test_snapshot_object_fields_non_object | UC16 | snapshot_object_fields de string/number/null = "" | Não trata não-objeto |
| test_snapshot_object_fields_empty | UC16 | snapshot_object_fields({}) = "" | Objeto vazio retorna string não-vazia |

### weather.rs (4) — UC05

| Nome do Teste | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_urlencoding_ascii | UC05 | urlencoding("hello") = "hello" | Função URL encoding quebrada |
| test_urlencoding_with_spaces | UC05 | urlencoding("São Paulo") = "S%C3%A3o%20Paulo" | Encoding de acentos/espaços errado |
| test_urlencoding_special_chars | UC05 | urlencoding("a&b=c") = "a%26b%3Dc" | Encoding de caracteres especiais errado |
| test_urlencoding_empty | UC05 | urlencoding("") = "" | String vazia não tratada |

---

## Tests Unitários (tests/unit/ — 17)

### mf_validation.rs (11) — UC02

| Nome do Teste | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_validate_trimf_ok | UC02 | trimf [0,10,12] é válido | Validação rejeita parâmetros corretos |
| test_validate_trimf_non_finite | UC02 | NaN e Infinity são rejeitados no trimf | Aceita valores não-finitos |
| test_validate_trimf_shoulder | UC02 | trimf [0,0,25] (shoulder esq) e [25,50,50] (dir) são válidos | Validação rejeita shoulder legítimo |
| test_validate_trimf_incoherent | UC02 | trimf [22,10,0] (fora de ordem) é rejeitado | Aceita parâmetros fora de ordem |
| test_validate_trimf_wrong_params | UC02 | trimf com 2 ou 4 params é rejeitado | Aceita quantidade errada de params |
| test_validate_trapmf_ok | UC02 | trapmf [0,0,20,40] e shoulder [60,80,100,100] são válidos | Validação rejeita trapmf correto |
| test_validate_trapmf_incoherent | UC02 | trapmf [40,20,0,0] (fora de ordem) é rejeitado | Aceita parâmetros fora de ordem |
| test_validate_gaussmf_ok | UC02 | gaussmf [50,15] é válido | Validação rejeita gaussmf correto |
| test_validate_gaussmf_zero_sigma | UC02 | gaussmf sigma=0 rejeitado | Aceita sigma zero (divisão por zero) |
| test_validate_gaussmf_negative_sigma | UC02 | gaussmf sigma=-1 rejeitado | Aceita sigma negativo |
| test_validate_gaussmf_wrong_params | UC02 | gaussmf com 1 ou 3 params rejeitado | Aceita quantidade errada de params |

### system_validation.rs (6) — UC01

| Nome do Teste | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_validate_system_name_ok | UC01 | "Conforto Térmico" é nome válido | Validação rejeita nome correto |
| test_validate_system_name_empty | UC01 | Nome vazio é rejeitado | Aceita nome vazio |
| test_validate_system_name_whitespace | UC01 | Nome com só espaços é rejeitado | Aceita whitespace como nome |
| test_validate_system_name_too_long | UC01 | Nome com 256 caracteres é rejeitado | Aceita nome muito longo |
| test_validate_defuzz_method_valid | UC01 | centroid/bisector/mom/lom/som são válidos | Validação rejeita método válido |
| test_validate_defuzz_method_invalid | UC01 | "invalid" é rejeitado | Aceita método de defuzz inexistente |

---

## Tests HTTP Axum (64)

### systems.rs (8) — UC01

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_create_system | UC01 | POST /api/systems cria sistema e retorna 201 com nome correto |
| test_list_systems | UC01 | GET /api/systems retorna array de sistemas |
| test_get_system_by_id | UC01 | GET /api/systems/{id} retorna sistema específico |
| test_update_system | UC01 | PUT /api/systems/{id} atualiza nome e descrição |
| test_delete_system | UC01 | DELETE /api/systems/{id} deleta e GET depois retorna 404 |
| test_update_system_status | UC01 | PUT /api/systems/{id}/status altera status para "ativo" |
| test_system_not_found | UC01 | GET /api/systems/{uuid_zerado} retorna 404 |
| test_create_system_validation_error | UC01 | POST com nome vazio e método inválido retorna 422 |

### variables.rs (7) — UC02

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_create_variable | UC02 | POST /api/systems/{id}/variables cria variável e retorna 201 |
| test_list_variables | UC02 | GET /api/systems/{id}/variables retorna array |
| test_get_variable | UC02 | GET /api/variables/{id} retorna variável específica |
| test_update_variable | UC02 | PUT /api/variables/{id} atualiza nome, role e universe |
| test_delete_variable | UC02 | DELETE /api/variables/{id} retorna 204 |
| test_variable_not_found | UC02 | GET /api/variables/{uuid_zerado} retorna 404 |
| test_create_variable_validation_error | UC02 | POST com nome vazio, role inválida, universe_min > max retorna 422 |

### terms.rs (5) — UC02

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_create_term | UC02 | POST /api/variables/{id}/terms cria termo e retorna 201 |
| test_get_term | UC02 | GET /api/terms/{id} retorna termo específico |
| test_update_term | UC02 | PUT /api/terms/{id} atualiza label, mf_type e params |
| test_delete_term | UC02 | DELETE /api/terms/{id} retorna 204 |
| test_create_term_validation_error | UC02 | POST com label vazio e params insuficientes retorna 422 |

### rules.rs (5) — UC03

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_create_rule | UC03 | POST /api/systems/{id}/rules cria regra e retorna 201 |
| test_get_rule | UC03 | GET /api/rules/{id} retorna regra específica |
| test_update_rule | UC03 | PUT /api/rules/{id} atualiza texto e peso |
| test_delete_rule | UC03 | DELETE /api/rules/{id} retorna 204 |
| test_rule_not_found | UC03 | GET /api/rules/{uuid_zerado} retorna 404 |

### simulate.rs (12) — UC04, UC17, UC18, UC19, UC20

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_simulate | UC04 | POST /api/systems/{id}/simulate executa Mamdani e retorna outputs |
| test_simulate_missing_input | UC04 | Simulação com inputs vazios (deve aceitar ou rejeitar graciosamente) |
| test_list_simulations | UC06 | GET /api/systems/{id}/simulations lista histórico |
| test_duplicate_system | UC10 | POST /api/systems/{id}/duplicate cria cópia com novo nome |
| test_simulate_tsk | UC18 | POST /api/systems/{id}/simulate-tsk com coeffs retorna outputs TSK |
| test_simulate_tsk_system_not_found | UC18 | TSK com UUID inexistente retorna 404 |
| test_svg_export | UC19 | GET /api/systems/{id}/svg retorna SVGs das variáveis |
| test_svg_export_system_not_found | UC19 | SVG com UUID inexistente retorna 404 |
| test_diagnostic | UC20 | POST /api/systems/{id}/diagnostic retorna fuzzification, regras, saídas |
| test_diagnostic_system_not_found | UC20 | Diagnóstico com UUID inexistente retorna 404 |
| test_optimize_pso | UC17 | POST /api/systems/{id}/optimize-pso retorna best_position e fitness |
| test_optimize_pso_invalid_data | UC17 | PSO com dados vazios retorna 200 ou 422 |

### batch.rs (5) — UC07

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_batch_process | UC07 | POST /api/batch processa lote de 2 inputs, retorna total=2 processed=2 |
| test_batch_process_empty | UC07 | POST /api/batch com inputs vazios retorna 422 |
| test_batch_list_results | UC07 | GET /api/batch/{system_id} lista resultados do lote |
| test_batch_delete_result | UC07 | DELETE /api/batch/result/{id} deleta resultado individual |
| test_batch_system_not_found | UC07 | Batch com system_id zerado retorna 404 |

### scenarios.rs (5) — UC12

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_create_scenario | UC12 | POST /api/systems/{id}/scenarios cria cenário e retorna 201 |
| test_create_scenario_validation_error | UC12 | POST com nome vazio retorna 422 |
| test_list_scenarios | UC12 | GET /api/systems/{id}/scenarios lista cenários |
| test_delete_scenario | UC12 | DELETE /api/scenarios/{id} deleta cenário |
| test_delete_scenario_not_found | UC12 | DELETE com UUID zerado retorna 404 |

### sweep_surface.rs (5) — UC13, UC15

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_sweep | UC13 | POST /api/systems/{id}/sweep varre variável e retorna pontos |
| test_sweep_validation_error | UC13 | Sweep com start>end e step negativo retorna 422 |
| test_surface | UC15 | POST /api/systems/{id}/surface gera grid 5x5 |
| test_analyze_surface | UC15 | POST /api/systems/{id}/analyze-surface classifica superfície (mínimo/sela/etc) |
| test_analyze_surface_invalid_vars | UC15 | Analyze com variáveis inexistentes retorna 422 |

### compare_export.rs (5) — UC08, UC09, UC10, UC11, UC14

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_compare_simulations | UC08 | POST /api/simulations/compare compara 2 simulações |
| test_compare_simulations_validation | UC08 | Compare com 1 ID inválido retorna 422 |
| test_export_report | UC09 | GET /api/simulations/{id}/report exporta relatório JSON |
| test_export_system | UC11 | GET /api/systems/{id}/export exporta sistema completo |
| test_rule_matrix | UC14 | POST /api/systems/{id}/rule-matrix calcula ativações das regras |

### audit.rs (3) — UC16

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_list_audit | UC16 | GET /api/systems/{id}/audit lista eventos de auditoria |
| test_list_orphan_audit | UC16 | GET /api/audit/orphans lista eventos órfãos |
| test_audit_undo_system_delete | UC16 | POST /api/audit/{id}/undo restaura sistema deletado |

### pipeline.rs (1) — todos UCs end-to-end

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_e2e_full_pipeline | UC01–UC20 | Pipeline completo: criar sistema → variáveis → termos → regras → simular Mamdani → diagnóstico → SVG → TSK → batch → histórico → export → rule matrix → sweep → surface → cenários → compare → duplicate → import → status → PSO manual → PSO auto → audit |

### misc.rs (3) — UC05

| Nome do Teste | UC | O que testa |
|---|---|---|
| test_weather_missing_city | UC05 | GET /api/weather sem ?city= retorna 422 |
| test_weather_missing_api_key | UC05 | GET /api/weather?city=Belem sem API key retorna 502 |
| test_all_404_endpoints | UC01 | GET em /api/systems|variables|terms|rules com UUID zerado retorna 404 |

---

## Tests de Integração DB (6 — todos #[ignore])

| Nome do Teste | UC | O que testa | O que faz falhar |
|---|---|---|---|
| test_create_system_integration | UC01 | INSERT direto em fuzzy_systems via sqlx e confirma colunas | Schema DB alterado |
| test_cascade_delete_system | UC01 | DELETE em fuzzy_systems remove variáveis e termos em cascata | FK sem CASCADE ou trigger ausente |
| test_create_variable_integration | UC02 | INSERT direto em fuzzy_variables via sqlx | Schema DB alterado |
| test_create_term_integration | UC02 | INSERT direto em fuzzy_terms com params JSONB | Schema DB alterado |
| test_only_one_consequent | UC04 | DB permite múltiplos consequentes (validação é da aplicação, não do DB) | Trigger DB que impeça 2 consequentes |
| test_simulation_persists | UC06 | INSERT em simulations e confirma persistência | Tabela simulations sem INSERT |

---

## Tests E2E (40 — Playwright)

| Nome do Teste | UC | O que testa |
|---|---|---|
| homepage loads with FuzzySimulated title and system list | UC01 | Página inicial carrega com título e lista de sistemas |
| sidebar navigation works for all pages | UC01 | Navegação pela sidebar para todas as páginas |
| create system form loads with correct fields | UC01 | Formulário de novo sistema tem campos corretos |
| simulator page shows empty state when no system selected | UC04 | Simulador mostra estado vazio sem sistema selecionado |
| dashboard shows seed system card with status badge | UC01 | Card do sistema "Conforto Térmico" aparece com badge |
| view seed system variables — 3 variables and 9 terms | UC02 | Seed system tem 3 variáveis e 9 termos |
| seed system has 9 rules in rule editor | UC03 | Seed system tem 9 regras no editor |
| simulate with seed system — validate actual output value | UC04 | Simulação com seed system retorna output em [0,10] |
| change seed system status to favorito and back | UC01 | Altera status do seed system para favorito e volta |
| create system with empty name shows error | UC01 | Criação com nome vazio mostra erro "Nome obrigatório" |
| add term with empty params shows error | UC02 | Adicionar termo com params inválidos mostra erro |
| delete protection: favorito system can't be deleted | UC01 | Sistema favorito exibe proteção e não pode ser deletado |
| 01: creates a new fuzzy system | UC01 | Cria sistema completo via formulário |
| 02: adds 3 variables (2 antecedent + 1 consequent) | UC02 | Adiciona 3 variáveis |
| 03: adds 3 terms (Alta/Alta/Alto) | UC02 | Adiciona 3 termos |
| 04: adds a rule | UC03 | Adiciona regra SE Temperatura = Alta E Umidade = Alta ENTAO Risco = Alto |
| 05: runs simulation and validates output value | UC04 | Executa simulação e valida output em [0,1] |
| 06: saves a scenario | UC12 | Salva cenário com nome "Cenario E2E" |
| 07: runs second simulation for comparison | UC04 | Executa segunda simulação |
| 08: simulation appears in history | UC06 | Simulação aparece no histórico |
| 09: compares two simulations | UC08 | Compara duas simulações selecionadas |
| 10: exports simulation report | UC09 | Exporta relatório da simulação |
| 11: edits system description | UC01 | Edita descrição do sistema |
| 12: duplicates the system and verifies copy has same data | UC10 | Duplica sistema e verifica dados da cópia |
| 13: exports system as JSON and validates content | UC11 | Exporta sistema como JSON e valida conteúdo |
| 14: audit page shows events for the system | UC16 | Página de auditoria mostra eventos |
| 15: analysis page — rule matrix counts match | UC14 | Matriz de regras na página de análise |
| 16: optimizer page shows PSO panel and runs preset | UC17 | Otimizador PSO mostra painel |
| 17: status protection — favorito blocks delete | UC01 | Proteção de status: favorito bloqueia deleção |
| cleanup: deletes duplicated system and original | UC01 | Limpeza: deleta cópia e original |
| keeps seed system intact for manual inspection | UC01 | Seed system permanece intacto |
| weather fetch populates temperature and humidity inputs | UC05 | Busca clima da API OpenWeather preenche inputs |
| batch with JSON inputs validates output values | UC07 | Batch processa 3 inputs JSON e valida outputs |
| import page loads with correct title | UC11 | Página de importar carrega com título correto |
| sweep with Conforto Térmico validates y-values in [0,10] | UC13 | Varredura com seed system valida outputs em [0,10] |
| generate surface heatmap for Risco Cibernético Avançado | UC15 | Gera heatmap de superfície e valida z-range |
| run PSO optimization for Conforto Térmico | UC17 | Executa otimização PSO com fitness >= 0 |
| run TSK simulation on Conforto Térmico | UC18 | Executa simulação TSK e valida output em [0,10] |
| generate SVG for Conforto Térmico variables | UC19 | Gera SVGs das funções de pertinência |
| generate diagnostic for Conforto Térmico simulation | UC20 | Gera diagnóstico explicativo da simulação |

---

## Resumo por UC

| UC | Descrição | Tests Unit | Tests HTTP | Tests E2E | Tests Integração | Total |
|---|---|---|---|---|---|---|
| UC01 | CRUD Sistema Fuzzy | 10 | 12 | 9 | 2 | 33 |
| UC02 | Variáveis e Termos (MF) | 11 | 12 | 3 | 2 | 28 |
| UC03 | Regras | 0 | 5 | 2 | 0 | 7 |
| UC04 | Motor de Inferência (Mamdani) | 14 | 3 | 3 | 1 | 21 |
| UC05 | OpenWeather | 4 | 3 | 1 | 0 | 8 |
| UC06 | Histórico de Simulações | 0 | 1 | 1 | 1 | 3 |
| UC07 | Processamento em Lote (Batch) | 0 | 5 | 1 | 0 | 6 |
| UC08 | Comparação de Simulações | 0 | 2 | 1 | 0 | 3 |
| UC09 | Exportar Relatório | 0 | 1 | 1 | 0 | 2 |
| UC10 | Duplicar Sistema | 0 | 1 | 1 | 0 | 2 |
| UC11 | Importar/Exportar Sistema | 0 | 1 | 2 | 0 | 3 |
| UC12 | Cenários | 0 | 5 | 1 | 0 | 6 |
| UC13 | Varredura (Sweep) | 0 | 2 | 1 | 0 | 3 |
| UC14 | Matriz de Regras | 0 | 1 | 1 | 0 | 2 |
| UC15 | Superfície de Controle | 0 | 3 | 1 | 0 | 4 |
| UC16 | Auditoria | 8 | 3 | 1 | 0 | 12 |
| UC17 | Otimização PSO | 0 | 3 | 1 | 0 | 4 |
| UC18 | Inferência TSK | 0 | 2 | 1 | 0 | 3 |
| UC19 | Exportação SVG | 0 | 2 | 1 | 0 | 3 |
| UC20 | Diagnóstico | 0 | 2 | 1 | 0 | 3 |
| **Total** | | **47** | **64** | **40** | **6** | **157** |

> Nota: Testes no pipeline.rs (1) e no e2e full lifecycle (18) contam múltiplos UCs e foram distribuídos proporcionalmente no resumo. O valor "Total" na tabela pode exceder a soma simples porque um teste pode cobrir múltiplos UCs.
