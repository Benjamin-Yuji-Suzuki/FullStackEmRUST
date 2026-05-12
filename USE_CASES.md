# 📋 Casos de Uso — FuzzySimulated

> Especificação completa dos 15 casos de uso da plataforma, seguindo o padrão:
> ator(es), pré-condições, fluxo principal, fluxos alternativos (com retorno ao fluxo principal) e pós-condições.

**Projeto:** FuzzySimulated  
**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional — CESUPA 01/2026  
**Repositório:** https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST

---

## Índice

| ID | Nome | Tela | Atores |
|---|---|---|---|
| [UC01](#uc01) | Criar novo sistema fuzzy | Dashboard | Usuário, Sistema |
| [UC02](#uc02) | Editar metadados de um sistema | Dashboard | Usuário, Sistema |
| [UC03](#uc03) | Excluir sistema fuzzy | Dashboard | Usuário, Sistema |
| [UC04](#uc04) | Adicionar variável antecedente | Editor de Variáveis | Usuário, Sistema |
| [UC05](#uc05) | Adicionar variável consequente | Editor de Variáveis | Usuário, Sistema |
| [UC06](#uc06) | Adicionar termo linguístico a uma variável | Editor de Variáveis | Usuário, Sistema |
| [UC07](#uc07) | Remover variável ou termo | Editor de Variáveis | Usuário, Sistema |
| [UC08](#uc08) | Criar regra fuzzy via interface visual | Editor de Regras | Usuário, Sistema |
| [UC09](#uc09) | Editar regra existente | Editor de Regras | Usuário, Sistema |
| [UC10](#uc10) | Remover regra | Editor de Regras | Usuário, Sistema |
| [UC11](#uc11) | Executar simulação com inputs manuais | Simulador | Usuário, Sistema, Backend |
| [UC12](#uc12) | Buscar dados climáticos reais via OpenWeather | Simulador | Usuário, Backend, OpenWeather API |
| [UC13](#uc13) | Visualizar pipeline completo da simulação | Simulador | Usuário, Sistema |
| [UC14](#uc14) | Consultar histórico de simulações | Histórico | Usuário, Sistema |
| [UC15](#uc15) | Validar sistema antes de executar | Sistema (automático) | Sistema, Backend |

---

## UC01

### Criar novo sistema fuzzy

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Usuário está na tela Dashboard. A aplicação está em execução e conectada ao banco de dados. |

**Fluxo Principal**

1. Usuário clica no botão "Novo Sistema" no Dashboard.
2. **Sistema** exibe um formulário modal com os campos: Nome (texto, obrigatório), Descrição (texto longo, opcional) e Método de defuzzificação (seleção: `centroid`, `bisector`, `mom`, `lom`, `som`; padrão: `centroid`).
3. Usuário preenche o campo Nome e, opcionalmente, Descrição e Método de defuzzificação.
4. Usuário clica em "Confirmar".
5. **Sistema** valida que o campo Nome não está vazio e não ultrapassa 255 caracteres.
6. **Sistema** envia os dados ao backend via POST `/api/systems`.
7. **Backend** persiste o novo registro na tabela `fuzzy_systems`, gerando automaticamente `id` (UUID), `created_at` e `updated_at`.
8. **Backend** retorna o objeto criado com status HTTP 201.
9. **Sistema** fecha o modal e redireciona o usuário para o Editor de Variáveis do sistema recém-criado.
10. O novo sistema passa a aparecer listado no Dashboard em acessos futuros.

**Fluxos Alternativos**

- **FA1 — Nome vazio (passo 5):** Sistema exibe mensagem de erro inline "O nome do sistema é obrigatório". O formulário permanece aberto. Retorna ao passo 3.
- **FA2 — Nome excede 255 caracteres (passo 5):** Sistema exibe "O nome deve ter no máximo 255 caracteres". Retorna ao passo 3.
- **FA3 — Falha de conexão com o backend (passo 6):** Sistema exibe "Não foi possível criar o sistema. Verifique sua conexão e tente novamente." Nenhum registro é criado. Retorna ao passo 4.
- **FA4 — Usuário cancela o formulário (qualquer passo):** Modal é fechado sem persistir nada. Dashboard permanece inalterado.

**Pós-condições**

- Um novo registro existe em `fuzzy_systems` com `id`, `name`, `defuzz_method`, `created_at` e `updated_at` preenchidos.
- O usuário está na tela do Editor de Variáveis do sistema criado, pronto para adicionar variáveis (UC04/UC05).

---

## UC02

### Editar metadados de um sistema

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos um sistema fuzzy existe no banco. Usuário está na tela Dashboard. |

**Fluxo Principal**

1. Usuário localiza o card do sistema desejado na listagem do Dashboard.
2. Usuário clica no botão "Editar" do card.
3. **Sistema** busca os dados atuais do sistema via GET `/api/systems/{id}`.
4. **Sistema** exibe formulário modal pré-preenchido com Nome, Descrição e Método de defuzzificação atuais.
5. Usuário altera um ou mais campos.
6. Usuário clica em "Salvar".
7. **Sistema** valida que Nome não está vazio e não ultrapassa 255 caracteres.
8. **Sistema** envia os dados atualizados ao backend via PUT `/api/systems/{id}`.
9. **Backend** executa `UPDATE` em `fuzzy_systems`, atualizando também `updated_at` com o timestamp atual.
10. **Backend** retorna o objeto atualizado com status HTTP 200.
11. **Sistema** fecha o modal e atualiza o card no Dashboard com os novos valores.

**Fluxos Alternativos**

- **FA1 — Nome esvaziado antes de salvar (passo 7):** Sistema exibe erro inline. Retorna ao passo 5.
- **FA2 — Usuário não altera nenhum campo e clica em Salvar (passo 6):** Sistema envia a requisição normalmente; backend atualiza apenas `updated_at`. Retorna ao passo 11.
- **FA3 — Usuário cancela (qualquer passo):** Modal é fechado; nenhuma alteração é persistida. Dashboard permanece com os dados anteriores.
- **FA4 — Falha no backend (passo 8):** Sistema exibe mensagem de erro. Retorna ao passo 6.

**Pós-condições**

- O registro em `fuzzy_systems` reflete os novos valores informados pelo usuário.
- O campo `updated_at` foi atualizado com o timestamp da operação.

---

## UC03

### Excluir sistema fuzzy

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos um sistema fuzzy existe. Usuário está no Dashboard. |

**Fluxo Principal**

1. Usuário localiza o card do sistema que deseja excluir.
2. Usuário clica no botão "Excluir" do card.
3. **Sistema** exibe diálogo de confirmação: "Tem certeza? Esta ação removerá permanentemente o sistema '[nome]' e todos os seus dados — variáveis, termos, regras e histórico de simulações."
4. Usuário clica em "Confirmar exclusão".
5. **Sistema** envia requisição DELETE para `/api/systems/{id}`.
6. **Backend** executa `DELETE` em `fuzzy_systems`; `ON DELETE CASCADE` propaga automaticamente a exclusão para `fuzzy_variables`, `fuzzy_terms`, `fuzzy_rules` e `simulations`.
7. **Backend** retorna status HTTP 204 (No Content).
8. **Sistema** remove o card do Dashboard sem recarregar a página.

**Fluxos Alternativos**

- **FA1 — Usuário clica em "Cancelar" no diálogo (passo 4):** Diálogo é fechado; nenhuma exclusão ocorre. Retorna ao passo 1.
- **FA2 — Falha no backend (passo 5):** Sistema exibe "Não foi possível excluir o sistema. Tente novamente." O card permanece no Dashboard. Retorna ao passo 2.

**Pós-condições**

- O registro do sistema e todos os seus dados dependentes foram permanentemente removidos do banco de dados.
- O Dashboard não exibe mais o card do sistema excluído.

---

## UC04

### Adicionar variável antecedente

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Um sistema fuzzy foi criado (UC01) e está aberto no Editor de Variáveis. |

**Fluxo Principal**

1. Usuário clica em "Adicionar Variável de Entrada" na seção de antecedentes do Editor de Variáveis.
2. **Sistema** exibe formulário com os campos: Nome da variável (texto, obrigatório), Universo mínimo (número, obrigatório), Universo máximo (número, obrigatório) e Resolução (inteiro, padrão: 501 pontos de discretização).
3. Usuário preenche os campos. Exemplo: Nome = "Temperatura", mín = 0, máx = 50, resolução = 501.
4. Usuário clica em "Adicionar".
5. **Sistema** valida: Nome não vazio; `universe_min < universe_max`; Resolução ≥ 2.
6. **Sistema** envia POST para `/api/systems/{id}/variables` com `role = 'antecedent'`.
7. **Backend** persiste o registro em `fuzzy_variables` com `role = 'antecedent'`.
8. **Backend** retorna o objeto criado com status HTTP 201.
9. **Sistema** exibe a nova variável na seção "Entradas" do editor, com painel vazio aguardando termos linguísticos.

**Fluxos Alternativos**

- **FA1 — `universe_min ≥ universe_max` (passo 5):** Sistema exibe "O limite mínimo deve ser estritamente menor que o máximo." Retorna ao passo 3.
- **FA2 — Nome vazio (passo 5):** Sistema exibe erro inline. Retorna ao passo 3.
- **FA3 — Resolução < 2 (passo 5):** Sistema exibe "A resolução mínima é de 2 pontos." Retorna ao passo 3.
- **FA4 — Nome duplicado no mesmo sistema (passo 5):** Sistema exibe aviso "Já existe uma variável com este nome neste sistema." Permite prosseguir ou renomear. Retorna ao passo 3.

**Pós-condições**

- Registro criado em `fuzzy_variables` com `role = 'antecedent'`, vinculado ao `system_id` correto.
- O Editor de Variáveis exibe a nova entrada pronta para receber termos (UC06).

---

## UC05

### Adicionar variável consequente

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Um sistema fuzzy foi criado (UC01) e está aberto no Editor de Variáveis. |

**Fluxo Principal**

1. Usuário clica em "Adicionar Variável de Saída" na seção de consequentes do Editor de Variáveis.
2. **Sistema** exibe o mesmo formulário de UC04 (Nome, Universo mínimo, Universo máximo, Resolução).
3. Usuário preenche os campos. Exemplo: Nome = "Conforto", mín = 0, máx = 100, resolução = 501.
4. Usuário clica em "Adicionar".
5. **Sistema** aplica as mesmas validações de UC04 (passo 5).
6. **Sistema** envia POST para `/api/systems/{id}/variables` com `role = 'consequent'`.
7. **Backend** persiste em `fuzzy_variables` com `role = 'consequent'`.
8. **Backend** retorna o objeto criado com status HTTP 201.
9. **Sistema** exibe a nova variável na seção "Saídas" do editor.

**Fluxos Alternativos**

- **FA1 — Erros de validação (passo 5):** Comportamento idêntico a UC04. Retorna ao passo 3.
- **FA2 — Tentativa de adicionar segunda variável consequente (passo 6):** Sistema exibe aviso "A biblioteca logicfuzzy-academic suporta uma variável consequente por sistema nesta versão." Permite prosseguir; comportamento em runtime depende da versão da biblioteca.

**Pós-condições**

- Registro criado em `fuzzy_variables` com `role = 'consequent'`, vinculado ao `system_id`.
- O Editor de Variáveis exibe a nova saída pronta para receber termos (UC06).

---

## UC06

### Adicionar termo linguístico a uma variável

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos uma variável (antecedente ou consequente) existe no sistema. Editor de Variáveis está aberto. |

**Fluxo Principal**

1. Usuário localiza a variável desejada no Editor de Variáveis e clica em "Adicionar Termo".
2. **Sistema** exibe formulário com os campos: Rótulo / Label (texto, obrigatório) e Tipo de função de pertinência (seleção: `trimf`, `trapmf`, `gaussmf`).
3. Usuário seleciona o tipo de MF. **Sistema** atualiza dinamicamente os campos de parâmetros:
   - `trimf` → campos a, b, c (onde a ≤ b ≤ c)
   - `trapmf` → campos a, b, c, d (onde a ≤ b ≤ c ≤ d)
   - `gaussmf` → campos mean (média) e sigma (desvio padrão, > 0)
4. Usuário preenche o rótulo e os parâmetros. Exemplo: "Quente", `trimf`, [35, 42, 50].
5. **Sistema** exibe prévia gráfica da função de pertinência em tempo real durante o preenchimento.
6. Usuário clica em "Salvar Termo".
7. **Sistema** valida: Rótulo não vazio; parâmetros respeitam a ordenação exigida pelo tipo de MF; para `gaussmf`, sigma > 0.
8. **Sistema** envia POST para `/api/variables/{variable_id}/terms`.
9. **Backend** persiste em `fuzzy_terms` com `params` armazenado como JSONB.
10. **Backend** retorna o objeto criado com status HTTP 201.
11. **Sistema** exibe o novo termo listado sob a variável, com o gráfico da MF renderizado.

**Fluxos Alternativos**

- **FA1 — Rótulo vazio (passo 7):** Sistema bloqueia e exibe erro inline. Retorna ao passo 4.
- **FA2 — Parâmetros fora de ordem, ex.: a > b para trimf (passo 7):** Sistema exibe "Os parâmetros devem respeitar a ordenação a ≤ b ≤ c." Retorna ao passo 4.
- **FA3 — sigma ≤ 0 para gaussmf (passo 7):** Sistema exibe "O desvio padrão deve ser maior que zero." Retorna ao passo 4.
- **FA4 — Parâmetros fora do universo de discurso (passo 7):** Sistema exibe aviso não bloqueante "Os parâmetros excedem o universo de discurso [min, max]. Deseja prosseguir?" Usuário confirma ou corrige. Retorna ao passo 6 ou prossegue ao passo 8.

**Pós-condições**

- Registro criado em `fuzzy_terms` associado ao `variable_id` correto.
- O Editor de Variáveis exibe o novo termo com o gráfico da função de pertinência renderizado.

---

## UC07

### Remover variável ou termo

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos uma variável ou termo existe no sistema. Editor de Variáveis está aberto. |

**Fluxo Principal — Remover variável**

1. Usuário localiza a variável que deseja remover no Editor de Variáveis.
2. Usuário clica em "Remover" ao lado da variável.
3. **Sistema** exibe diálogo de confirmação: "Remover a variável '[nome]' excluirá todos os seus termos e poderá tornar regras existentes inconsistentes. Confirmar?"
4. Usuário clica em "Confirmar".
5. **Sistema** envia DELETE para `/api/variables/{variable_id}`.
6. **Backend** executa `DELETE` em `fuzzy_variables`; `ON DELETE CASCADE` remove todos os `fuzzy_terms` vinculados.
7. **Backend** retorna status HTTP 204.
8. **Sistema** remove o painel da variável do editor sem recarregar a página.

**Fluxo Principal — Remover termo**

1. Usuário localiza o termo que deseja remover dentro de uma variável no editor.
2. Usuário clica em "Remover" ao lado do termo.
3. **Sistema** exibe diálogo de confirmação simples: "Remover o termo '[rótulo]'?"
4. Usuário clica em "Confirmar".
5. **Sistema** envia DELETE para `/api/terms/{term_id}`.
6. **Backend** executa `DELETE` em `fuzzy_terms`.
7. **Backend** retorna status HTTP 204.
8. **Sistema** remove o termo da listagem sem recarregar a página.

**Fluxos Alternativos**

- **FA1 — Usuário cancela o diálogo (passo 4 de qualquer fluxo):** Diálogo é fechado; nenhuma exclusão ocorre. Retorna ao passo 1 do fluxo correspondente.
- **FA2 — Falha no backend (passo 5 de qualquer fluxo):** Sistema exibe mensagem de erro. Retorna ao passo 2 do fluxo correspondente.

**Pós-condições**

- O registro removido e seus dependentes (se houver) não existem mais no banco de dados.
- O Editor de Variáveis reflete a remoção imediatamente.

---

## UC08

### Criar regra fuzzy via interface visual

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | O sistema possui ao menos uma variável antecedente com ao menos um termo e uma variável consequente com ao menos um termo. Editor de Regras está aberto. |

**Fluxo Principal**

1. Usuário clica em "Nova Regra" no Editor de Regras.
2. **Sistema** exibe o construtor visual de regras com: dropdown por variável antecedente para selecionar o termo ("é [termo]") ou "qualquer"; seletor de operador lógico entre antecedentes (AND / OR); dropdown para o consequente (variável de saída e seu termo); campo de peso (float [0.0, 1.0], padrão: 1.0).
3. **Sistema** exibe pré-visualização da regra em linguagem natural em tempo real. Exemplo: "SE Temperatura é Quente E Umidade é Alta ENTÃO Conforto é Desconfortável [peso: 1.0]".
4. Usuário configura a regra e clica em "Adicionar Regra".
5. **Sistema** valida: ao menos um antecedente foi selecionado (não é "qualquer"); consequente foi selecionado; peso está em [0.0, 1.0].
6. **Sistema** gera a `rule_text` padronizada e envia POST para `/api/systems/{id}/rules`.
7. **Backend** persiste em `fuzzy_rules` com `position` = (quantidade atual de regras + 1).
8. **Backend** retorna o objeto criado com status HTTP 201.
9. **Sistema** exibe a nova regra ao final da lista no Editor de Regras.

**Fluxos Alternativos**

- **FA1 — Nenhum antecedente selecionado (passo 5):** Sistema exibe "Selecione ao menos uma condição antecedente." Retorna ao passo 3.
- **FA2 — Consequente não selecionado (passo 5):** Sistema exibe "Selecione o termo consequente da regra." Retorna ao passo 3.
- **FA3 — Peso fora de [0.0, 1.0] (passo 5):** Sistema corrige automaticamente para o limite mais próximo e exibe aviso. Retorna ao passo 4.
- **FA4 — Falha no backend (passo 6):** Sistema exibe mensagem de erro. Retorna ao passo 4.

**Pós-condições**

- Novo registro em `fuzzy_rules` associado ao `system_id` com `rule_text`, `weight` e `position` definidos.

---

## UC09

### Editar regra existente

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos uma regra existe no sistema. Editor de Regras está aberto. |

**Fluxo Principal**

1. Usuário localiza a regra que deseja editar na listagem do Editor de Regras.
2. Usuário clica em "Editar" ao lado da regra.
3. **Sistema** busca os dados atuais da regra via GET `/api/rules/{rule_id}`.
4. **Sistema** exibe o construtor visual pré-preenchido com os valores atuais (antecedentes, operador, consequente e peso).
5. Usuário altera os campos desejados. A pré-visualização é atualizada em tempo real.
6. Usuário clica em "Salvar".
7. **Sistema** aplica as mesmas validações de UC08 (passo 5).
8. **Sistema** envia PUT para `/api/rules/{rule_id}` com os novos dados.
9. **Backend** executa `UPDATE` em `fuzzy_rules`, atualizando `rule_text` e os demais campos.
10. **Backend** retorna o objeto atualizado com status HTTP 200.
11. **Sistema** atualiza a exibição da regra na listagem.

**Fluxos Alternativos**

- **FA1 — Erros de validação (passo 7):** Comportamento idêntico a UC08. Retorna ao passo 5.
- **FA2 — Usuário cancela (qualquer passo):** Nenhuma alteração é persistida. Editor de Regras exibe os valores anteriores. Retorna ao passo 1.

**Pós-condições**

- O registro em `fuzzy_rules` reflete os novos valores definidos pelo usuário.

---

## UC10

### Remover regra

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos uma regra existe. Editor de Regras está aberto. |

**Fluxo Principal**

1. Usuário localiza a regra que deseja remover na listagem.
2. Usuário clica em "Remover" ao lado da regra.
3. **Sistema** exibe diálogo de confirmação: "Remover esta regra? Esta ação não pode ser desfeita."
4. Usuário clica em "Confirmar".
5. **Sistema** envia DELETE para `/api/rules/{rule_id}`.
6. **Backend** executa `DELETE` em `fuzzy_rules`.
7. **Backend** reordena o campo `position` das regras restantes.
8. **Backend** retorna status HTTP 204.
9. **Sistema** remove a regra da listagem e atualiza a numeração exibida.

**Fluxos Alternativos**

- **FA1 — Usuário cancela o diálogo (passo 4):** Diálogo é fechado; nenhuma exclusão ocorre. Retorna ao passo 1.
- **FA2 — Falha no backend (passo 5):** Sistema exibe mensagem de erro. A regra permanece na listagem. Retorna ao passo 2.

**Pós-condições**

- O registro em `fuzzy_rules` foi removido.
- As demais regras têm seus campos `position` atualizados sequencialmente.

---

## UC11

### Executar simulação com inputs manuais

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema / Frontend (secundário), Backend (secundário) |
| **Pré-condições** | O sistema possui ao menos uma variável antecedente com termos, uma variável consequente com termos e ao menos uma regra cadastrada. A tela Simulador está aberta. |

**Fluxo Principal**

1. Usuário visualiza os campos de input, um para cada variável antecedente, com o universo de discurso indicado como placeholder.
2. Usuário informa um valor numérico crisp para cada variável antecedente. Exemplo: Temperatura = 38, Umidade = 75.
3. Usuário clica em "Simular".
4. **Sistema** aciona UC15 (Validar sistema) antes de prosseguir.
5. **Sistema** envia POST para `/api/systems/{id}/simulate` com o objeto de inputs.
6. **Backend** carrega a configuração completa do sistema (variáveis, termos, regras) do banco de dados.
7. **Backend** executa o pipeline Mamdani via `logicfuzzy-academic`:
   - **Fuzzificação:** calcula o grau de pertinência de cada input em cada termo de sua variável.
   - **Avaliação de regras:** para cada regra, aplica o operador lógico (AND = mínimo, OR = máximo) e obtém o grau de ativação α.
   - **Implicação:** corta a MF do consequente pelo grau de ativação α (implicação mínimo).
   - **Agregação:** une (máximo) todos os consequentes ativados em um único conjunto fuzzy.
   - **Defuzzificação:** aplica o método configurado (ex.: centroide) ao conjunto agregado e obtém o valor crisp de saída.
8. **Backend** persiste a simulação em `simulations` com `inputs` e `outputs` em JSONB e `executed_at` com o timestamp atual.
9. **Backend** retorna o resultado com status HTTP 201.
10. **Sistema** exibe o valor de saída defuzzificado em destaque na tela.
11. **Sistema** aciona UC13 para exibir o pipeline visual completo.

**Fluxos Alternativos**

- **FA1 — UC15 falha (passo 4):** Simulação bloqueada. Sistema exibe a mensagem de erro de UC15 e orienta o usuário à tela correspondente. Não retorna ao fluxo principal.
- **FA2 — Input fora do universo de discurso (passo 2):** Sistema exibe aviso visual no campo e clampeia o valor ao limite ao enviar (passo 5). Prossegue ao passo 5 com valor corrigido.
- **FA3 — Nenhuma regra ativa para os inputs fornecidos (passo 7):** Backend retorna aviso. Sistema exibe "Nenhuma regra foi ativada para estes valores. Ajuste os inputs ou a base de regras." Nenhuma simulação é persistida. Retorna ao passo 2.
- **FA4 — Falha no backend (passo 5):** Sistema exibe mensagem de erro genérica. Nenhuma simulação é persistida. Retorna ao passo 3.

**Pós-condições**

- Novo registro em `simulations` com `system_id`, `inputs`, `outputs` e `executed_at`.
- O valor de saída defuzzificado é exibido na tela do Simulador.
- O pipeline visual (UC13) é renderizado automaticamente.

---

## UC12

### Buscar dados climáticos reais via OpenWeather

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Backend (secundário), OpenWeather API (ator externo) |
| **Pré-condições** | Tela Simulador está aberta. A variável `OPENWEATHER_API_KEY` está configurada corretamente no servidor. |

**Fluxo Principal**

1. Usuário digita o nome de uma cidade no campo "Buscar por cidade". Exemplo: "Belém".
2. Usuário clica em "Buscar clima".
3. **Sistema** valida que o campo não está vazio.
4. **Sistema** envia GET para `/api/weather?city=Belém` no backend.
5. **Backend** constrói a URL e chama a OpenWeather Current Weather API:
   ```
   GET https://api.openweathermap.org/data/2.5/weather?q=Belém&appid={KEY}&units=metric
   ```
6. **OpenWeather API** processa a requisição e retorna o JSON com os dados climáticos.
7. **Backend** extrai `main.temp` (°C) e `main.humidity` (%) do JSON retornado.
8. **Backend** retorna os valores ao frontend com status HTTP 200.
9. **Sistema** preenche automaticamente os campos de input correspondentes às variáveis de temperatura e umidade no Simulador.
10. **Sistema** exibe indicador visual informando que os dados foram preenchidos automaticamente e a cidade consultada.
11. Usuário revisa os valores e, se desejar, prossegue com a simulação (UC11).

**Fluxos Alternativos**

- **FA1 — Campo de cidade vazio (passo 3):** Sistema exibe "Informe o nome de uma cidade." Não realiza requisição. Retorna ao passo 1.
- **FA2 — Cidade não encontrada pela API — HTTP 404 (passo 6):** Backend repassa o erro. Sistema exibe "Cidade não encontrada. Verifique o nome e tente novamente." Retorna ao passo 1.
- **FA3 — Falha de rede ou timeout (passo 5):** Backend retorna erro. Sistema exibe "Não foi possível buscar dados climáticos. Insira os valores manualmente." Retorna ao passo 1.
- **FA4 — API key inválida ou expirada — HTTP 401 (passo 6):** Backend retorna erro de configuração. Sistema exibe "Erro de autenticação com o serviço climático. Contate o administrador." Retorna ao passo 1.
- **FA5 — Variáveis de temperatura/umidade não existem no sistema atual (passo 9):** Sistema exibe aviso "Nenhuma variável de temperatura ou umidade foi encontrada neste sistema. Os dados foram buscados mas não puderam ser preenchidos automaticamente." Os valores ficam disponíveis para cópia manual.

**Pós-condições**

- Os campos de input de temperatura e umidade do Simulador estão preenchidos com dados climáticos reais.
- O nome da cidade consultada está visível na interface.
- Os dados serão persistidos em `weather_data` e `city` de `simulations` quando UC11 for executado.

---

## UC13

### Visualizar pipeline completo da simulação

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Uma simulação foi executada com sucesso via UC11. Os dados de fuzzificação, ativação de regras, agregação e defuzzificação estão disponíveis no resultado retornado pelo backend. |

**Fluxo Principal**

1. Imediatamente após UC11 ser concluído com sucesso, **Sistema** renderiza automaticamente o painel de visualização do pipeline na tela do Simulador.
2. **Painel 1 — Fuzzificação:** para cada variável antecedente, exibe o gráfico com todas as funções de pertinência; marca o valor crisp do input com linha vertical; exibe os graus de pertinência calculados por termo (ex.: μ(Quente) = 0.76).
3. **Painel 2 — Regras Ativadas:** lista todas as regras do sistema com seu grau de ativação α; regras com α = 0 são exibidas como inativas (esmaecidas); regras com α > 0 são destacadas com o consequente ativado indicado.
4. **Painel 3 — Agregação:** exibe o conjunto fuzzy agregado da variável consequente (resultado da união de todos os consequentes ativados) com área sombreada.
5. **Painel 4 — Defuzzificação:** exibe o conjunto agregado com destaque para o ponto de saída crisp calculado; indica o método utilizado (ex.: "Centroide") e o valor numérico final.
6. Usuário pode interagir com cada painel individualmente (hover para ver valores exatos).

**Fluxos Alternativos**

- **FA1 — Nenhuma regra foi ativada — α = 0 para todas (passo 3):** Os painéis 3 e 4 exibem conjunto vazio com mensagem "Nenhuma regra foi ativada para estes inputs. O sistema não pôde produzir uma saída." Retorna ao passo 6.

**Pós-condições**

- O usuário possui visibilidade completa e interativa de cada etapa do processo de inferência Mamdani para a simulação executada.

---

## UC14

### Consultar histórico de simulações

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | O usuário está na tela Histórico com um sistema fuzzy selecionado. |

**Fluxo Principal**

1. **Sistema** realiza GET para `/api/systems/{id}/simulations` ordenado por `executed_at` decrescente.
2. **Sistema** exibe a listagem de simulações. Para cada item: data e hora de execução formatadas; cidade consultada (se disponível via UC12) ou "—"; resumo dos inputs (ex.: "Temp: 38°C | Umid: 75%"); valor de output defuzzificado.
3. Usuário clica em uma simulação para expandir seus detalhes.
4. **Sistema** exibe o painel de detalhes contendo: todos os inputs com seus valores; todos os outputs; dados climáticos completos (`weather_data` em JSON formatado), se disponíveis; método de defuzzificação utilizado.
5. Usuário clica em "Remover" em uma simulação que deseja excluir.
6. **Sistema** exibe diálogo de confirmação simples.
7. Usuário confirma.
8. **Sistema** envia DELETE para `/api/simulations/{simulation_id}`.
9. **Backend** remove o registro de `simulations`.
10. **Sistema** atualiza a listagem removendo o item excluído.

**Fluxos Alternativos**

- **FA1 — Nenhuma simulação registrada (passo 1):** Sistema exibe "Nenhuma simulação encontrada para este sistema. Execute uma simulação para começar." Retorna ao passo 1 após nova execução.
- **FA2 — Usuário cancela exclusão (passo 7):** Diálogo é fechado; simulação permanece no histórico. Retorna ao passo 2.
- **FA3 — Falha ao carregar histórico (passo 1):** Sistema exibe mensagem de erro e botão "Tentar novamente". Retorna ao passo 1.

**Pós-condições**

- O histórico permanece inalterado, exceto em caso de exclusão explícita confirmada pelo usuário.
- Simulações excluídas são permanentemente removidas de `simulations`.

---

## UC15

### Validar sistema antes de executar

| Campo | Descrição |
|---|---|
| **Atores** | Sistema / Frontend (primário), Backend (secundário) |
| **Pré-condições** | O usuário solicitou execução de simulação (UC11, passo 3). |

> Este caso de uso é disparado automaticamente pelo sistema, sem interação direta do usuário.

**Fluxo Principal**

1. **Sistema** verifica localmente (frontend) se o sistema fuzzy possui ao menos uma variável com `role = 'antecedent'`.
2. **Sistema** verifica se existe ao menos uma variável com `role = 'consequent'`.
3. **Sistema** verifica se cada variável cadastrada possui ao menos um termo linguístico associado.
4. **Sistema** verifica se existe ao menos uma regra cadastrada em `fuzzy_rules` para este sistema.
5. **Backend** realiza as mesmas verificações no lado do servidor antes de iniciar o pipeline (validação dupla — defense in depth).
6. Todas as verificações passam → **Backend** autoriza e inicia a execução do pipeline de UC11.

**Fluxos Alternativos**

- **FA1 — Sem variável antecedente (passo 1):** Simulação bloqueada. Sistema exibe "Este sistema não possui variáveis de entrada. Acesse o Editor de Variáveis e adicione ao menos uma variável antecedente com termos linguísticos." Não retorna ao fluxo principal.
- **FA2 — Sem variável consequente (passo 2):** Simulação bloqueada. Sistema exibe "Este sistema não possui variável de saída. Acesse o Editor de Variáveis e adicione uma variável consequente com termos linguísticos." Não retorna ao fluxo principal.
- **FA3 — Variável sem termos (passo 3):** Simulação bloqueada. Sistema exibe "A variável '[nome]' não possui termos linguísticos. Acesse o Editor de Variáveis e adicione ao menos um termo." Não retorna ao fluxo principal.
- **FA4 — Sem regras (passo 4):** Simulação bloqueada. Sistema exibe "Este sistema não possui regras. Acesse o Editor de Regras e crie ao menos uma regra antes de simular." Não retorna ao fluxo principal.
- **FA5 — Validação do backend falha após frontend aprovar (passo 5):** Backend retorna HTTP 422 com detalhes do erro. Sistema exibe a mensagem retornada. Não retorna ao fluxo principal.

**Pós-condições**

- **Sucesso:** Pipeline de simulação de UC11 é iniciado a partir do passo 6.
- **Falha:** Usuário recebe mensagem específica indicando qual requisito está faltando e é orientado à tela correspondente para corrigi-lo.
