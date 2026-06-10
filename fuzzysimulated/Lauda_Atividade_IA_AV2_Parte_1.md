# ESCOLA DE NEGÓCIOS, TECNOLOGIA E INOVAÇÃO DO CESUPA

## TRABALHO DE PESQUISA E DESENVOLVIMENTO
## PARTE 1: SISTEMAS DE CONTROLE FUZZY
### INTELIGÊNCIA ARTIFICIAL E COMPUTACIONAL (0700M8)

**Prof. Daniel Leal Souza** – Semestre: 01/2026

---

| Campo | Descrição |
|---|---|
| **Disciplina** | Inteligência Artificial e Computacional: Ciência da Computação. |
| **Tema** | Pesquisa, modelagem, implementação, validação, documentação técnica e apresentação de uma solução baseada em Sistemas de Controle Fuzzy. |
| **Conteúdos mobilizados** | Lógica Fuzzy; conjuntos fuzzy; variáveis linguísticas; funções de pertinência; fuzzificação; operadores fuzzy; base de regras; inferência; defuzzificação; sistemas Mamdani; sistemas TSK; validação por simulação; documentação técnica; uso responsável de IA. |
| **Formato** | Trabalho em equipe. Equipes de até **4 integrantes** seguem os requisitos regulares. Equipes com **5 integrantes**, quando autorizadas, deverão cumprir uma tarefa de ampliação obrigatória, descrita nesta lauda. |
| **Pontuação regular** | **2,0 pts**, conforme o Plano de Ensino. |
| **Pontuação extra** | Até **0,5 pts** opcionais, mediante entrega de uma extensão Neuro-Fuzzy ou de otimização automática de parâmetros fuzzy com AG ou PSO. |
| **Entrega e apresentação** | CC5MA: 09/06/2026. CC5NA: 11/06/2026. Submissão final no Google Classroom até 23h59 da data da respectiva turma, salvo orientação posterior do professor. |
| **Entregáveis** | Documento em PDF + link do repositório GitHub obrigatório + código-fonte + manual de execução + base de regras + funções de pertinência + resultados experimentais + slides + declaração de uso de IA. |

---

## 1. Finalidade e perfil da atividade

Esta atividade avalia a capacidade da equipe de **pesquisar, modelar, implementar, validar, documentar e defender tecnicamente** uma solução baseada em **sistemas de controle fuzzy**. O trabalho deverá partir de um problema prático, científico, social, operacional, industrial, agrícola, educacional, comercial ou de apoio à decisão que envolva imprecisão, gradação de decisão, incerteza qualitativa, julgamento linguístico ou controle aproximado.

O trabalho foi dimensionado para ser executável, com qualidade, em pouco mais de um mês. A expectativa não é a construção de um produto comercial completo, mas de um protótipo tecnicamente consistente, reprodutível, bem documentado e defendido com domínio conceitual. Não será suficiente apenas executar um sistema ou uma biblioteca pronta: a equipe deverá explicar as variáveis, os universos de discurso, as funções de pertinência, a base de regras, os operadores utilizados, o mecanismo de inferência, o método de defuzzificação ou, no caso TSK, o cálculo ponderado da saída.

Durante a apresentação e a entrega do trabalho, será exigido de cada aluno o domínio dos conceitos apresentados no trabalho.

Todo trabalho deverá possuir um **repositório no GitHub**, criado e mantido pela equipe, contendo o código-fonte, o documento ou instruções de execução, evidências mínimas de organização e o histórico de desenvolvimento do projeto. O link do repositório deverá ser informado no documento principal e no Google Classroom. Repositórios privados serão aceitos apenas se o professor tiver acesso antes do prazo final de entrega.

---

## 2. Resumo do trabalho

Cada equipe deverá escolher exatamente uma modalidade principal. As modalidades preservam a proposta original da atividade, mas foram reorganizadas para reduzir redundâncias e facilitar a leitura.

