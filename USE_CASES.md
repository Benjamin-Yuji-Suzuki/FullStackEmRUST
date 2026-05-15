# 📋 Casos de Uso — FuzzySimulated

> Especificação dos 16 casos de uso da plataforma, seguindo o padrão UML:
> ator(es), pré-condições, fluxo principal, fluxos alternativos (com retorno ao fluxo principal) e pós-condições.
> Atores são SEMPRE entidades externas à fronteira do sistema.

**Projeto:** FuzzySimulated  
**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados — CESUPA 01/2026  
**Repositório:** https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST

---

## Índice

| ID | Nome | Ator(es) |
|---|---|---|
| [UC01](#uc01) | Gerenciar Sistemas Fuzzy | Usuário |
| [UC02](#uc02) | Gerenciar Variáveis e Termos | Usuário |
| [UC03](#uc03) | Gerenciar Regras Fuzzy | Usuário |
| [UC04](#uc04) | Executar Simulação | Usuário |
| [UC05](#uc05) | Buscar Dados Climáticos | Usuário, OpenWeather API |
| [UC06](#uc06) | Consultar Histórico de Simulações | Usuário |
| [UC07](#uc07) | Processar Inferência em Lote | Usuário |
| [UC08](#uc08) | Comparar Simulações | Usuário |
| [UC09](#uc09) | Exportar Relatório de Simulação | Usuário |
| [UC10](#uc10) | Duplicar Sistema Fuzzy | Usuário |
| [UC11](#uc11) | Exportar e Importar Sistema | Usuário |
| [UC12](#uc12) | Salvar Cenário de Simulação | Usuário |
| [UC13](#uc13) | Executar Varredura de Entrada | Usuário |
| [UC14](#uc14) | Visualizar Matriz de Regras Ativadas | Usuário |
| [UC15](#uc15) | Visualizar Superfície de Controle | Usuário |
| [UC16](#uc16) | Gerenciar Histórico de Alterações | Usuário |
| [UC17](#uc17) | Otimizar Parâmetros com PSO | Usuário |
| [UC18](#uc18) | Executar Inferência TSK | Usuário |
| [UC19](#uc19) | Exportar Visualizações SVG | Usuário |
| [UC20](#uc20) | Visualizar Relatório de Diagnóstico | Usuário |
| [UC17](#uc17) | Otimizar Parâmetros com PSO | Usuário |
| [UC18](#uc18) | Executar Inferência TSK | Usuário |
| [UC19](#uc19) | Exportar Visualizações SVG | Usuário |
| [UC20](#uc20) | Visualizar Relatório de Diagnóstico | Usuário |

---

## UC01

### Gerenciar Sistemas Fuzzy

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Usuário está na tela Dashboard. A aplicação está em execução e conectada ao banco de dados. |

**Fluxo Principal — Listar sistemas**

1. **Sistema** carrega e exibe todos os sistemas fuzzy cadastrados em formato de cards no Dashboard.
2. Cada card exibe: nome, descrição, método de defuzzificação, número de variáveis e regras.
3. Usuário visualiza a lista e pode filtrar ou ordenar os sistemas.

**Fluxo Principal — Criar sistema**

1. Usuário clica no botão "Novo Sistema".
2. **Sistema** exibe formulário modal com campos: Nome (texto, obrigatório), Descrição (texto longo, opcional) e Método de defuzzificação (seleção: `centroid`, `bisector`, `mom`, `lom`, `som`; padrão: `centroid`).
3. Usuário preenche os campos e clica em "Confirmar".
4. **Sistema** valida que o Nome não está vazio e não ultrapassa 255 caracteres.
5. **Sistema** persiste o novo registro.
6. **Sistema** fecha o modal e redireciona o usuário para o Editor de Variáveis do sistema criado.
7. O novo sistema aparece listado no Dashboard.

**Fluxo Principal — Visualizar sistema**

1. Usuário clica em um card do sistema.
2. **Sistema** exibe painel de detalhes: nome, descrição, método de defuzzificação, data de criação, listagem resumida de variáveis e quantidade de regras.

**Fluxo Principal — Editar sistema**

1. Usuário clica no botão "Editar" do card.
2. **Sistema** busca os dados atuais e exibe formulário modal pré-preenchido.
3. Usuário altera os campos desejados e clica em "Salvar".
4. **Sistema** valida os dados e persiste as alterações, atualizando o timestamp.
5. **Sistema** fecha o modal e atualiza o card no Dashboard.

**Fluxo Principal — Excluir sistema**

1. Usuário clica no botão "Excluir" do card.
2. **Sistema** exibe diálogo de confirmação com aviso de exclusão em cascata.
3. Usuário confirma.
4. **Sistema** remove permanentemente o sistema e todos os dados associados.
5. **Sistema** remove o card do Dashboard.

**Fluxos Alternativos**

- **FA1 — Nome vazio ao criar/editar:** Sistema exibe "O nome do sistema é obrigatório". Retorna ao preenchimento.
- **FA2 — Nome > 255 caracteres:** Sistema exibe "Máximo 255 caracteres". Retorna ao preenchimento.
- **FA3 — Usuário cancela (qualquer operação):** Modal fechado sem persistir.
- **FA4 — Falha de comunicação:** Sistema exibe mensagem de erro. Nenhuma alteração é persistida.

**Pós-condições**

- Registro em `fuzzy_systems` criado, alterado ou removido conforme a operação.
- Dados dependentes são removidos em cascata quando um sistema é excluído.
- Dashboard reflete o estado atualizado.

---

## UC02

### Gerenciar Variáveis e Termos

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Um sistema fuzzy existe (UC01). Usuário está no Editor de Variáveis do sistema. |

**Fluxo Principal — Listar variáveis e termos**

1. **Sistema** exibe na barra lateral todas as variáveis do sistema, separadas por grupo (Antecedentes / Consequentes).
2. Ao selecionar uma variável, **Sistema** exibe seus detalhes: nome, papel, universo, resolução e lista de termos linguísticos com gráfico de funções de pertinência.

**Fluxo Principal — Adicionar variável antecedente**

1. Usuário clica em "Adicionar Variável de Entrada".
2. **Sistema** exibe formulário: Nome (texto), Universo mínimo (float), Universo máximo (float), Resolução (inteiro, padrão: 501).
3. Usuário preenche e clica em "Adicionar".
4. **Sistema** valida: nome não vazio e único; mínimo < máximo; resolução ≥ 2.
5. **Sistema** persiste a variável com `role = 'antecedent'`.
6. **Sistema** exibe a nova variável na lista.

**Fluxo Principal — Adicionar variável consequente**

1. Usuário clica em "Adicionar Variável de Saída".
2. **Sistema** verifica se já existe consequente (Mamdani permite apenas um). Se existir, bloqueia.
3. Usuário preenche os campos e clica em "Adicionar".
4. **Sistema** valida e persiste com `role = 'consequent'`.
5. **Sistema** exibe a variável em seção separada.

**Fluxo Principal — Adicionar termo linguístico**

1. Usuário clica em "Adicionar Termo" na variável desejada.
2. **Sistema** exibe formulário: Rótulo (texto), Tipo de MF (seleção: `trimf`, `trapmf`, `gaussmf`), Parâmetros (campos dinâmicos).
3. Usuário preenche e clica em "Adicionar".
4. **Sistema** valida: rótulo não vazio e único; parâmetros coerentes (`trimf: a≤b≤c`, `trapmf: a≤b≤c≤d`, `gaussmf: σ>0`).
5. **Sistema** persiste o termo e atualiza o gráfico de pertinências.

**Fluxo Principal — Remover variável ou termo**

1. Usuário clica no ícone de remoção ao lado de uma variável ou termo.
2. **Sistema** exibe diálogo de confirmação (para variável, avisa que termos associados também serão excluídos).
3. Usuário confirma.
4. **Sistema** remove o registro e atualiza a listagem.

**Fluxos Alternativos**

- **FA1 — Nome duplicado:** Sistema exibe "Já existe uma variável com este nome". Retorna ao preenchimento.
- **FA2 — Universo mínimo ≥ máximo:** Sistema exibe "Mínimo deve ser menor que máximo". Retorna ao preenchimento.
- **FA3 — Já existe consequente:** Sistema bloqueia "Este sistema já possui variável de saída".
- **FA4 — Rótulo de termo duplicado:** Sistema exibe "Já existe um termo com este rótulo". Retorna ao preenchimento.
- **FA5 — Parâmetros fora do universo:** Sistema exibe aviso "Parâmetros extrapolam o universo. Confirma?" Usuário decide.
- **FA6 — Variável referenciada em regras:** Sistema exibe "Referenciada em [N] regras. Removê-la as invalidará. Confirma?"
- **FA7 — Usuário cancela:** Nenhuma alteração persistida.

**Pós-condições**

- Registros em `fuzzy_variables` e/ou `fuzzy_terms` criados ou removidos.
- Gráfico de pertinências atualizado na interface.

---

## UC03

### Gerenciar Regras Fuzzy

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema possui variáveis antecedentes com termos e uma variável consequente com termos (UC02). Usuário está no Editor de Regras. |

**Fluxo Principal — Listar regras**

1. **Sistema** carrega todas as regras do sistema ordenadas por posição.
2. Cada regra é exibida no formato `SE [var] É [termo] E ... ENTÃO [var] É [termo] [peso]`.

**Fluxo Principal — Criar regra**

1. Usuário clica em "Nova Regra".
2. **Sistema** exibe construtor visual com seletores de variável, termo (com NOT), conector (AND/OR) e consequente.
3. Usuário monta a regra, ajusta o peso (0.0–1.0, padrão: 1.0) e clica em "Adicionar Regra".
4. **Sistema** valida: ao menos um antecedente, exatamente um consequente, regra não duplicada.
5. **Sistema** persiste a regra.
6. **Sistema** exibe a nova regra na lista.

**Fluxo Principal — Editar regra**

1. Usuário clica em "Editar" na regra desejada.
2. **Sistema** carrega a regra no construtor pré-preenchido.
3. Usuário altera e clica em "Salvar".
4. **Sistema** valida e persiste as alterações.
5. **Sistema** atualiza a exibição da regra.

**Fluxo Principal — Excluir regra**

1. Usuário clica no ícone de remoção.
2. **Sistema** exibe diálogo de confirmação.
3. Usuário confirma.
4. **Sistema** remove a regra e atualiza a lista.

**Fluxos Alternativos**

- **FA1 — Sem antecedente:** "A regra precisa de ao menos uma condição". Retorna.
- **FA2 — Sem consequente:** "A regra precisa de exatamente uma conclusão". Retorna.
- **FA3 — Regra duplicada:** "Esta regra já existe". Retorna.
- **FA4 — Usuário cancela:** Construtor fechado.
- **FA5 — Falha de comunicação:** Nenhuma alteração persistida.

**Pós-condições**

- Registro em `fuzzy_rules` criado, alterado ou removido.
- Lista de regras atualizada.

---

## UC04

### Executar Simulação

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema possui ao menos uma variável antecedente com termos, uma consequente com termos e ao menos uma regra (UC02 + UC03). Usuário está na tela Simulador. |

**Fluxo Principal**

1. **Sistema** exibe campos de input para cada variável antecedente, com universo de discurso indicado.
2. Usuário preenche os valores e clica em "Executar Simulação".
3. **Sistema** valida os pré-requisitos (variáveis, termos e regras existentes).
4. **Sistema** executa o pipeline Mamdani: fuzzificação, avaliação de regras, agregação e defuzzificação.
5. **Sistema** persiste o resultado (inputs, outputs, timestamp).
6. **Sistema** exibe o valor de saída defuzzificado de forma destacada.
7. **Sistema** disponibiliza o pipeline visual completo: fuzzificação, avaliação de regras, agregação e defuzzificação.

**Fluxos Alternativos**

- **FA1 — Pré-requisitos não atendidos:** Simulação bloqueada com mensagem específica.
- **FA2 — Input fora do universo:** Aviso em tempo real. Sistema rejeitará valor inválido.
- **FA3 — Nenhuma regra ativada:** "Nenhuma regra foi ativada para estes valores". Retorna.
- **FA4 — Falha de comunicação:** Mensagem de erro. Retorna.

**Pós-condições**

- Registro persistido em `simulations`.
- Valor de saída e pipeline visual exibidos.

---

## UC05

### Buscar Dados Climáticos

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | OpenWeather API |
| **Pré-condições** | Tela Simulador está aberta (UC04). Chave de API configurada no servidor. |

**Fluxo Principal**

1. Usuário digita o nome de uma cidade e clica em "Buscar clima".
2. **Sistema** valida o campo não vazio e consulta o serviço externo.
3. **OpenWeather API** retorna temperatura e umidade.
4. **Sistema** preenche automaticamente os campos correspondentes no Simulador.
5. **Sistema** exibe indicador visual com a cidade consultada.

**Fluxos Alternativos**

- **FA1 — Campo vazio:** "Informe o nome de uma cidade". Retorna.
- **FA2 — Cidade não encontrada:** "Cidade não encontrada". Retorna.
- **FA3 — Falha de comunicação:** "Não foi possível buscar dados. Insira manualmente." Retorna.
- **FA4 — Chave inválida:** "Erro de autenticação com o serviço climático". Retorna.
- **FA5 — Variáveis de temperatura/umidade inexistentes:** Aviso; valores disponíveis para cópia manual.

**Pós-condições**

- Campos preenchidos com dados reais.
- Nome da cidade visível na interface.

---

## UC06

### Consultar Histórico de Simulações

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Usuário está na tela Histórico. |

**Fluxo Principal — Listar simulações**

1. **Sistema** carrega listagem de simulações ordenada por data decrescente.
2. Para cada item: data/hora, cidade (se houver), resumo dos inputs e valor de saída.

**Fluxo Principal — Visualizar detalhes**

1. Usuário clica em uma simulação para expandir.
2. **Sistema** exibe painel com inputs completos, outputs, dados climáticos (se disponíveis) e método de defuzzificação.

**Fluxo Principal — Excluir simulação**

1. Usuário clica em "Remover" em uma simulação.
2. **Sistema** exibe diálogo de confirmação.
3. Usuário confirma.
4. **Sistema** remove o registro e atualiza a listagem.

**Fluxos Alternativos**

- **FA1 — Nenhuma simulação:** "Nenhuma simulação encontrada".
- **FA2 — Usuário cancela exclusão:** Simulação permanece.
- **FA3 — Falha ao carregar:** Mensagem de erro e botão "Tentar novamente".

**Pós-condições**

- Listagem exibida conforme estado atual. Exclusões são permanentes.

---

## UC07

### Processar Inferência em Lote

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema fuzzy válido selecionado (UC02 + UC03). Usuário possui arquivo com colunas numéricas mapeáveis. Usuário está no Dashboard Batch. |

**Fluxo Principal**

1. Usuário seleciona um sistema fuzzy e um arquivo, clica em "Carregar".
2. **Sistema** valida formato e tamanho do arquivo.
3. **Sistema** exibe colunas do arquivo e interface de mapeamento.
4. Usuário renomeia colunas se necessário (letras, números e underscores; sem duplicatas).
5. Usuário mapeia cada variável antecedente à coluna correspondente e confirma.
6. **Sistema** processa cada linha: aplica o mapeamento, executa inferência e coleta resultados.
7. **Sistema** persiste os resultados e exibe resumo (processados, erros, distribuição).

**Fluxos Alternativos**

- **FA1 — Arquivo inválido:** "Arquivo inválido ou corrompido". Retorna.
- **FA2 — Coluna ausente ou não numérica:** Erro com detalhes. Retorna ao mapeamento.
- **FA3 — Linhas com valores inválidos:** Registra como erro e prossegue.
- **FA4 — Nenhuma linha válida:** "Nenhuma linha gerou saída válida". Retorna.
- **FA5 — Arquivo excede limite:** "Arquivo excede o tamanho máximo". Retorna.
- **FA6 — Nome de coluna inválido:** "Use apenas letras, números e underscores". Retorna.
- **FA7 — Sistema incompleto:** Bloqueia com mensagem.

**Pós-condições**

- Resultados persistidos vinculados ao sistema e arquivo fonte.
- Dashboard Batch exibe distribuição e resultados.
- Arquivo original não é modificado.

---

## UC08

### Comparar Simulações

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Ao menos duas simulações existem no histórico (UC04 + UC06). Usuário está na tela de Comparação. |

**Fluxo Principal**

1. Usuário seleciona duas ou mais simulações no histórico e clica em "Comparar".
2. **Sistema** exibe as simulações lado a lado em uma tabela comparativa.
3. Para cada simulação: inputs, output defuzzificado, classificação, regras ativadas e timestamp.
4. **Sistema** destaca diferenças entre os outputs e input com maior sensibilidade.
5. Usuário pode selecionar diferentes simulações para refazer a comparação.

**Fluxos Alternativos**

- **FA1 — Menos de duas simulações selecionadas:** Sistema exibe "Selecione ao menos duas simulações para comparar". Retorna.
- **FA2 — Simulações de sistemas diferentes:** Sistema exibe "Apenas simulações do mesmo sistema podem ser comparadas". Retorna.

**Pós-condições**

- Tabela comparativa exibida com diferenças destacadas.

---

## UC09

### Exportar Relatório de Simulação

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Ao menos uma simulação foi executada (UC04). Usuário está no Histórico ou na tela de resultado da simulação. |

**Fluxo Principal**

1. Usuário clica em "Exportar Relatório" em uma simulação ou no resultado da simulação recém-executada.
2. **Sistema** pergunta o formato desejado: PDF ou CSV.
3. Usuário seleciona o formato.
4. **Sistema** gera o relatório contendo: data e hora, sistema utilizado, inputs, output defuzzificado, método de defuzzificação, dados climáticos (se houver) e pipeline completo (fuzzificação, regras ativadas, agregação).
5. **Sistema** disponibiliza o arquivo para download.

**Fluxos Alternativos**

- **FA1 — Falha na geração:** Sistema exibe "Não foi possível gerar o relatório". Retorna.

**Pós-condições**

- Arquivo no formato escolhido é baixado pelo usuário.
- Nenhuma alteração no banco de dados.

---

## UC10

### Duplicar Sistema Fuzzy

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Ao menos um sistema fuzzy existe (UC01). Usuário está no Dashboard. |

**Fluxo Principal**

1. Usuário clica em "Duplicar" no card do sistema desejado.
2. **Sistema** exibe formulário para definir o nome do novo sistema (pré-preenchido com "[Nome Original] (cópia)").
3. Usuário ajusta o nome se desejar e confirma.
4. **Sistema** clona o sistema original com todos os dados: variáveis, termos, regras e configurações.
5. **Sistema** exibe o novo sistema no Dashboard com status "Rascunho".

**Fluxos Alternativos**

- **FA1 — Nome duplicado:** Sistema exibe "Já existe um sistema com este nome". Retorna.
- **FA2 — Falha de comunicação:** Mensagem de erro. Nada é criado.

**Pós-condições**

- Novo registro em `fuzzy_systems` com cópia completa de variáveis, termos e regras.
- Dashboard exibe o sistema duplicado.

---

## UC11

### Exportar e Importar Sistema

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Para exportar: ao menos um sistema existe (UC01). Para importar: usuário possui um arquivo JSON de sistema. Usuário está no Dashboard. |

**Fluxo Principal — Exportar sistema**

1. Usuário clica em "Exportar" no card do sistema.
2. **Sistema** gera um arquivo JSON contendo: nome, descrição, método de defuzzificação, variáveis (com universos e termos completos) e regras.
3. **Sistema** disponibiliza o arquivo para download.

**Fluxo Principal — Importar sistema**

1. Usuário clica em "Importar Sistema" no Dashboard.
2. **Sistema** exibe campo para selecionar arquivo JSON.
3. Usuário seleciona o arquivo e clica em "Importar".
4. **Sistema** valida a estrutura do JSON (campos obrigatórios, consistência das variáveis e regras).
5. **Sistema** persiste o novo sistema com todos os dados e exibe no Dashboard.

**Fluxos Alternativos**

- **FA1 — JSON inválido (importação):** Sistema exibe "Arquivo JSON inválido ou mal formatado". Retorna.
- **FA2 — Estrutura incompleta:** Sistema exibe "O JSON não contém todos os campos obrigatórios". Retorna.
- **FA3 — Nome duplicado (importação):** Sistema exibe "Já existe um sistema com este nome". Retorna.

**Pós-condições**

- Exportação: arquivo JSON baixado. Banco inalterado.
- Importação: novo sistema criado no banco com todos os dados.

---

## UC12

### Salvar Cenário de Simulação

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Usuário está na tela Simulador com valores preenchidos nos inputs (UC04). |

**Fluxo Principal**

1. Usuário clica em "Salvar Cenário" no Simulador.
2. **Sistema** exibe formulário com campo de nome (pré-preenchido com "Cenário [data]").
3. Usuário nomeia o cenário e confirma.
4. **Sistema** persiste os valores de input atuais associados ao sistema e ao nome informado.
5. **Sistema** exibe o cenário na lista "Cenários Salvos" ao lado dos inputs.

**Fluxo Principal — Carregar cenário salvo**

1. Usuário clica em um cenário na lista "Cenários Salvos".
2. **Sistema** preenche automaticamente todos os campos de input com os valores salvos.
3. Usuário pode executar a simulação (UC04) ou ajustar os valores.

**Fluxo Principal — Excluir cenário**

1. Usuário clica no ícone de remoção ao lado de um cenário salvo.
2. **Sistema** exibe diálogo de confirmação.
3. Usuário confirma.
4. **Sistema** remove o cenário da lista.

**Fluxos Alternativos**

- **FA1 — Nome duplicado:** "Já existe um cenário com este nome neste sistema". Retorna.
- **FA2 — Nenhum input preenchido:** "Preencha ao menos um input antes de salvar". Retorna.

**Pós-condições**

- Cenário persistido e disponível para carregamento futuro.
- Cenários removidos são permanentemente excluídos.

---

## UC13

### Executar Varredura de Entrada

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema fuzzy válido (UC02 + UC03). Usuário está na tela Simulador ou em tela de análise. |

**Fluxo Principal**

1. Usuário seleciona uma variável de entrada para varrer e define o intervalo (início, fim, passo).
2. Usuário define valores fixos para as demais variáveis.
3. Usuário clica em "Executar Varredura".
4. **Sistema** executa a simulação para cada ponto do intervalo, mantendo as demais entradas fixas.
5. **Sistema** exibe gráfico 2D (entrada varrida × saída) com curva contínua.
6. Usuário pode alternar a variável varrida ou alterar o passo para refinar.

**Fluxos Alternativos**

- **FA1 — Intervalo inválido (início ≥ fim):** Sistema exibe "Início deve ser menor que fim". Retorna.
- **FA2 — Passo inválido (≤ 0):** "Passo deve ser maior que zero". Retorna.
- **FA3 — Sistema incompleto:** Bloqueia com mensagem.

**Pós-condições**

- Gráfico 2D exibido com a curva de sensibilidade da saída em função da entrada varrida.
- Nenhum registro persistido (análise em memória).

---

## UC14

### Visualizar Matriz de Regras Ativadas

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Simulação executada ou varredura concluída (UC04 ou UC13). Dados de ativação de regras disponíveis. |

**Fluxo Principal**

1. **Sistema** exibe uma matriz (grid) onde linhas representam combinações de antecedentes e colunas representam regras.
2. Cada célula é colorida conforme o grau de ativação α da regra (mais escuro = maior ativação).
3. Regras com α = 0 aparecem em cinza.
4. Usuário passa o mouse sobre uma célula para ver o valor exato de α.
5. Usuário pode filtrar a matriz para mostrar apenas regras com α > 0.

**Fluxos Alternativos**

- **FA1 — Nenhuma regra ativada:** Matriz exibe todas as células em cinza com mensagem "Nenhuma regra foi ativada".

**Pós-condições**

- Matriz visual interativa exibida. Nenhuma persistência adicional.

---

## UC15

### Visualizar Superfície de Controle

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema fuzzy com ao menos duas variáveis antecedentes e uma consequente (UC02). Usuário está na tela de análise. |

**Fluxo Principal**

1. Usuário seleciona duas variáveis de entrada para os eixos X e Y.
2. Usuário define o intervalo e a resolução para cada eixo.
3. Usuário define valores fixos para as demais variáveis (se houver).
4. Usuário clica em "Gerar Superfície".
5. **Sistema** calcula a saída para cada combinação (x, y) no grid definido.
6. **Sistema** renderiza um gráfico 2D de mapa de calor (coordenada x, y = inputs, cor = output).
7. Usuário pode alternar os pares de entrada ou ajustar a resolução.

**Fluxos Alternativos**

- **FA1 — Menos de duas entradas:** "O sistema precisa de ao menos duas variáveis de entrada". Retorna.
- **FA2 — Resolução muito alta (> 100 pts por eixo):** Aviso "Isso pode impactar o desempenho. Confirma?" Usuário decide.

**Pós-condições**

- Mapa de calor 2D exibido. Nenhum registro persistido.

---

## UC16

### Gerenciar Histórico de Alterações

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Um sistema fuzzy existe (UC01). Usuário está no Editor de Variáveis, Regras ou Dashboard. |

**Fluxo Principal — Visualizar timeline de alterações**

1. Usuário clica em "Histórico de Alterações" no painel do sistema.
2. **Sistema** exibe timeline cronológica reversa de todas as ações realizadas no sistema: criação/edição/exclusão de variáveis, termos, regras e metadados.
3. Cada evento exibe: data/hora, tipo de ação (criar, editar, excluir), entidade afetada e descrição da mudança.
4. Usuário clica em um evento para ver o diff (antes/depois) dos dados alterados.

**Fluxo Principal — Desfazer alteração**

1. Usuário localiza um evento na timeline e clica em "Desfazer".
2. **Sistema** exibe confirmação: "Desfazer esta ação? Isso reverterá o estado anterior."
3. Usuário confirma.
4. **Sistema** restaura o estado anterior à ação, registrando o desfazer como novo evento na timeline.
5. **Sistema** atualiza a interface refletindo o estado restaurado.

**Fluxo Principal — Refazer alteração**

1. Usuário clica em "Refazer" após ter desfeito uma ação.
2. **Sistema** reaplica a ação desfeita.
3. **Sistema** registra o refazer como novo evento.
4. **Sistema** atualiza a interface.

**Fluxos Alternativos**

- **FA1 — Timeline vazia:** "Nenhuma alteração registrada para este sistema".
- **FA2 — Ação não pode ser desfeita (ex.: exclusão em cascata complexa):** Sistema exibe "Esta ação não pode ser desfeita automaticamente".
- **FA3 — Conflito ao desfazer (estado atual difere do esperado):** Sistema exibe "Não é possível desfazer: o estado atual do sistema difere do esperado. Verifique as alterações manuais."

**Pós-condições**

- Timeline exibida com todas as ações registradas.
- Ao desfazer/refazer: estado do sistema restaurado e novo evento registrado na timeline.

---

## UC17

### Otimizar Parâmetros com PSO

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema fuzzy com variáveis, termos e regras configurados (UC02 + UC03). Usuário está na tela de Otimização. |

**Fluxo Principal**

1. Usuário seleciona o que deseja otimizar: parâmetros das funções de pertinência, pesos das regras ou ambos.
2. Usuário define se possui dados de referência (pares entrada-saída esperada) para a função objetivo.
3. Usuário configura parâmetros do PSO: tamanho da população, número máximo de iterações, limites de busca para cada parâmetro, peso da inércia, coeficientes cognitivo e social, tolerância e paciência (early stopping).
4. Usuário clica em "Iniciar Otimização".
5. **Sistema** executa o PSO em background (`spawn_blocking`), avaliando a função objetivo (erro quadrático médio ou customizada) a cada iteração.
6. **Sistema** exibe gráfico de convergência (melhor fitness por iteração) em tempo real.
7. Ao final, **Sistema** exibe os parâmetros ótimos encontrados e o fitness final.
8. Usuário pode aplicar os parâmetros otimizados ao sistema atual ou salvar como nova versão.

**Fluxos Alternativos**

- **FA1 — Nenhum dado de referência:** Usuário pode definir função objetivo manual (ex: minimizar/maximizar saída para determinados inputs).
- **FA2 — Limites inválidos:** "Limite mínimo deve ser menor que o máximo para cada parâmetro". Retorna.
- **FA3 — Sem parâmetros selecionados:** "Selecione ao menos um parâmetro para otimizar". Retorna.
- **FA4 — Otimização divergente:** "A otimização não convergiu dentro da tolerância. Ajuste os parâmetros do PSO e tente novamente."
- **FA5 — Usuário cancela:** Otimização interrompida. Resultados parciais descartados.

**Pós-condições**

- Parâmetros otimizados exibidos e disponíveis para aplicação.
- Gráfico de convergência disponível para download.
- Nenhuma alteração no banco até o usuário confirmar a aplicação.

---

## UC18

### Executar Inferência TSK

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema fuzzy com variáveis antecedentes e ao menos uma regra. Usuário está na tela Simulador ou Análise. |

**Fluxo Principal**

1. Usuário seleciona o motor de inferência: "Mamdani" ou "TSK (Takagi-Sugeno-Kang)".
2. Ao selecionar TSK, **Sistema** exibe campos de coeficientes para cada consequente de regra: `y = a₀ + a₁·x₁ + a₂·x₂ + ...` onde `a₀` é o bias e `a₁, a₂...` são coeficientes para cada entrada, na ordem alfabética das variáveis.
3. Usuário preenche os coeficientes de cada regra e clica em "Executar".
4. **Sistema** executa a inferência TSK: fuzzificação → grau de ativação por regra → média ponderada dos consequentes → saída crisp.
5. **Sistema** exibe o valor de saída, o cálculo detalhado (ativação de cada regra × seu consequente) e o peso final.
6. Usuário pode alternar entre Mamdani e TSK para comparar os resultados.

**Fluxos Alternativos**

- **FA1 — Sistema configurado para Mamdani sem consequente fuzzy:** "O sistema precisa de uma variável consequente com termos para TSK ou remova os termos para usar apenas coeficientes."
- **FA2 — Coeficientes inconsistentes:** "Número de coeficientes deve ser 1 + N (bias + uma para cada entrada)". Retorna.
- **FA3 — Nenhuma regra ativada:** Mensagem similar ao UC04-FA3.

**Pós-condições**

- Resultado TSK exibido com detalhamento dos cálculos.
- Comparação Mamdani × TSK disponível se ambos forem executados.

---

## UC19

### Exportar Visualizações SVG

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Sistema fuzzy com variáveis e termos configurados (UC02). Usuário está no Editor de Variáveis ou Simulador. |

**Fluxo Principal**

1. Usuário clica em "Exportar SVG" no painel de uma variável ou no resultado da simulação.
2. **Sistema** pergunta o que exportar: "Funções de pertinência de uma variável", "Todas as variáveis", "Conjunto agregado" ou "Pipeline completo da simulação".
3. Usuário seleciona a opção desejada.
4. **Sistema** gera o gráfico SVG com tema Catppuccin Mocha (dark) ou tema claro, conforme preferência.
5. **Sistema** exibe prévia do SVG na tela e disponibiliza botão de download.
6. Usuário pode baixar o arquivo SVG ou copiar o código SVG para a área de transferência.

**Fluxos Alternativos**

- **FA1 — Nenhuma variável configurada:** "Não há dados para exportar."
- **FA2 — Falha na geração:** "Não foi possível gerar o SVG". Retorna.

**Pós-condições**

- Arquivo SVG baixado ou código copiado. Nenhuma alteração no banco.

---

## UC20

### Visualizar Relatório de Diagnóstico

| Campo | Descrição |
|---|---|
| **Ator Primário** | Usuário |
| **Atores Secundários** | — |
| **Pré-condições** | Simulação executada com sucesso (UC04). Usuário está no resultado da simulação ou no Histórico. |

**Fluxo Principal**

1. Usuário clica em "Diagnóstico" ou "Explain" no resultado de uma simulação.
2. **Sistema** gera relatório detalhado do pipeline de inferência contendo:
   - Tabela de fuzzificação: grau de pertinência de cada input em cada termo.
   - Tabela de regras: grau de ativação (α) de cada regra, com destaque para regras não ativadas (α = 0).
   - Tabela de implicação: contribuição de cada regra para o conjunto agregado.
   - Ponto de defuzzificação: valor crisp final e método utilizado.
   - Tabela COG (Center of Gravity): discretização do centroide para verificação manual.
3. **Sistema** exibe o relatório em formato de painéis colapsáveis na interface.
4. Usuário pode expandir cada seção para ver detalhes ou exportar o diagnóstico como JSON/CSV.

**Fluxos Alternativos**

- **FA1 — Simulação sem dados de diagnóstico:** "Esta simulação não possui dados detalhados. Execute novamente com diagnóstico ativado."
- **FA2 — Nenhuma regra ativada:** Relatório exibe "Nenhuma regra foi ativada" com a tabela de fuzzificação para depuração.

**Pós-condições**

- Relatório de diagnóstico exibido e disponível para exportação.
- Nenhuma alteração no banco.
