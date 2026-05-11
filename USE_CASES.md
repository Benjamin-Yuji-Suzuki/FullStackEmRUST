# 📋 Casos de Uso — FuzzySimulated

> Documento de especificação dos 15 casos de uso da plataforma, seguindo o padrão: ator, pré-condições, fluxo principal, fluxos alternativos e pós-condições.

**Projeto:** FuzzySimulated
**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional — CESUPA 01/2026
**Repositório:** https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST

---

## Índice

| ID | Nome | Tela |
|---|---|---|
| [UC01](#uc01) | Criar novo sistema fuzzy | Dashboard |
| [UC02](#uc02) | Editar metadados de um sistema | Dashboard |
| [UC03](#uc03) | Excluir sistema fuzzy | Dashboard |
| [UC04](#uc04) | Adicionar variável antecedente | Editor de Variáveis |
| [UC05](#uc05) | Adicionar variável consequente | Editor de Variáveis |
| [UC06](#uc06) | Adicionar termo linguístico a uma variável | Editor de Variáveis |
| [UC07](#uc07) | Remover variável ou termo | Editor de Variáveis |
| [UC08](#uc08) | Criar regra fuzzy via interface visual | Editor de Regras |
| [UC09](#uc09) | Editar regra existente | Editor de Regras |
| [UC10](#uc10) | Remover regra | Editor de Regras |
| [UC11](#uc11) | Executar simulação com inputs manuais | Simulador |
| [UC12](#uc12) | Buscar dados climáticos reais via OpenWeather | Simulador |
| [UC13](#uc13) | Visualizar pipeline completo da simulação | Simulador |
| [UC14](#uc14) | Consultar histórico de simulações | Histórico |
| [UC15](#uc15) | Validar sistema antes de executar | Sistema (automático) |

---

## UC01

### Criar novo sistema fuzzy

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Usuário está na tela Dashboard |

**Fluxo Principal**

1. Usuário clica em "Novo Sistema".
2. Sistema exibe formulário com campos: Nome (obrigatório), Descrição (opcional) e Método de defuzzificação (seleção: `centroid`, `bisector`, `mom`, `lom`, `som`).
3. Usuário preenche os campos e confirma.
4. Sistema valida que o campo Nome não está vazio.
5. Sistema persiste o novo registro em `fuzzy_systems` e retorna o UUID gerado.
6. Sistema redireciona o usuário para o Editor de Variáveis do sistema recém-criado.
7. O novo sistema aparece listado no Dashboard.

**Fluxos Alternativos**

- **FA1 — Nome vazio:** Sistema exibe mensagem de erro inline e não persiste o registro.
- **FA2 — Falha de conexão com o banco:** Sistema exibe mensagem de erro genérica; nenhum registro é criado.

**Pós-condições**

- Um novo registro existe em `fuzzy_systems` com `id`, `name`, `defuzz_method`, `created_at` e `updated_at`.
- O usuário está na tela do Editor de Variáveis do sistema criado.

---

## UC02

### Editar metadados de um sistema

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Ao menos um sistema fuzzy existe; usuário está no Dashboard |

**Fluxo Principal**

1. Usuário localiza o sistema desejado na listagem do Dashboard.
2. Usuário clica em "Editar" no card do sistema.
3. Sistema exibe formulário pré-preenchido com Nome, Descrição e Método de defuzzificação atuais.
4. Usuário altera um ou mais campos e confirma.
5. Sistema valida que Nome não está vazio.
6. Sistema executa `UPDATE` em `fuzzy_systems`, atualizando também `updated_at`.
7. Dashboard atualiza o card com os novos valores.

**Fluxos Alternativos**

- **FA1 — Usuário cancela:** Formulário é descartado; nenhuma alteração é persistida.
- **FA2 — Nome limpo antes de salvar:** Sistema exibe erro e bloqueia o envio.

**Pós-condições**

- O registro em `fuzzy_systems` reflete os novos valores e `updated_at` foi atualizado.

---

## UC03

### Excluir sistema fuzzy

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Ao menos um sistema fuzzy existe; usuário está no Dashboard |

**Fluxo Principal**

1. Usuário clica em "Excluir" no card do sistema desejado.
2. Sistema exibe diálogo de confirmação alertando que variáveis, termos, regras e simulações associados também serão removidos.
3. Usuário confirma a exclusão.
4. Sistema executa `DELETE` em `fuzzy_systems`; o `ON DELETE CASCADE` propaga a exclusão para `fuzzy_variables`, `fuzzy_terms`, `fuzzy_rules` e `simulations`.
5. Sistema retorna ao Dashboard; o card do sistema removido desaparece da lista.

**Fluxos Alternativos**

- **FA1 — Usuário cancela no diálogo:** Nenhuma exclusão é realizada; Dashboard permanece inalterado.

**Pós-condições**

- O sistema e todos os seus dados dependentes foram removidos do banco de dados.

---

## UC04

### Adicionar variável antecedente

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Um sistema fuzzy existe e está aberto no Editor de Variáveis |

**Fluxo Principal**

1. Usuário clica em "Adicionar Variável de Entrada".
2. Sistema exibe formulário com campos: Nome, Universo mínimo, Universo máximo e Resolução (padrão: 501 pontos).
3. Usuário preenche os campos e confirma.
4. Sistema valida: Nome não vazio; `universe_min < universe_max`; resolução ≥ 2.
5. Sistema persiste em `fuzzy_variables` com `role = 'antecedent'`.
6. A nova variável aparece na seção "Entradas" do editor, pronta para receber termos linguísticos.

**Fluxos Alternativos**

- **FA1 — `universe_min ≥ universe_max`:** Sistema exibe erro "O limite mínimo deve ser menor que o máximo".
- **FA2 — Nome duplicado no mesmo sistema:** Sistema exibe aviso; usuário pode prosseguir ou renomear.

**Pós-condições**

- Registro criado em `fuzzy_variables` com `role = 'antecedent'` vinculado ao `system_id` correto.

---

## UC05

### Adicionar variável consequente

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Um sistema fuzzy existe e está aberto no Editor de Variáveis |

**Fluxo Principal**

1. Usuário clica em "Adicionar Variável de Saída".
2. Sistema exibe o mesmo formulário de UC04.
3. Usuário preenche os campos e confirma.
4. Sistema valida as mesmas regras de UC04.
5. Sistema persiste em `fuzzy_variables` com `role = 'consequent'`.
6. A nova variável aparece na seção "Saídas" do editor.

**Fluxos Alternativos**

- **FA1 — Tentativa de adicionar segunda saída quando o motor fuzzy atual suporta apenas uma:** Sistema exibe aviso informativo; prossegue normalmente (suporte a múltiplas saídas depende da versão da biblioteca).

**Pós-condições**

- Registro criado em `fuzzy_variables` com `role = 'consequent'` vinculado ao `system_id`.

---

## UC06

### Adicionar termo linguístico a uma variável

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Ao menos uma variável existe no sistema; Editor de Variáveis está aberto |

**Fluxo Principal**

1. Usuário seleciona uma variável e clica em "Adicionar Termo".
2. Sistema exibe formulário com campos: Rótulo (label), Tipo de função de pertinência (`trimf`, `trapmf`, `gaussmf`) e campos de parâmetros dinâmicos de acordo com o tipo escolhido.
3. Usuário preenche os campos. Exemplos de parâmetros:
   - `trimf`: [a, b, c] onde a ≤ b ≤ c
   - `trapmf`: [a, b, c, d] onde a ≤ b ≤ c ≤ d
   - `gaussmf`: [mean, sigma] onde sigma > 0
4. Sistema valida os parâmetros quanto à consistência com o universo de discurso da variável.
5. Sistema persiste em `fuzzy_terms` com `params` armazenado como JSONB.
6. O novo termo aparece listado sob a variável, com prévia gráfica da função de pertinência.

**Fluxos Alternativos**

- **FA1 — Parâmetros fora do universo:** Sistema exibe aviso (não bloqueia, pois pode ser intencional em alguns casos).
- **FA2 — Rótulo vazio:** Sistema bloqueia e exibe erro.
- **FA3 — sigma ≤ 0 para gaussmf:** Sistema bloqueia e exibe erro.

**Pós-condições**

- Registro criado em `fuzzy_terms` associado ao `variable_id` correto.

---

## UC07

### Remover variável ou termo

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Ao menos uma variável ou termo existe; Editor de Variáveis está aberto |

**Fluxo Principal — Remover variável**

1. Usuário clica em "Remover" ao lado de uma variável.
2. Sistema exibe diálogo de confirmação informando que todos os termos da variável serão excluídos e que regras que referenciam esta variável poderão ficar inconsistentes.
3. Usuário confirma.
4. Sistema executa `DELETE` em `fuzzy_variables`; `ON DELETE CASCADE` remove todos os `fuzzy_terms` associados.
5. O editor atualiza a listagem de variáveis.

**Fluxo Principal — Remover termo**

1. Usuário clica em "Remover" ao lado de um termo específico.
2. Sistema exibe diálogo de confirmação simples.
3. Usuário confirma.
4. Sistema executa `DELETE` em `fuzzy_terms`.
5. O editor atualiza a listagem de termos da variável.

**Fluxos Alternativos**

- **FA1 — Cancelamento:** Nenhuma exclusão ocorre.

**Pós-condições**

- O registro removido e seus dependentes não existem mais no banco de dados.

---

## UC08

### Criar regra fuzzy via interface visual

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | O sistema possui ao menos uma variável antecedente com termos e uma variável consequente com termos; Editor de Regras está aberto |

**Fluxo Principal**

1. Usuário clica em "Nova Regra".
2. Sistema exibe construtor visual de regras com:
   - Seletores dropdown para cada variável antecedente e seus termos (antecedentes da regra).
   - Operador lógico entre antecedentes (AND / OR).
   - Seletor dropdown para o consequente (variável de saída e seu termo).
   - Campo de peso (float entre 0.0 e 1.0, padrão 1.0).
3. Usuário configura a regra (ex.: "SE Temperatura é Quente E Umidade é Alta ENTÃO Conforto é Desconfortável", peso 1.0).
4. Sistema gera a `rule_text` em formato textual padronizado.
5. Sistema valida que ao menos um antecedente e um consequente foram selecionados.
6. Sistema persiste em `fuzzy_rules` com `position` = último índice + 1.
7. A regra aparece na lista do Editor de Regras.

**Fluxos Alternativos**

- **FA1 — Nenhum antecedente selecionado:** Sistema bloqueia com mensagem de erro.
- **FA2 — Peso fora do intervalo [0, 1]:** Sistema normaliza ou exibe erro.

**Pós-condições**

- Novo registro em `fuzzy_rules` associado ao `system_id`.

---

## UC09

### Editar regra existente

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Ao menos uma regra existe; Editor de Regras está aberto |

**Fluxo Principal**

1. Usuário clica em "Editar" ao lado de uma regra.
2. Sistema exibe o construtor visual pré-preenchido com os valores atuais da regra.
3. Usuário altera antecedentes, consequente, operador ou peso.
4. Sistema valida os campos (mesmas validações de UC08).
5. Sistema executa `UPDATE` em `fuzzy_rules` e atualiza a `rule_text`.
6. A regra atualizada aparece na lista.

**Fluxos Alternativos**

- **FA1 — Usuário cancela:** Nenhuma alteração é persistida.

**Pós-condições**

- O registro em `fuzzy_rules` reflete os novos valores.

---

## UC10

### Remover regra

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Ao menos uma regra existe; Editor de Regras está aberto |

**Fluxo Principal**

1. Usuário clica em "Remover" ao lado de uma regra.
2. Sistema exibe diálogo de confirmação.
3. Usuário confirma.
4. Sistema executa `DELETE` em `fuzzy_rules`.
5. A regra desaparece da lista; as posições das demais são reordenadas.

**Fluxos Alternativos**

- **FA1 — Cancelamento:** Nenhuma exclusão ocorre.

**Pós-condições**

- O registro removido não existe mais em `fuzzy_rules`.

---

## UC11

### Executar simulação com inputs manuais

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | O sistema passou pela validação de UC15 (variáveis, termos e regras presentes); tela Simulador está aberta |

**Fluxo Principal**

1. Usuário informa valores numéricos crisp para cada variável antecedente nos campos de input.
2. Usuário clica em "Simular".
3. Sistema chama UC15 para validação prévia.
4. Backend executa o pipeline Mamdani via `logicfuzzy-academic`:
   - Fuzzificação de cada input.
   - Avaliação de todas as regras (cálculo dos graus de ativação).
   - Agregação dos consequentes ativados.
   - Defuzzificação pelo método configurado no sistema.
5. Sistema persiste a simulação em `simulations` com `inputs` e `outputs` em JSONB.
6. Frontend exibe o valor de saída defuzzificado e os gráficos do pipeline (UC13).

**Fluxos Alternativos**

- **FA1 — Input fora do universo de discurso:** Sistema exibe aviso e clampeia o valor ao limite do universo.
- **FA2 — Nenhuma regra ativa para os inputs fornecidos:** Sistema retorna saída nula e exibe aviso "Nenhuma regra foi ativada para estes valores".
- **FA3 — Falha no backend:** Sistema exibe mensagem de erro; nenhuma simulação é persistida.

**Pós-condições**

- Novo registro em `simulations` com `inputs`, `outputs` e `executed_at`.
- Resultados exibidos na tela do Simulador.

---

## UC12

### Buscar dados climáticos reais via OpenWeather

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Tela Simulador está aberta; `OPENWEATHER_API_KEY` configurada no servidor |

**Fluxo Principal**

1. Usuário digita o nome de uma cidade no campo "Buscar por cidade" (ex.: "Belém").
2. Usuário clica em "Buscar clima".
3. Backend realiza requisição à OpenWeather Current Weather API:
   ```
   GET https://api.openweathermap.org/data/2.5/weather?q=Belém&appid={KEY}&units=metric
   ```
4. API retorna `temp` (°C) e `humidity` (%).
5. Backend repassa os valores ao frontend.
6. Frontend preenche automaticamente os campos de input correspondentes às variáveis de temperatura e umidade.
7. Usuário pode confirmar os valores e prosseguir com a simulação (UC11).

**Fluxos Alternativos**

- **FA1 — Cidade não encontrada (HTTP 404):** Sistema exibe "Cidade não encontrada. Verifique o nome e tente novamente."
- **FA2 — Falha de rede ou timeout:** Sistema exibe "Não foi possível buscar dados climáticos. Insira os valores manualmente."
- **FA3 — API key inválida (HTTP 401):** Sistema exibe erro de configuração (visível apenas em modo de desenvolvimento).

**Pós-condições**

- Os campos de input do Simulador estão preenchidos com dados reais.
- `weather_data` e `city` serão persistidos em `simulations` após execução de UC11.

---

## UC13

### Visualizar pipeline completo da simulação

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Uma simulação foi executada com sucesso (UC11) |

**Fluxo Principal**

1. Após a execução de UC11, o Simulador exibe automaticamente o painel de visualização do pipeline.
2. **Painel de Fuzzificação:** para cada variável antecedente, exibe o gráfico das funções de pertinência com o valor crisp marcado e os graus de pertinência calculados por termo.
3. **Painel de Regras Ativadas:** lista as regras disparadas com seus respectivos graus de ativação (α); regras com α = 0 aparecem como inativas.
4. **Painel de Agregação:** exibe o conjunto fuzzy agregado resultante da união de todos os consequentes ativados.
5. **Painel de Defuzzificação:** exibe o conjunto agregado com o valor crisp de saída destacado (ponto centroide ou método configurado).
6. Usuário pode inspecionar cada painel individualmente.

**Fluxos Alternativos**

- **FA1 — Nenhuma regra foi ativada:** Painéis de agregação e defuzzificação exibem conjunto vazio com aviso explicativo.

**Pós-condições**

- Usuário possui visibilidade completa de cada etapa do processo de inferência fuzzy.

---

## UC14

### Consultar histórico de simulações

| Campo | Descrição |
|---|---|
| **Ator** | Usuário |
| **Pré-condições** | Ao menos uma simulação foi executada; usuário está na tela Histórico |

**Fluxo Principal**

1. Sistema lista todas as simulações do sistema selecionado, ordenadas por `executed_at` decrescente.
2. Cada item exibe: data/hora, cidade (se preenchida), valores de input resumidos e valor de output.
3. Usuário clica em uma simulação para expandir os detalhes.
4. Sistema exibe: todos os inputs, todos os outputs, `weather_data` completo (se disponível) e método de defuzzificação utilizado.
5. Usuário pode excluir uma simulação clicando em "Remover" (confirmação necessária).

**Fluxos Alternativos**

- **FA1 — Nenhuma simulação registrada:** Sistema exibe mensagem "Nenhuma simulação encontrada para este sistema."

**Pós-condições**

- O histórico permanece inalterado (exceto em caso de exclusão explícita).

---

## UC15

### Validar sistema antes de executar

| Campo | Descrição |
|---|---|
| **Ator** | Sistema (acionado automaticamente antes de UC11) |
| **Pré-condições** | Usuário solicitou execução de simulação |

**Fluxo Principal**

1. Sistema verifica se o sistema fuzzy possui ao menos uma variável com `role = 'antecedent'`.
2. Sistema verifica se existe ao menos uma variável com `role = 'consequent'`.
3. Sistema verifica se cada variável possui ao menos um termo linguístico cadastrado.
4. Sistema verifica se existe ao menos uma regra cadastrada em `fuzzy_rules`.
5. Todas as verificações passam → sistema autoriza a execução de UC11.

**Fluxos Alternativos**

- **FA1 — Sem variável antecedente:** Simulação bloqueada; sistema exibe "Adicione ao menos uma variável de entrada com termos linguísticos."
- **FA2 — Sem variável consequente:** Simulação bloqueada; sistema exibe "Adicione ao menos uma variável de saída com termos linguísticos."
- **FA3 — Variável sem termos:** Simulação bloqueada; sistema exibe "A variável '[nome]' não possui termos linguísticos."
- **FA4 — Sem regras:** Simulação bloqueada; sistema exibe "Adicione ao menos uma regra à base de regras."

**Pós-condições**

- Em caso de sucesso: pipeline de simulação é iniciado (UC11).
- Em caso de falha: usuário é redirecionado ou orientado à tela correspondente para corrigir a inconsistência.