| Opção | Foco | Resultado esperado |
|---|---|---|
| **A** | Pesquisa e implementação de sistemas fuzzy baseados em artigos científicos | Estudo de literatura, escolha de artigo(s), implementação de uma reprodução mínima ou adaptação fundamentada e escrita em formato de artigo técnico/científico. |
| **B** | Aplicação ou produto de mercado | Levantamento de problema real, requisitos, implementação de protótipo funcional, documentação de produto, testes e apresentação como solução aplicável. |
| **C** | Takagi-Sugeno-Kang (TSK) | Execução da Opção A ou B usando TSK. |

Nas opções A e B, o modelo padrão será **Mamdani**. Na opção C, dado que as regras exigem o uso de Takagi-Sugeno-Kang como motor de inferência, as regras deverão possuir consequentes constantes, lineares ou afins, por exemplo:

> Se x₁ é Alto e x₂ é Baixo, então y = a₀ + a₁x₁ + a₂x₂.

---

## 3. Requisitos técnicos obrigatórios

A tabela abaixo reúne os requisitos mínimos. A equipe poderá ir além desses itens, desde que mantenha coerência e viabilidade.

| Dimensão | Exigência mínima |
|---|---|
| **Problema** | O problema deve ser realista, claramente delimitado e adequado à lógica fuzzy. Exemplos excessivamente simples, meras trocas de nomes em tutoriais ou repetições de sala sem expansão serão penalizados. |
| **Variáveis** | Pelo menos 2 entradas e 1 saída. Soluções com 3 ou mais entradas serão valorizadas quando bem justificadas. |
| **Termos linguísticos** | Pelo menos 3 termos linguísticos na principal variável de entrada. No modelo Mamdani, pelo menos 3 termos linguísticos na saída. |
| **Funções de pertinência** | Devem ser apresentadas por gráficos, fórmulas, parâmetros ou tabelas. Podem ser triangulares, trapezoidais, gaussianas, sigmoides, bell-shaped, etc. |
| **Base de regras** | Pelo menos 12 regras efetivamente utilizadas, salvo justificativa técnica forte. As regras devem refletir literatura, conhecimento do domínio, dados, consulta a especialista ou análise própria. |
| **Inferência e saída** | Em Mamdani, explicar operadores, implicação, agregação e defuzzificação. Em TSK, explicar pesos das regras, consequentes e média ponderada ou procedimento equivalente. |
| **Testes** | Pelo menos 6 cenários de teste, incluindo casos baixos, médios, altos, fronteiriços, conflitantes ou críticos. |
| **Validação** | A equipe deverá analisar o comportamento do sistema, não apenas listar saídas. Esperam-se tabelas, gráficos, superfícies de controle, mapas de decisão, curvas de sensibilidade ou comparação com referência. |
| **Reprodução e consistência** | O código deve executar, o manual deve permitir reprodução e os resultados do relatório devem corresponder ao que foi implementado. O link do GitHub deve permitir acesso aos arquivos necessários para execução e avaliação. |
| **GitHub obrigatório** | Cada equipe deverá manter um repositório GitHub próprio, com acesso concedido ao professor. O repositório deverá conter README, código-fonte, instruções de execução, dependências, organização dos arquivos e, quando possível, histórico de commits compatível com o desenvolvimento do projeto. |

---

## 4. O que caracteriza uma boa solução fuzzy?

Uma solução fuzzy bem elaborada apresenta **coerência entre problema, variáveis, universos de discurso, funções de pertinência, base de regras, mecanismo de inferência e resultados**. A equipe deverá evitar sistemas artificiais, genéricos ou óbvios demais. Será valorizado o trabalho que demonstre que a lógica fuzzy foi escolhida por ser adequada ao problema, e não apenas porque era exigida na atividade.

