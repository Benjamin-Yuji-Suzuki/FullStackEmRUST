# 📋 Casos de Uso — FuzzySimulated

> Especificação completa dos 17 casos de uso da plataforma, seguindo o padrão:
> ator(es), pré-condições, fluxo principal, fluxos alternativos (com retorno ao fluxo principal) e pós-condições.

**Projeto:** FuzzySimulated  
**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados — CESUPA 01/2026  
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
| [UC16](#uc16) | Carregar dataset Parquet e executar inferência em lote | Dashboard Batch | Usuário, Backend, Polars |
| [UC17](#uc17) | Renomear colunas do Parquet via dashboard | Gerenciamento de Variáveis do Dataset | Usuário, Sistema |

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
6. **Backend** executa `DELETE` em `fuzzy_systems`; `ON DELETE CASCADE` propaga automaticamente a exclusão para `fuzzy_variables`, `fuzzy_terms`, `fuzzy_rules`, `simulations` e `batch_results`.
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
| **Pré-condições** | Um sistema fuzzy foi criado (UC01). Usuário está no Editor de Variáveis do sistema. |

**Fluxo Principal**

1. Usuário clica em "Adicionar Variável de Entrada" no Editor de Variáveis.
2. **Sistema** exibe formulário com campos: Nome da variável (texto, obrigatório), Universo mínimo (float, obrigatório), Universo máximo (float, obrigatório), Resolução (inteiro, padrão: 501).
3. Usuário preenche os campos e clica em "Adicionar".
4. **Sistema** valida: Nome não vazio; Universo mínimo < Universo máximo; Resolução ≥ 2.
5. **Sistema** envia POST para `/api/systems/{id}/variables` com `role = 'antecedent'`.
6. **Backend** persiste o registro em `fuzzy_variables`.
7. **Backend** retorna o objeto criado com HTTP 201.
8. **Sistema** exibe a nova variável na lista do Editor de Variáveis, pronta para receber termos linguísticos (UC06).

**Fluxos Alternativos**

- **FA1 — Nome já existe neste sistema (passo 4):** Sistema exibe "Já existe uma variável com este nome neste sistema." Retorna ao passo 3.
- **FA2 — Universo mínimo ≥ máximo (passo 4):** Sistema exibe "O valor mínimo deve ser menor que o máximo." Retorna ao passo 3.
- **FA3 — Falha no backend (passo 5):** Sistema exibe erro genérico. Retorna ao passo 3.
- **FA4 — Usuário cancela:** Formulário é fechado sem persistir nada.

**Pós-condições**

- Novo registro em `fuzzy_variables` com `role = 'antecedent'` vinculado ao sistema.
- A variável aparece listada no Editor de Variáveis, aguardando termos linguísticos.

---

## UC05

### Adicionar variável consequente

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Um sistema fuzzy foi criado (UC01). Usuário está no Editor de Variáveis. Não existe ainda uma variável consequente cadastrada (o sistema Mamdani permite apenas uma). |

**Fluxo Principal**

1. Usuário clica em "Adicionar Variável de Saída" no Editor de Variáveis.
2. **Sistema** exibe formulário idêntico ao de UC04.
3. Usuário preenche os campos e clica em "Adicionar".
4. **Sistema** valida: Nome não vazio; Universo mínimo < Universo máximo; ausência de consequente já cadastrado.
5. **Sistema** envia POST para `/api/systems/{id}/variables` com `role = 'consequent'`.
6. **Backend** persiste o registro em `fuzzy_variables`.
7. **Backend** retorna o objeto criado com HTTP 201.
8. **Sistema** exibe a variável de saída em seção separada do Editor de Variáveis.

**Fluxos Alternativos**

- **FA1 — Já existe uma variável consequente (passo 4):** Sistema bloqueia a ação e exibe "Este sistema já possui uma variável de saída. Remova-a antes de adicionar outra (UC07)." Formulário é fechado.
- **FA2 — Universo inválido (passo 4):** Mesmo tratamento de UC04-FA2.
- **FA3 — Falha no backend (passo 5):** Mesmo tratamento de UC04-FA3.

**Pós-condições**

- Novo registro em `fuzzy_variables` com `role = 'consequent'` vinculado ao sistema.
- A variável de saída aparece na seção correspondente do Editor, aguardando termos linguísticos.

---

## UC06

### Adicionar termo linguístico a uma variável

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | A variável (antecedente ou consequente) foi criada. Usuário está no painel de termos da variável no Editor de Variáveis. |

**Fluxo Principal**

1. Usuário clica em "Adicionar Termo" na variável desejada.
2. **Sistema** exibe formulário com campos: Rótulo (texto, obrigatório), Tipo de função de pertinência (seleção: `trimf`, `trapmf`, `gaussmf`), Parâmetros (campos dinâmicos conforme o tipo escolhido).
3. Usuário preenche todos os campos e clica em "Adicionar".
4. **Sistema** valida: Rótulo não vazio; parâmetros numéricos coerentes para o tipo escolhido (ex.: para `trimf`, `a ≤ b ≤ c`); parâmetros dentro do universo de discurso da variável.
5. **Sistema** envia POST para `/api/variables/{variable_id}/terms`.
6. **Backend** persiste o registro em `fuzzy_terms` com `params` como JSONB.
7. **Backend** retorna o objeto criado com HTTP 201.
8. **Sistema** exibe o novo termo na lista e renderiza uma prévia do gráfico da função de pertinência atualizado.

**Fluxos Alternativos**

- **FA1 — Rótulo já existe nesta variável (passo 4):** Sistema exibe "Já existe um termo com este rótulo nesta variável." Retorna ao passo 3.
- **FA2 — Parâmetros incoerentes (passo 4):** Sistema exibe a regra violada (ex.: "Para trimf, os parâmetros devem satisfazer a ≤ b ≤ c"). Retorna ao passo 3.
- **FA3 — Parâmetros fora do universo (passo 4):** Sistema exibe aviso "Os parâmetros extrapolam o universo de discurso [min, max]. Confirma mesmo assim?" Usuário pode prosseguir ou ajustar.
- **FA4 — Falha no backend (passo 5):** Sistema exibe erro. Retorna ao passo 3.

**Pós-condições**

- Novo registro em `fuzzy_terms` vinculado à variável.
- O gráfico de pertinências da variável é atualizado na interface.

---

## UC07

### Remover variável ou termo

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | A variável ou o termo a ser removido existe no sistema. Usuário está no Editor de Variáveis. |

**Fluxo Principal**

1. Usuário clica no ícone de remoção ao lado de uma variável ou de um termo linguístico.
2. **Sistema** exibe diálogo de confirmação informando o que será removido e, no caso de variável, avisa que todos os seus termos associados também serão excluídos.
3. Usuário confirma a remoção.
4. **Sistema** envia DELETE para `/api/variables/{id}` (para variável) ou `/api/terms/{id}` (para termo).
5. **Backend** executa a exclusão; `ON DELETE CASCADE` remove os termos filhos quando a variável é excluída.
6. **Backend** retorna HTTP 204.
7. **Sistema** remove o item da listagem sem recarregar a página.

**Fluxos Alternativos**

- **FA1 — Usuário cancela no diálogo (passo 3):** Diálogo é fechado; nada é excluído.
- **FA2 — Variável referenciada em regras existentes (passo 4):** Backend retorna aviso. Sistema exibe "Esta variável é referenciada em [N] regras. Removê-la também invalidará essas regras. Confirma?" Usuário decide.
- **FA3 — Falha no backend (passo 4):** Sistema exibe mensagem de erro. Item permanece na listagem.

**Pós-condições**

- O registro removido não existe mais no banco de dados.
- Se uma variável foi removida, todos os seus `fuzzy_terms` também foram excluídos via CASCADE.
- Regras que referenciavam a variável ou termo removido passam a falhar na validação (UC15).

---

## UC08

### Criar regra fuzzy via interface visual

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | O sistema fuzzy possui ao menos uma variável antecedente com termos e uma variável consequente com termos. Usuário está no Editor de Regras. |

**Fluxo Principal**

1. Usuário clica em "Nova Regra" no Editor de Regras.
2. **Sistema** exibe construtor visual de regra com: seletor de variável antecedente, seletor de termo (com opção NOT), conector (AND/OR), e seletor de consequente/termo.
3. Usuário monta a regra selecionando antecedentes, conectores e consequente.
4. Usuário opcionalmente ajusta o peso da regra (float entre 0.0 e 1.0; padrão: 1.0).
5. Usuário clica em "Adicionar Regra".
6. **Sistema** gera o texto da regra no formato: `IF [var] IS [NOT] [termo] AND/OR ... THEN [var] IS [termo]`.
7. **Sistema** valida que a regra possui ao menos um antecedente e exatamente um consequente.
8. **Sistema** envia POST para `/api/systems/{id}/rules` com `rule_text` e `weight`.
9. **Backend** persiste o registro em `fuzzy_rules`.
10. **Backend** retorna o objeto criado com HTTP 201.
11. **Sistema** exibe a nova regra na lista do Editor de Regras na última posição.

**Fluxos Alternativos**

- **FA1 — Nenhum antecedente selecionado (passo 7):** Sistema exibe "A regra precisa de ao menos uma condição (antecedente)." Retorna ao passo 3.
- **FA2 — Nenhum consequente selecionado (passo 7):** Sistema exibe "A regra precisa de exatamente uma conclusão (consequente)." Retorna ao passo 3.
- **FA3 — Regra idêntica já existe (passo 8):** Backend retorna conflito. Sistema exibe aviso "Esta regra já existe." Retorna ao passo 3.
- **FA4 — Falha no backend (passo 8):** Sistema exibe erro. Retorna ao passo 5.

**Pós-condições**

- Novo registro em `fuzzy_rules` vinculado ao sistema.
- A regra aparece listada no Editor de Regras.

---

## UC09

### Editar regra existente

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos uma regra existe no sistema. Usuário está no Editor de Regras. |

**Fluxo Principal**

1. Usuário clica em "Editar" na regra desejada.
2. **Sistema** carrega a regra no construtor visual, pré-preenchendo antecedentes, conectores, consequente e peso.
3. Usuário altera os campos desejados.
4. Usuário clica em "Salvar".
5. **Sistema** valida a regra da mesma forma que UC08 (passo 7).
6. **Sistema** envia PUT para `/api/rules/{id}` com os dados atualizados.
7. **Backend** executa UPDATE em `fuzzy_rules`.
8. **Backend** retorna o objeto atualizado com HTTP 200.
9. **Sistema** atualiza a exibição da regra na lista.

**Fluxos Alternativos**

- **FA1 — Validação falha (passo 5):** Mesmo tratamento de UC08-FA1/FA2.
- **FA2 — Usuário cancela:** Construtor é fechado sem persistir alterações.
- **FA3 — Falha no backend (passo 6):** Sistema exibe erro. Retorna ao passo 4.

**Pós-condições**

- O registro em `fuzzy_rules` reflete as alterações do usuário.
- A lista de regras exibe a versão atualizada.

---

## UC10

### Remover regra

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário) |
| **Pré-condições** | Ao menos uma regra existe no sistema. Usuário está no Editor de Regras. |

**Fluxo Principal**

1. Usuário clica no ícone de remoção ao lado de uma regra.
2. **Sistema** exibe diálogo de confirmação: "Remover esta regra? Esta ação não pode ser desfeita."
3. Usuário confirma.
4. **Sistema** envia DELETE para `/api/rules/{id}`.
5. **Backend** remove o registro de `fuzzy_rules`.
6. **Backend** retorna HTTP 204.
7. **Sistema** remove a regra da lista.

**Fluxos Alternativos**

- **FA1 — Usuário cancela (passo 3):** Diálogo é fechado; regra permanece.
- **FA2 — Falha no backend (passo 4):** Sistema exibe erro. Regra permanece na lista.

**Pós-condições**

- O registro da regra foi permanentemente removido de `fuzzy_rules`.
- A lista de regras não exibe mais a regra excluída.

---

## UC11

### Executar simulação com inputs manuais

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema (secundário), Backend (terciário) |
| **Pré-condições** | O sistema fuzzy passou pela validação de UC15 (ao menos 1 antecedente com termos, 1 consequente com termos, ao menos 1 regra). Usuário está na tela Simulador. |

**Fluxo Principal**

1. **Sistema** exibe um campo de input para cada variável antecedente do sistema, com o universo de discurso indicado.
2. Usuário preenche manualmente os valores numéricos para cada input.
3. Usuário clica em "Executar Simulação".
4. **Sistema** executa UC15 (validação).
5. **Sistema** envia POST para `/api/systems/{id}/simulate` com `inputs: { [nome_var]: valor, ... }`.
6. **Backend** instancia o `MamdaniEngine` do `logicfuzzy-academic`, configura variáveis e regras conforme o banco, injeta os inputs e executa `engine.compute()`.
7. **Backend** persiste o resultado na tabela `simulations` com `inputs`, `outputs`, `executed_at`.
8. **Backend** retorna o resultado completo (outputs crisp, dados de fuzzificação, graus de ativação de regras, conjunto agregado) com HTTP 200.
9. **Sistema** exibe o valor de saída defuzzificado de forma destacada.
10. **Sistema** renderiza automaticamente o painel de pipeline (UC13).

**Fluxos Alternativos**

- **FA1 — Input fora do universo de discurso (passo 2):** Sistema exibe aviso em tempo real ao lado do campo. Usuário pode prosseguir, mas o backend tratará o valor como inválido e retornará erro `InputOutOfRange`. Retorna ao passo 2.
- **FA2 — Campo de input vazio (passo 3):** Sistema bloqueia o envio e destaca os campos não preenchidos. Retorna ao passo 2.
- **FA3 — Validação UC15 falha (passo 4):** Simulação bloqueada; mensagem específica é exibida (ver UC15).
- **FA4 — Nenhuma regra foi ativada — `NoRulesFired` (passo 6):** Backend retorna erro específico. Sistema exibe "Nenhuma regra foi ativada para estes valores de entrada. O sistema não pôde produzir uma saída." Retorna ao passo 2.
- **FA5 — Falha no backend (passo 5):** Sistema exibe erro genérico. Retorna ao passo 3.

**Pós-condições**

- Um novo registro existe em `simulations` com os inputs, outputs e timestamp.
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

---

## UC16

### Carregar dataset Parquet e executar inferência em lote

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Backend / Axum (secundário), Polars (ator interno) |
| **Pré-condições** | Um sistema fuzzy válido está selecionado (passou em UC15). O usuário possui um arquivo `.parquet` com colunas numéricas mapeáveis às variáveis antecedentes do sistema. Colunas com nomes inválidos já foram renomeadas via UC17 (opcional). Usuário está no Dashboard Batch. |

**Fluxo Principal**

1. Usuário seleciona um sistema fuzzy no Dashboard Batch.
2. **Sistema** exibe painel de upload e a lista de variáveis antecedentes do sistema selecionado.
3. Usuário seleciona o arquivo `.parquet` e clica em "Carregar".
4. **Sistema** valida localmente: extensão `.parquet`; tamanho ≤ limite configurado (padrão: 50 MB).
5. **Sistema** lê os nomes das colunas do Parquet (via prévia no frontend) e exibe a interface de mapeamento: para cada variável antecedente do sistema, o usuário seleciona a coluna correspondente no dataset.
6. Usuário confirma o mapeamento e clica em "Processar em Lote".
7. **Sistema** envia POST para `/api/batch/upload` com o arquivo e o mapeamento de colunas via multipart form-data.
8. **Backend** recebe o arquivo e executa em `spawn_blocking` (thread pool separado, sem bloquear o runtime Tokio):
   - Lê o Parquet com Polars.
   - Aplica o mapeamento de colunas recebido.
   - Valida que as colunas mapeadas existem e são numéricas.
   - Itera sobre cada linha do DataFrame.
   - Para cada linha: injeta os valores mapeados como inputs no `MamdaniEngine`; executa `engine.compute()`; coleta o output defuzzificado.
9. **Backend** persiste todos os resultados em `batch_results` em uma transação única.
10. **Backend** retorna resumo com HTTP 200: linhas processadas, erros (`NoRulesFired`), distribuição dos outputs por faixa.
11. **Sistema** exibe o resumo no Dashboard Batch: gráfico de distribuição, tabela dos N casos de maior output, total processado vs. erros.
12. Usuário pode clicar em "Ver todos os resultados" para navegar pela tabela completa de `batch_results`.

**Fluxos Alternativos**

- **FA1 — Arquivo não é um Parquet válido (passo 8):** Backend retorna HTTP 422. Sistema exibe "O arquivo não é um Parquet válido ou está corrompido." Retorna ao passo 3.
- **FA2 — Coluna mapeada ausente ou não numérica (passo 8):** Backend retorna HTTP 422 com detalhes. Sistema exibe "A coluna '[nome]' não foi encontrada ou não é numérica." Retorna ao passo 5.
- **FA3 — Linhas com valores nulos ou fora do universo (passo 8):** Backend registra a linha com `error: true` em `batch_results.inputs` e prossegue. O resumo final informa quantas linhas foram puladas.
- **FA4 — Todas as linhas falharam com `NoRulesFired` (passo 8):** Backend retorna HTTP 200 com `processed: N, errors: N`. Sistema exibe "Nenhuma linha gerou saída válida. Verifique se o sistema fuzzy está configurado para o intervalo de valores do dataset." Retorna ao passo 1.
- **FA5 — Arquivo excede o limite de tamanho (passo 4):** Sistema exibe "O arquivo excede o tamanho máximo permitido (50 MB)." Não realiza upload. Retorna ao passo 3.
- **FA6 — Falha na transação de persistência (passo 9):** Backend faz rollback. Sistema exibe "Erro ao salvar os resultados. Nenhum dado parcial foi gravado. Tente novamente." Retorna ao passo 6.
- **FA7 — Sistema fuzzy não passou em UC15 (passo 1):** Sistema bloqueia a seleção e exibe "O sistema fuzzy selecionado está incompleto. Configure variáveis, termos e regras antes de processar em lote." Não permite prosseguir.

**Pós-condições**

- Os resultados de todas as linhas processadas com sucesso estão persistidos em `batch_results`, vinculados ao `system_id` e ao arquivo fonte (`source_file`).
- O Dashboard Batch exibe a distribuição dos outputs e os casos de maior criticidade.
- O usuário pode acessar os resultados em sessões futuras via consulta a `batch_results`.

---

## UC17

### Renomear colunas do Parquet via dashboard

| Campo | Descrição |
|---|---|
| **Atores** | Usuário (primário), Sistema / Frontend (secundário) |
| **Pré-condições** | Um arquivo `.parquet` foi carregado ou está em processo de carregamento no Dashboard Batch (UC16, passo 3–5). O dataset contém colunas com nomes que possuem caracteres especiais, espaços ou abreviações incompatíveis com as variáveis fuzzy do sistema. |

> Este caso de uso é executado dentro do fluxo do UC16, antes do mapeamento de colunas (passo 5). Toda a operação ocorre no frontend — nenhuma persistência no banco de dados é realizada; o mapeamento é mantido em memória durante a sessão de upload.

**Fluxo Principal**

1. Após carregar o arquivo `.parquet` (UC16, passo 3), **Sistema** exibe a tabela de colunas detectadas: nome original da coluna, tipo inferido (float, int, string) e prévia dos primeiros valores.
2. Usuário identifica uma coluna com nome inválido ou incompatível (ex.: `"impacto financeiro ($)"`, `"imp_fin_2024"`, `"coluna com espaço"`).
3. Usuário clica no campo de nome editável ao lado da coluna desejada.
4. **Sistema** habilita o campo de texto com o nome original pré-preenchido.
5. Usuário digita o novo nome normalizado (ex.: `"impacto_financeiro"`).
6. **Sistema** valida o novo nome em tempo real: apenas letras, números e underscores são permitidos; o nome não pode estar em branco nem duplicar outro nome já atribuído na sessão.
7. Usuário confirma pressionando Enter ou clicando fora do campo.
8. **Sistema** atualiza o nome exibido na tabela com o novo valor e marca a coluna como "renomeada" visualmente.
9. Usuário repete os passos 3–8 para quantas colunas desejar.
10. Após concluir as renomeações, usuário prossegue para o mapeamento de colunas (UC16, passo 5), onde os novos nomes já aparecem como opções disponíveis.

**Fluxos Alternativos**

- **FA1 — Nome inválido (caracteres especiais) (passo 6):** Sistema exibe inline "Use apenas letras, números e underscores." O campo fica em estado de erro até correção. Retorna ao passo 5.
- **FA2 — Nome duplicado (passo 6):** Sistema exibe "Este nome já está em uso por outra coluna." Retorna ao passo 5.
- **FA3 — Nome em branco (passo 6):** Sistema exibe "O nome não pode ser vazio." Retorna ao passo 5.
- **FA4 — Usuário deseja desfazer a renomeação (passo 8):** Usuário clica em "Restaurar original" ao lado da coluna. **Sistema** reverte o nome para o valor original do Parquet. Retorna ao passo 1.
- **FA5 — Usuário não renomeia nenhuma coluna:** Todos os nomes originais são utilizados diretamente no mapeamento de UC16. O UC17 é opcional.

**Pós-condições**

- O mapeamento de renomeação está armazenado em memória no frontend (estrutura `coluna_original → nome_normalizado`).
- As colunas renomeadas aparecem com seus novos nomes na interface de mapeamento do UC16.
- Nenhuma alteração é persistida no banco de dados; o arquivo Parquet original não é modificado.
- Ao encerrar ou cancelar o upload, o mapeamento de renomeação é descartado.