| Aspecto esperado | Evidência de qualidade |
|---|---|
| **Delimitação do problema** | A decisão, classificação, recomendação ou ação de controle é compreensível e tem utilidade no domínio escolhido. |
| **Entradas e saída** | Cada variável tem papel explícito; variáveis irrelevantes ou artificiais são evitadas. |
| **Universos de discurso** | Intervalos, unidades, limites e hipóteses são declarados e coerentes com o domínio. |
| **Pertinência** | As funções não são estreitas a ponto de eliminar o caráter fuzzy, nem largas a ponto de tornar o sistema indiferente. |
| **Regras** | A base cobre casos típicos, intermediários, críticos e situações de conflito; as regras possuem justificativa. |
| **Análise crítica** | A equipe discute quando o sistema funciona bem, quando falha, quais parâmetros são sensíveis e que melhorias são possíveis. |
| **Organização no GitHub** | O repositório permite localizar código, documentação, resultados, instruções de execução e artefatos relevantes sem depender de explicações externas. |

### 4.1 Exemplos de problemas aceitáveis

Os exemplos abaixo são sugestões, não uma lista fechada: climatização inteligente; priorização de atendimento em saúde; risco de crédito, fraude ou inadimplência; controle de velocidade de robô ou veículo autônomo; irrigação agrícola com múltiplas variáveis e expansão substancial; avaliação de risco em projetos; controle de estoque e reposição; manutenção preditiva; avaliação de desempenho acadêmico; recomendação logística; precificação dinâmica; apoio à decisão em ambientes com múltiplos critérios.

> **Atenção:** Exemplos muito parecidos com gorjetas, irrigação simples, ventilador básico, risco de projeto com apenas duas variáveis ou qualquer exemplo apresentado em sala de aula deverão apresentar **expansão clara significativa**. A equipe deve assegurar novas variáveis, nova validação, novas configurações e nova aplicação que as diferenciem substancialmente dos exemplos apresentados em aula. Caso contrário, a nota poderá ser limitada e/ou sofrer penalidades.

---

## 5. Modalidades do trabalho

### 5.1 Opção A: Pesquisa em artigos científicos

Nesta modalidade, a equipe deverá conduzir levantamento bibliográfico sobre uso, aplicação ou melhoria de sistemas de controle fuzzy. O resultado deverá ser apresentado em formato de artigo técnico/científico, seguindo padrão reconhecível de conferência, periódico ou relatório científico estruturado.

A equipe deverá pesquisar artigos científicos relacionados ao problema escolhido. O levantamento deverá incluir os seguintes pontos:

1. Bases ou mecanismos utilizados na busca, tais como IEEE Xplore, ACM Digital Library, ScienceDirect, SpringerLink, Scopus, Web of Science, Google Scholar ou repositórios equivalentes;
2. Palavras-chave utilizadas;
3. Critérios de inclusão e exclusão dos artigos;
4. Justificativa para a escolha do artigo principal ou dos artigos principais utilizados como referência.

| Item | O que entregar |
|---|---|
| **Levantamento** | Bases ou mecanismos de busca utilizados; palavras-chave; critérios de inclusão/exclusão; tabela com pelo menos 5 trabalhos relacionados. |
| **A1: Reprodução** | Escolher um artigo principal, explicar o problema, identificar entradas/saídas/regras/pertinências, implementar uma reprodução mínima viável e comparar resultados. |
| **A2: Adaptação** | Usar um ou mais artigos como base para construir uma solução própria, declarando o que foi reaproveitado, adaptado, expandido ou modificado. |
| **Quando dados faltarem** | Declarar limitações de reprodução, justificar adaptações e apresentar uma reprodução mínima viável. |
| **Estrutura mínima** | Título, autores, resumo, introdução, fundamentação teórica, trabalhos relacionados, metodologia, modelagem fuzzy, implementação, experimentos, discussão, conclusão e referências. |

### 5.2 Opção B: Aplicação ou produto baseado em controle fuzzy

Nesta modalidade, a equipe deverá construir uma solução apresentada como produto, protótipo ou ferramenta de apoio à decisão. O produto pode ser notebook interativo, aplicação web, dashboard, API, aplicação desktop, aplicação mobile, simulador, sistema embarcado, interface em linha de comando bem documentada ou solução equivalente.

| Item | O que entregar |
|---|---|
| **Problema e público-alvo** | Descrever o problema, quem usaria a solução, qual decisão o sistema apoia e por que a lógica fuzzy é adequada. |
| **Requisitos** | Listar requisitos funcionais e não funcionais, entradas, saídas, fluxo de uso, limitações e riscos de interpretação incorreta. |
| **Protótipo** | Entregar uma versão funcional e demonstrável. Não é necessário produto comercial completo, mas a execução deve ser clara. |
| **Documentação** | Visão geral, arquitetura, modelo fuzzy, instalação, execução, manual de uso, exemplos de entrada/saída, testes, limitações, melhorias futuras e link do repositório GitHub. |
| **Apresentação** | Defender a solução como produto: problema, usuário-alvo, diferencial, demonstração funcional, riscos, limitações e próximos passos. |

### 5.3 Opção C: Sistema Fuzzy Takagi–Sugeno–Kang (TSK)

A Opção C consiste em executar a Opção A ou B usando TSK. A equipe deverá declarar **C-A** quando seguir a trilha de artigo científico, ou **C-B** quando seguir a trilha de produto. Diferentemente de Mamdani, as regras TSK não têm consequentes fuzzy como "saída baixa", "saída média" ou "saída alta"; elas têm consequentes constantes, lineares ou afins.

| Exigência TSK | Descrição |
|---|---|
| **Antecedentes** | Funções de pertinência nas entradas devem ser descritas como no modelo fuzzy usual. |
| **Consequentes** | Cada regra deve produzir uma função constante, linear ou afim das entradas. |
| **Peso da regra** | Explicar como o grau de ativação de cada regra foi calculado. |
| **Saída final** | Explicar o cálculo por média ponderada ou método equivalente. |
| **Comparação recomendada** | Discutir, conceitual ou experimentalmente, diferenças entre TSK e Mamdani: interpretabilidade, continuidade da saída, facilidade de ajuste e custo computacional. |

---

## 6. Equipes, participação e tarefa extra para equipes com 5 integrantes

O formato regular é de até **4 integrantes**. Em caráter excepcional, uma equipe poderá ter **5 integrantes**; nesse caso, a ampliação de equipe deverá ser acompanhada de ampliação objetiva do trabalho. A regra busca preservar proporcionalidade entre quantidade de alunos e volume de entrega.

Equipes com 5 integrantes deverão cumprir, além dos requisitos mínimos, uma **trilha obrigatória de ampliação** dentre as opções abaixo. Essa tarefa não é a mesma coisa que a pontuação extra opcional; ela é condição para que a equipe ampliada seja avaliada sem desvantagem por divisão excessiva de tarefas.

| Trilha de ampliação | Tarefa adicional obrigatória para equipe com 5 integrantes |
|---|---|
| **Ampliação técnica do modelo** | Usar no mínimo 3 entradas, pelo menos 18 regras e no mínimo 12 cenários de teste, incluindo análise de sensibilidade de pelo menos uma função de pertinência. |
| **Comparação de modelos** | Implementar ou simular uma comparação entre duas versões: por exemplo, Mamdani versus TSK, ou duas bases de regras/funções de pertinência, discutindo diferenças de saída. |
| **Validação ampliada** | Realizar validação com dados reais, sintéticos controlados ou consulta estruturada a especialista/usuário, apresentando tabela de comparação entre expectativa e saída do sistema. |
| **Produto ampliado** | Na Opção B, entregar interface mais completa, registro de logs/experimentos, exportação de resultados ou módulo de configuração de parâmetros pelo usuário. |

> Todos os integrantes devem compreender o projeto. Durante a apresentação, o professor poderá direcionar perguntas a qualquer aluno. A ausência de domínio conceitual por parte de um integrante poderá afetar a nota individual de apresentação, mesmo que o produto funcione.

---

## 7. Pontuação extra opcional: até 0,5 pts

A pontuação extra é opcional, não substitui os requisitos regulares e somente será considerada se o trabalho principal estiver funcional e minimamente completo. A equipe poderá escolher uma das opções abaixo.

| Opção extra | O que deve ser feito |
|---|---|
| **1. Neuro-Fuzzy** | Pesquisar e implementar uma extensão Neuro-Fuzzy, demonstrando como redes neurais, aprendizado supervisionado ou ajuste de parâmetros podem se conectar ao sistema fuzzy. A equipe deverá explicar a arquitetura, o fluxo de dados, o que foi aprendido/ajustado e quais limitações existem. |
| **2. Otimização de Hiperparâmetros com AG ou PSO** | Fazer conexão com Computação Evolutiva implementando otimização automática de parâmetros fuzzy com AG ou PSO. Podem ser ajustados limites das funções de pertinência, pesos de regras, consequentes TSK ou parâmetros equivalentes. A equipe deverá definir função objetivo, representação da solução, critérios de parada e comparação antes/depois. |
| **3. Implementação de artigos com elevado fator de impacto** | Utilize bases de dados renomadas como Web of Science (JCR), Scopus ou Google Acadêmico, focando em periódicos com alto Journal Impact Factor (JIF). A busca deve ser refinada por área do conhecimento no Journal Citation Reports (JCR) para comparar revistas. Para a plataforma Qualis, pesquisar por artigos classificados como A1, A2, A3 ou A4 nas áreas de Computação ou Engenharias IV (Engenharia de Computação). |

A pontuação extra será atribuída conforme qualidade técnica, integração com o sistema principal, clareza da explicação e evidências experimentais. Entrega superficial, apenas conceitual ou desconectada do projeto não receberá pontuação extra.

---

## 8. Entregáveis obrigatórios e repositório GitHub

A submissão final deverá conter os arquivos solicitados e o link do repositório GitHub do projeto. O repositório é **obrigatório** para todas as equipes e faz parte da avaliação. Recomenda-se nomear arquivos, pastas e commits de forma clara, incluindo turma, equipe, opção escolhida e finalidade de cada artefato.

### 8.1 Regras mínimas para o GitHub

| Item | Exigência mínima |
|---|---|
| **Acesso** | O repositório deve ser público ou privado com acesso concedido ao professor até o prazo final. Link quebrado, repositório inacessível ou permissão não concedida será tratado como ausência de GitHub. |
| **README** | Deve conter título do projeto, turma, integrantes, modalidade escolhida, resumo da solução, tecnologias usadas, instruções de instalação, execução, reprodução dos testes e descrição dos principais arquivos. |
| **Organização** | O repositório deve separar, quando aplicável, código-fonte, notebooks, dados ou amostras, documentação, resultados, imagens, slides e relatório. Arquivos soltos e sem identificação serão penalizados. |
| **Reprodutibilidade** | Devem estar presentes dependências, versão de bibliotecas, comandos de execução ou notebook executável. Quando dados completos não puderem ser publicados, a equipe deverá fornecer amostra, dados sintéticos ou instruções claras de obtenção. |
| **Coerência** | O conteúdo do GitHub deve corresponder ao relatório, à apresentação e à demonstração. Código que não corresponde ao sistema apresentado será penalizado. |

### 8.2 Lista de entregáveis

| Entregável | Conteúdo mínimo |
|---|---|
| **Documento principal** | Relatório, artigo ou documentação de produto em PDF, contendo problema, fundamentação, metodologia, modelagem fuzzy, implementação, resultados, discussão, referências e link do GitHub. |
| **Repositório GitHub** | Link obrigatório do repositório contendo README, código-fonte, instruções de execução, dependências, artefatos relevantes e organização compatível com o trabalho entregue. |
| **Código-fonte** | Arquivos organizados, executáveis e compatíveis com o que foi descrito no documento, preferencialmente mantidos no GitHub. O código deve conter comentários suficientes para compreensão. |
| **Manual de execução** | Instruções para instalar dependências, configurar ambiente, executar o sistema e reproduzir os principais resultados. |
| **Base de regras** | Tabela explícita com todas as regras. Em TSK, incluir os consequentes e a forma de cálculo da saída. |
| **Funções de pertinência** | Gráficos, fórmulas, parâmetros ou tabelas das funções de pertinência, com universos de discurso e unidades. |
| **Cenários de teste** | Tabela com entradas, saída produzida, interpretação e comentário sobre coerência do resultado. |
| **Evidências de execução** | Prints, logs, notebooks executados, capturas de tela, gráficos, superfícies de controle, vídeos curtos opcionais ou outputs reprodutíveis. |
| **Slides** | Arquivo em PDF ou link acessível com a apresentação. Slides devem apoiar a defesa técnica, não substituir a demonstração. |
| **Declaração de uso de IA** | Seção ou documento específico indicando ferramenta, finalidade, prompts resumidos, partes aproveitadas e revisão humana. |
| **Referências** | Artigos, livros, documentação técnica, bases de dados, tutoriais e ferramentas utilizadas, citados de forma consistente. |

---

## 9. Declaração obrigatória de uso de IA

O uso de IA generativa, IA agêntica, assistentes de programação, ferramentas de autocompletar código, geradores de texto, ferramentas de busca assistida ou agentes de desenvolvimento é **permitido**. A equipe, entretanto, deverá declarar o uso com transparência e demonstrar revisão humana.

| Ferramenta | Finalidade | Prompt/comando resumido | Revisão crítica da equipe |
|---|---|---|---|
| Ex.: ChatGPT, Gemini, Copilot, Claude, Cursor etc. | Ex.: revisar texto, gerar esboço de código, depurar erro, sugerir regras. | Descrever resumidamente, sem copiar conversas completas. | Explicar o que foi aceito, corrigido, rejeitado, testado ou validado. |

> Declarar o uso de IA não reduz a nota. O que reduz a nota é usar IA sem compreender, sem revisar, sem validar, sem citar fontes ou sem declarar. Quando a IA for usada para gerar código, documentação ou testes, a equipe deverá garantir que o material presente no GitHub tenha sido revisado, executado e validado pelos integrantes.

---

## 10. Estrutura recomendada para o documento principal

A estrutura poderá variar conforme a modalidade escolhida, mas deverá permitir avaliação técnica clara.

| Parte | Conteúdo |
|---|---|
| **1** | Capa ou cabeçalho com título, turma, equipe, integrantes, opção escolhida e link do repositório GitHub. |
| **2** | Resumo, introdução, motivação, descrição do problema e justificativa para uso de lógica fuzzy. |
| **3** | Fundamentação teórica e, conforme a opção, trabalhos relacionados ou análise de mercado/requisitos. |
| **4** | Metodologia, modelagem fuzzy, variáveis, universos de discurso, funções de pertinência e base de regras. |
| **5** | Implementação, arquitetura do sistema, dependências, estrutura do GitHub, interface ou modo de execução. |
| **6** | Experimentos, cenários de teste, resultados, gráficos/tabelas, análise crítica e limitações. |
| **7** | Conclusão, trabalhos futuros, declaração de uso de IA, referências e apêndices. |

---

## 11. Rubrica de avaliação (0,0–2,0 pts)

A tabela abaixo descreve os critérios de avaliação a serem utilizados durante o trabalho. O peso de cada critério orienta a avaliação, mas o professor poderá considerar a coerência global do trabalho, a dificuldade da solução escolhida e a qualidade da defesa oral.

| Critério | Peso | Como será avaliado |
|---|---|---|
| **1. Escolha do problema e fundamentação** | 0,25 | Relevância do problema, justificativa do uso de fuzzy, pesquisa bibliográfica ou análise de mercado, qualidade das fontes, clareza das hipóteses e delimitação do escopo. |
| **2. Modelagem fuzzy** | 0,40 | Coerência das entradas e saídas, universos de discurso, termos linguísticos, funções de pertinência, regras, operadores, inferência, defuzzificação ou cálculo TSK. Avalia-se mais a consistência técnica do que a quantidade mecânica de elementos. |
| **3. Implementação e funcionamento** | 0,30 | Sistema funcional, código organizado, execução reproduzível, compatibilidade entre relatório e implementação, clareza do manual, qualidade da demonstração e consistência com o código publicado no GitHub. |
| **4. Experimentos e análise** | 0,30 | Cenários de teste, casos extremos/fronteiriços, tabelas e gráficos, interpretação dos resultados, discussão de limitações, análise de sensibilidade ou comparação com referência. |
| **5. Documento escrito** | 0,20 | Estrutura, clareza, linguagem técnica, completude, figuras e tabelas, referências, adequação ao formato escolhido: artigo, relatório técnico ou documentação de produto. |
| **6. Apresentação, demonstração e arguição** | 0,35 | Organização da exposição, demonstração funcional, participação dos integrantes, domínio conceitual e capacidade de responder perguntas técnicas sem depender apenas da leitura de slides. |
| **7. GitHub, reprodutibilidade, integridade e uso de IA** | 0,20 | Existência e acessibilidade do GitHub; README; organização de arquivos; instruções de execução; compatibilidade entre código, relatório e apresentação; histórico ou evidência de desenvolvimento; declaração de uso de IA; referências corretas e honestidade metodológica. |

### 11.1 Faixas qualitativas de desempenho

| Faixa | Interpretação |
|---|---|
| **Excelente** | Trabalho funcional, bem modelado, bem justificado, reprodutível, organizado no GitHub e defendido com segurança. Apresenta análise crítica e evidências fortes. |
| **Adequado** | Atende aos principais requisitos, possui implementação funcional, documentação suficiente e GitHub acessível, ainda que com limitações pontuais. |
| **Parcial** | Há implementação ou documentação, mas com lacunas relevantes em modelagem, validação, clareza, GitHub ou domínio conceitual. |
| **Insuficiente** | O sistema não executa, a modelagem fuzzy é frágil, o GitHub está ausente/inacessível, o relatório não permite avaliação técnica ou a equipe não consegue explicar o que entregou. |

---

## 12. Penalidades, limites de nota e situações críticas

As penalidades poderão ser aplicadas cumulativamente, sempre considerando gravidade, reincidência, prejuízo à avaliação e evidências apresentadas.

| Situação | Consequência possível |
|---|---|
| Ausência de sistema fuzzy funcional | Redução severa de nota ou nota zero, dependendo do que foi entregue. |
| Código ausente, não executável ou incompatível com o relatório | Redução proporcional, podendo comprometer implementação, GitHub e reprodutibilidade. |
| Repositório GitHub ausente, inacessível, vazio ou sem permissão ao professor | Nota zero no critério de GitHub e reprodutibilidade. Dependendo do prejuízo à avaliação do código, a nota final poderá ser limitada a 1,4 pts. |
| GitHub sem README, sem instruções de execução ou com organização insuficiente | Redução no critério de GitHub, reprodutibilidade e implementação. Se o problema impedir execução ou avaliação, outras penalidades poderão ser acumuladas. |
| Link do GitHub enviado incorretamente, quebrado ou apenas após o prazo | Tratado como entrega incompleta ou fora do prazo, conforme a gravidade e as regras do Google Classroom. |
| Base de regras arbitrária, pequena demais ou sem relação com o domínio | Redução na modelagem fuzzy e na análise técnica. |
| Funções de pertinência sem justificativa, sem parâmetros ou sem visualização | Redução na modelagem e na documentação. |
| Ausência de validação experimental | Redução relevante em experimentos e discussão crítica. |
| Trabalho muito parecido com exemplo de sala ou tutorial público | Nota poderá ser limitada a 1,2 pts; em caso de cópia literal, poderá ser atribuída nota zero. |
| Uso de IA sem declaração ou sem compreensão | Redução proporcional; se houver autoria falsa, plágio ou falsificação, poderá haver nota zero. |
| Integrante sem participação ou sem domínio mínimo na apresentação | Redução individual na parte de apresentação/arguição, a critério do professor. |
| Entrega fora do prazo | Sujeita às regras do Google Classroom e às orientações do professor. |
| Similaridade entre trabalhos, protótipos, artigos idênticos escolhidos por mais de uma equipe | Nota poderá ser limitada a 1,0 pts conforme o grau de similaridade for estabelecido; em caso de cópia literal, poderá ser atribuída nota zero. |

---

## 13. Checklist geral de entrega

- [ ] Equipe e opção escolhida identificadas.
- [ ] Problema prático definido e justificado.
- [ ] Entradas, saída e universos de discurso definidos.
- [ ] Funções de pertinência especificadas e visualizadas.
- [ ] Base de regras completa e justificada.
- [ ] Inferência/defuzzificação ou cálculo TSK explicado.
- [ ] Sistema implementado e funcional.
- [ ] Cenários de teste executados e analisados.
- [ ] Código-fonte organizado.
- [ ] Repositório GitHub criado e acessível.
- [ ] README do GitHub completo.
- [ ] Manual de execução incluído.
- [ ] Documento principal finalizado.
- [ ] Slides preparados.
- [ ] Uso de IA declarado.
- [ ] Referências citadas corretamente.
- [ ] Arquivos e link do GitHub submetidos no Classroom.
- [ ] Todos os integrantes preparados para arguição.

### 13.1 Checklists específicos por modalidade

**Opção A**
- [ ] Artigos pesquisados
- [ ] Tabela com pelo menos 5 trabalhos relacionados
- [ ] Escolha A1 ou A2 declarada
- [ ] Reprodução/adaptação implementada
- [ ] Comparação com literatura
- [ ] Artigo técnico/científico estruturado

**Opção B**
- [ ] Público-alvo definido
- [ ] Proposta de valor
- [ ] Requisitos
- [ ] Protótipo demonstrável
- [ ] Documentação de produto
- [ ] GitHub com código e instruções de execução
- [ ] Apresentação com defesa técnica e demonstração

**Opção C**
- [ ] Modalidade C-A ou C-B declarada
- [ ] Regras com consequentes TSK
- [ ] Cálculo ponderado explicado
- [ ] Adequação do TSK justificada
- [ ] Comparação conceitual ou experimental com Mamdani discutida

**5 integrantes**
- [ ] Trilha de ampliação escolhida
- [ ] Tarefa extra executada
- [ ] Contribuição de cada integrante documentada
- [ ] Todos aptos à arguição

**Pontuação extra**
- [ ] Opção Neuro-Fuzzy ou AG/PSO declarada
- [ ] Implementação integrada ao projeto
- [ ] Código da extensão presente no GitHub
- [ ] Função objetivo ou arquitetura explicada
- [ ] Resultados antes/depois ou análise comparativa apresentados

---

## 14. Orientações finais

Serão valorizados trabalhos que demonstrem pesquisa real, modelagem fuzzy consistente, implementação funcional, validação experimental, organização no GitHub, boa comunicação técnica e domínio conceitual durante a arguição. A equipe deve estar preparada para responder por que escolheu aquelas variáveis, por que as funções de pertinência são adequadas, por que as regras fazem sentido e como os resultados devem ser interpretados.

A nota máxima será reservada a trabalhos suficientemente complexos, reproduzíveis, bem documentados, devidamente publicados no GitHub e defendidos com segurança. A equipe não precisa construir um sistema perfeito; precisa construir uma **solução fuzzy tecnicamente coerente, demonstrável, defensável, analisada com honestidade e compatível com o tempo disponível**.

---

*Página 10 de 10 — 08/05/2026*
