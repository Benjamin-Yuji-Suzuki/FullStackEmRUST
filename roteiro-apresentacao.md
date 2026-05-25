# FuzzySimulated — Roteiro de Apresentação (Ao Vivo, sem Slides)

> **Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados · Resolução de Problemas Multivariáveis — CESUPA 02/2026  
> **Equipe:** Benjamin Yuji Suzuki  
> **Duração estimada:** 25–30 minutos  

---
## 1. Abertura (~1min)

"Bom dia. Meu nome é Benjamin Yuji Suzuki e vou apresentar o **FuzzySimulated**, uma plataforma full-stack 100% Rust para construção e simulação de sistemas de inferência fuzzy. O projeto integra quatro disciplinas: Qualidade de Software, Inteligência Artificial, Ciência de Dados e Resolução de Problemas Multivariáveis.

Não preparei slides — vou mostrar o sistema rodando ao vivo e explicar cada parte enquanto navego."

---
## 1.5 Sprint 3 — Entrega e Apresentação Final (30s)

"Este trabalho representa a entrega final do **Sprint 3** do projeto. Entreguei o sistema completo e funcional, com todas as 5 telas e os 20 casos de uso implementados (após remoção dos UC21-UC25 da disciplina de Matemática, que tem projeto separado). 

A suíte de testes está completa nos três níveis:
- Unitários: 46 testes (30 inline + 16 tests/)
- Integração HTTP: 64 testes  
- End-to-end: 41 testes Playwright
- Integração DB: 6 testes ignorados (transaction rollback)

Inclui relatórios de cobertura de código (meta 70-80% atingida com engine em 91.76%) e análise estática de qualidade via SonarQube. A apresentação ao vivo está acontecendo agora mesmo, com execução das suítes de testes diante da turma e explicação das decisões técnicas tomadas."

---
## 2. Por que um Sistema Fuzzy? (~1min30s)

"O tema do projeto é livre. Escolhi **sistemas de inferência fuzzy** porque:

- **Modelam incerteza** — diferente da lógica binária (0 ou 1), a lógica fuzzy admite valores parciais. Exemplo: conforto térmico — 25°C com 60% de umidade não é simplesmente 'confortável' ou 'desconfortável', mas um grau entre os dois.
- **Aplicação real em cibersegurança** — classificar risco de incidentes considerando múltiplas variáveis (probabilidade, impacto, vulnerabilidade) é um problema inerentemente fuzzy.
- **Ferramentas existentes são limitadas** — MATLAB Fuzzy Toolbox é proprietário, JFuzzyLogic é Java legada. Não há uma plataforma moderna, open-source, em Rust.

O nome 'FuzzySimulated' reflete o objetivo: simular inferência fuzzy com motores Mamdani, TSK e otimização PSO."

---
## 3. Stack Tecnológica (~1min30s)

"A stack é 100% Rust. Não tem uma linha de JavaScript, HTML ou CSS escrito manualmente — tudo é gerado pelo framework.

**Frontend:** Leptos 0.8 — framework reativo que compila Rust para WASM. Ele faz SSR (server-side rendering) na primeira carga e depois hidrata no navegador com WASM. Isso significa que a página carrega rápido (HTML pronto) e depois fica interativa sem JS.

**Backend:** Axum 0.8 — framework async type-safe do ecossistema Tokio. Integração nativa com Tower middleware. As rotas REST ficam em `server/src/routes/`.

**Banco:** PostgreSQL com SQLx. O SQLx verifica queries em tempo de compilação — se eu escrever um SELECT com coluna errada, o código nem compila. Uso JSONB para dados semi-estruturados como termos fuzzy, inputs de simulação e snapshots de auditoria.

**Motor Fuzzy:** Crate própria chamada `logicfuzzy-academic`, publicada no crates.io. Implementa Mamdani, TSK, PSO (Particle Swarm Optimization), exportação SVG e relatório de diagnóstico.

**Build:** `cargo-leptos` compila WASM + servidor + CSS num único comando. Uso dart-sass + Lightning CSS para o `style/main.scss`.

**API Externa:** OpenWeather para dados reais de temperatura e umidade por cidade."

---
## 4. Arquitetura Geral (~1min)

"O sistema segue uma arquitetura cliente-servidor com SSR.

O Leptos renderiza o HTML no servidor — quando você acessa a URL, já chega HTML pronto. Depois da hidratação WASM, as interações do usuário (clicar em botão, mudar slider) disparam chamadas `fetch` para a REST API Axum. A API processa a lógica de negócio (validação, motor fuzzy) e persiste no PostgreSQL via SQLx. Dados climáticos reais vêm da OpenWeather.

O banco tem **8 tabelas** com chaves UUID e campos JSONB. São **9 migrations versionadas**, da 001 até a 009. A última migration (009) é o seed que povoa 4 sistemas de demonstração."

---
## 5. Arquitetura do Framework Rust e Organização de Pastas (~2min)

"Vamos falar um pouco sobre como o Rust organiza projetos, o que é diferente de outros ecossistemas como Node.js ou Python.

**Workspace Cargo:** Este projeto utiliza um *workspace* do Cargo, definido no `Cargo.toml` raiz. Um workspace permite múltiplos crates (bibliotecas ou binários) compartilharem dependências e profiles de compilação. Meu workspace tem três crates:

- `server`: o backend Axum (binário)
- `app`: código compartilhado Leptos (componentes + funções de servidor)
- `frontend`: ponto de entrada WASM (binário)

Isso reduz duplicação: por exemplo, tipos como `FuzzySystem` são definidos uma vez no `server` e reutilizados no `app` via dependência interna.

**Por que pastas reduzidas?** Em projetos full-stack tradicionais (ex: React + Express), você geralmente tem pastas separadas como `src/`, `components/`, `routes/`, `models/`, `services/` etc., duplicando estrutura entre frontend e backend. No Rust/Leptos, grande parte da lógica vive no mesmo crate `app`, e o backend tem sua própria estrutura enxuta. Isso resulta em menos arquivos no total, apesar de cada crate ter seu próprio `src/`.

**Curiosidade:** Enquanto um projeto full-stack equivalente em JavaScript (React + Node + Express) pode ter 50-100 arquivos de código-fonte (JS/TS, HTML, CSS, config), meu projeto Rust tem cerca de 35 arquivos `.rs` — mas cada um tende a ser mais denso devido ao forte sistema de tipos e expressividade da linguagem. O diferencial é a segurança em tempo de compilação: erros que em JS só aparecem em runtime (como `undefined is not a function`) aqui impedem o código de compilar.

**Exemplo de problema encontrado e solução:** Durante o desenvolvimento, percebi que estava repetidamente usando `.unwrap()` em resultados de operações que poderiam falhar (como buscas no banco ou parsing de JSON). Isso é perigoso porque pode causar panic em produção. Substitui todos por tratamento explícito de erro usando `match` ou `?` operator, retornando `AppError` adequado. Isso melhorou a robustez e passou no SonarQube sem violações.

Outro problema: uso excessivo de `#[allow(dead_code)]` em estruturas inteiras. Movi essas atribuições para campos específicos que realmente não são usados em certos variantes, reduzindo warnings desnecessários.

E o `is_ok()`? Em testes, muitas vezes verificamos se um `Result` é ok com `assert!(result.is_ok())` em vez de simplesmente fazer `result.unwrap()` e deixar o teste panic em caso de falha. Isso dá mensagens de erro mais claras quando algo dá errado."

---
## 6. Problemas Encontrados e Soluções (~2min)

"Durante o desenvolvimento, encontrei diversos problemas interessantes que valem a pena compartir:

1. **Deadlock em testes paralelos com PostgreSQL:** Inicialmente, meus testes HTTP rodavam em paralelo e todos faziam `TRUNCATE CASCADE` no banco, causando deadlock. Solução: usei a crate `serial_test` com atributo `#[serial]` para serializar o acesso ao banco, além de dar a cada teste seu próprio connection pool via fixture `TestApp`.

2. **Migração de seed com duplicação estrutural:** O SonarQube acusava alta duplicação no migration 002_seed.sql porque eu tinha blocos INSERT quase idênticos para quatro sistemas. Resolvi criando uma constante JSONB no migration 009_reset_and_seed.sql e usando um loop PL/pgSQL para inserir os dados programaticamente, eliminando a duplicação literal.

3. **Wasm bundle size inicial grande:** O primeiro build do WASM tinha quase 2MB. Após ativar otimizações no `Cargo.toml` (`opt-level = 'z'`, `lto = true`, `strip = true`) e remover dependências desnecessárias, reduzi para ~400KB — aceitável para carregamento inicial.

4. **Inconsistência em funções de pertinência:** Descobri que minha validação de `gaussmf` aceitava sigma=0, o que resulta em divisão por zero na inferência. Adicionei validação rigorosa em `validation.rs` e testes correspondentes em `mf_validation.rs`.

5. **Snapshots de auditoria vazando relacionamentos:** Ao implementar o undo, percebi que simplesmente restaurar um sistema deletado não reconectava suas variáveis/termos/regras porque as chaves estrangeiras estavam apontando para UUIDs que agora pertenciam a outros registros. Solução: usei `ON DELETE SET NULL` nas FKs da tabela `audit_events` e, ao fazer undo, re-insiro os registros orphanados com seus UUIDs originais.

6. **Diferença entre compilação debug e release:** Alguns testes passavam no debug mas falhavam no release devido a otimizações que expunham bugs de race condition em lógica de estado compartilhado. Resolvi usando `Mutex` e `RwLock` onde apropriado e revisando o acesso concorrente no engine."

---
## 7. Demo — Navegação no Sistema (navegando ao vivo)

### 7.1 Dashboard (~30s)

[Abra `http://127.0.0.1:3000`]

"Aqui é o dashboard. Mostra a lista de sistemas fuzzy com KPIs: número de variáveis, termos e regras de cada sistema. Cada sistema tem um card com badge de status — ativo, favorito, concluído ou desativado. No canto superior, o link para 'Novo Sistema'."

### 7.2 Sistema Seed 'Risco Cibernético' (~1min)

[Acesse um sistema seed]

"Vou entrar no sistema 'Risco Cibernético Avançado'. Esse é o modelo de demonstração mais completo. Tem 4 variáveis de entrada: Probabilidade, Impacto Financeiro, Vulnerabilidade e Nível de Habilidade do Atacante. A variável de saída é o Nível de Risco, com termos como Baixo, Médio, Alto, Crítico.

As abas organizam: dashboard do sistema, variáveis, termos, regras, simulação, análise, histórico, cenários."

### 7.3 Variáveis e Termos (~1min)

[Vá para a aba Variáveis]

"Cada sistema tem variáveis que podem ser antecedentes (entrada) ou consequente (saída). Cada variável tem termos linguísticos com funções de pertinência.

Mostrando aqui: a variável 'Probabilidade' tem 3 termos — Baixa, Média, Alta — cada um com parâmetros diferentes. Três tipos de MF disponíveis: `trimf` (triangular, parâmetros a≤b≤c), `trapmf` (trapezoidal, a≤b≤c≤d), `gaussmf` (gaussiana, sigma>0). A validação é rigorosa — se tentar salvar parâmetros inválidos, o backend rejeita."

### 7.4 Regras Fuzzy (~1min)

[Vá para a aba Regras]

"As regras seguem o formato textual: 'SE var é termo E ... ENTÃO var é termo'. Exemplo: 'SE Probabilidade é Alta E ImpactoFinanceiro é Alto ENTÃO NivelRisco é Crítico'.

Aqui estão 42 regras para o sistema de risco cibernético. É possível criar, editar e deletar. O sistema valida se as variáveis e termos existem antes de salvar."

### 7.5 Simulação Mamdani (~2min — parte central)

[Vá para Simular → aba Mamdani]

"Vou simular um cenário: probabilidade 80, impacto financeiro 90, vulnerabilidade 85, habilidade do atacante 70.

Clico em 'Simular'. O resultado: nível de risco 87, classificado como 'Crítico'. O sistema mostra:

- O valor numérico de saída (87)
- O label linguístico (Crítico)
- O gráfico SVG da variável de saída com a linha vertical no resultado
- Cada termo com seu grau de ativação

O motor Mamdani funciona em 3 etapas:
1. **Fuzzificação** — calcula o grau de pertinência de cada input em cada termo
2. **Agregação** — operador MIN (E) combina os antecedentes de cada regra
3. **Defuzzificação** — centroide da área resultante"

### 7.6 Diagnóstico (~1min)

[Vá para aba Diagnóstico]

"A aba Diagnóstico mostra o relatório detalhado: para cada regra, o grau de ativação individual, quais termos foram disparados e o resultado parcial. É útil para entender *por que* o sistema chegou naquele resultado — explicabilidade, essencial em IA."

### 7.7 TSK (~1min)

[Vá para aba TSK]

"O motor TSK (Takagi-Sugeno-Kang) é alternativo ao Mamdani. Em vez de termos linguísticos na saída, cada regra tem um polinômio — por exemplo, 'SE Probabilidade é Alta ENTÃO risco = 0.7*prob + 0.3*impacto'. A saída final é a média ponderada dos polinômios.

A vantagem do TSK ser computacionalmente mais eficiente e integrar melhor com otimização."

### 7.8 SVG (~30s)

[Vá para a aba SVG]

"Aqui exporto o gráfico de pertinência de qualquer variável em SVG. Clico em 'Exportar SVG' e baixo um arquivo vetorial — pode ser aberto no navegador, editado no Inkscape ou inserido em relatórios."

### 7.9 Superfície + Sweep + Matriz (~1min)

[Vá para Análise]

"A aba Análise tem três visualizações:

**Superfície:** grid 5x5 de inputs com a saída mapeada em tons de cor. Mostra como dois parâmetros interagem.

**Sweep:** varre um parâmetro em 11 pontos e plota a curva de resposta.

**Matriz de Regras:** tabela mostrando quais regras foram ativadas e com que força para o input atual."

### 7.10 Batch (~1min)

[Vá para Simular → Batch]

"O processamento em lote permite simular múltiplos inputs de uma vez. Posso carregar um arquivo CSV ou Parquet com várias linhas de inputs, e o sistema processa todas de uma vez. O resultado é exportável.

Útil para análise de dados em escala — por exemplo, simular 100 cenários de ataque diferentes e ver a distribuição dos níveis de risco."

### 7.11 Histórico e Comparação (~1min)

[Vá para Histórico]

"Cada simulação é salva no histórico. Posso ver resultados anteriores, comparar lado a lado — por exemplo, o cenário de hoje vs o de ontem — e ver como mudanças nos inputs afetam a saída."

### 7.12 Otimizador PSO (~2min)

[Vá para Otimizador]

"O **PSO (Particle Swarm Optimization)** é um algoritmo populacional que ajusta parâmetros das funções de pertinência para maximizar/minimizar uma função objetivo. Útil para calibrar automaticamente os termos fuzzy com base em dados de referência.

O usuário fornece pares de input-output desejados, e o PSO encontra a melhor configuração dos parâmetros das MF para aproximar esse comportamento."

### 7.13 Cenários (~30s)

"Posso salvar combinações de inputs como cenário. Um cenário 'Ataque Alta Severidade' pré-preenche sliders com valores específicos. Útil para simulações recorrentes."

### 7.14 Auditoria com Undo (~1min)

[Vá para Auditoria]

"A auditoria registra cada alteração no sistema — criação, edição, exclusão de sistema, variável, termo ou regra. Cada evento tem um snapshot JSONB do estado before e after.

A funcionalidade Undo restaura o estado anterior. Por exemplo, se eu deletar um sistema, posso voltar atrás e restaurá-lo com todas as variáveis, termos e regras intactos.

Isso foi implementado com trigger ON DELETE SET NULL + re-inserção dos registros orphanados."

### 7.15 OpenWeather (~30s)

[Vá para o endpoint Weather ou componente de clima]

"Integração com OpenWeather. Digito 'Belém' e o sistema busca temperatura e umidade reais da cidade. Esses dados podem ser usados como input no sistema 'Conforto Térmico' para simular o conforto ambiental local."

### 7.16 Importar/Exportar/Duplicar (~30s)

"O sistema permite exportar um sistema completo como JSON — baixa tudo: sistema, variáveis, termos, regras, cenários. Esse JSON pode ser importado em outra instância. Duplicar cria uma cópia exata com novo UUID."

---
## 8. Testes — Estratégia e Cobertura (~3min)

"Vou mostrar os testes. Uso três níveis de teste mais um nível de integração DB ignorado."

### 8.1 Testes Unitários

[Abra terminal, rode `cargo test -p server --lib`]

"30 testes inline direto no código-fonte. Testam funções de pertinência (trimf, trapmf, gaussmf), parser de regras, inferência Mamdani, mapeamento de erros para HTTP."

[Rode também `cargo test -p server --test unit` no terminal]

"Mais 16 testes unitários em `tests/unit/` — validação de MF, criação de sistema. Total: **46 testes unitários**."

**Por que em Rust?** O Rust tem suporte nativo a testes com os atributos `#[cfg(test)]` e `#[test]` — não preciso instalar framework externo, biblioteca de assertions ou runner. Basta escrever a função com `#[test]` e rodar `cargo test`. Os testes podem ficar inline (dentro do próprio módulo) ou em arquivos separados em `tests/`. Isso é uma vantagem enorme sobre C/Java/Python, onde você precisa configurar um framework separado."

### 8.2 Testes HTTP (Integração)

[Rode `DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test axum_api` no terminal]

"64 testes HTTP que batem na API real com banco PostgreSQL. Cada teste cria seu próprio pool, faz `TRUNCATE CASCADE` no setup, roda serializado com `#[serial_test::serial]` pra evitar deadlock.

Cobrem todos os endpoints: CRUD de sistemas, variáveis, termos, regras, simulação Mamdani/TSK/SVG/Diagnóstico, sweep, superfície, batch, PSO, auditoria, weather."

**Por que em Rust?** Uso o mesmo `Router` do Axum com um `TestApp` customizado que cria um pool de conexão isolado. O cliente HTTP é o `reqwest` (crate Rust). A vantagem é que não preciso mockar nada — o mesmo código que roda em produção é usado nos testes. Se um endpoint funciona no teste, funciona no servidor real. Outra vantagem: o SQLx compila as queries; se a migration mudar e o SELECT ficar inconsistente, o teste nem compila."

### 8.3 Testes de Integração DB

[Rode `DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server -- --ignored`]

"6 testes com `#[ignore]` que usam transações com rollback. Testam persistência real: criar sistema e verificar no banco, cascade delete, simulação que persiste."

### 8.4 Testes E2E

"41 testes com Playwright + Chromium contra o servidor rodando. Simulam jornadas completas: navegação, CRUD, simulação, delete protection, full lifecycle de 20 operações."

**Por que Playwright e não Rust?** O ecossistema de automação de navegador em Rust ainda é imaturo — existem opções como `headless_chrome` e `fantoccini`, mas sofrem com instabilidade, falta de suporte a navegadores múltiplos e APIs limitadas. Playwright é o padrão-ouro da indústria para testes E2E, desenvolvido pela Microsoft, roda qualquer navegador (Chromium, Firefox, Safari), tem espera automática, gera screenshots e vídeos, e a sintaxe é concisa. Não valia a pena reinventar a roda em Rust para algo que Playwright já faz perfeitamente. O teste em si é simples — 'faça login, clique aqui, veja se aparece isso' — não precisa ser na mesma linguagem do backend."

### 8.5 Total

"**157 testes no total**: 46 unit + 64 HTTP + 6 integration + 41 E2E."

### 8.6 Cobertura

[Mostre o `coverage/html/index.html` se tiver]

"Uso `cargo-llvm-cov` para cobertura. Engine: 91.76%. Errors: 100%. Validation: 98.15%. Média do server ~70%. Atingi a meta de 70-80% para os módulos centrais de lógica de negócio."

---
## 9. Qualidade — SonarQube (+-1min)

"Uso **SonarQube Cloud** para análise estática contínua. Principais métricas:

**Duplicação:** o seed SQL tinha ~30% de duplicação porque eu repetia blocos INSERT para 4 sistemas. Resolvi reescrevendo o migration 009 com uma única constante JSONB e iterando programaticamente. Também extraí labels repetidas para CONSTANT PL/pgSQL. Resultado: zero duplicação estrutural e literal.

**Code Smells:** substituí todos os `unreachable!()` por `Err(AppError::Validation)`, `panic!()` em audit_routes por `Result`, `unwrap()` em app/lib.rs por pattern matching. Movi `#[allow(dead_code)]` de struct-level para field-level.

**Segurança:** `cargo audit` reporta 0 vulnerabilidades. Um advisory do `rsa` (Marvin Attack) apareceu no meu relatório porque o `sqlx-mysql` é uma dependência transitiva de alguns crates de teste. Embora eu não estivesse usando MySQL em nenhum lugar do projeto (apenas PostgreSQL via `sqlx-postgres`), removi qualquer referência a MySQL do `Cargo.lock` e garanti que apenas o driver PostgreSQL fosse incluído. Isso eliminou o advisory e reforçou a postura de segurança: nenhuma superfície de ataque desnecessária."

---
## 10. Lições Aprendidas (~1min30s)

"**Rust full-stack é viável, mas exige paciência.** Leptos + WASM elimina JavaScript completamente, mas o ecossistema ainda é jovem — muitos warnings vêm do próprio framework, não do meu código.

**Testes paralelos com banco compartilhado são um desafio.** TRUNCATE CASCADE concorrente causa deadlock no PostgreSQL. Solução: serial_test + pool isolado.

**Comecei implementando o motor fuzzy manualmente**, depois migrei para a crate `logicfuzzy-academic`, que abstrai 599 linhas de lógica de pertinência, inferência e PSO.

**JSONB para auditoria com undo real** — snapshots before/after em JSONB permitem restaurar qualquer estado anterior. O desafio foi preservar relacionamentos ao restaurar sistemas deletados, resolvido com ON DELETE SET NULL + re-inserção.

**SonarQube com SQL** — seed data com blocos repetidos gera falso positivo de duplicação. Solução: loops programáticos + JSONB constante.

**Diferença cultural:** sendo o único na turma fazendo full-stack em uma linguagem diferente (Rust em vez de JavaScript/TypeScript), tive que explicar constantemente minhas escolhas. Mas esse diferencial trouxe vantagens: segurança de tipos elimina toda uma classe de bugs, o desempenho é superior e a experiência de aprender Rust profundamente valeu o esforço adicional."
---

## 11. Histórico de Desenvolvimento (últimos 40 commits)

"Para contextualizar a evolução do projeto, resumo os últimos 40 commits:

**Fase 1 — Reorganização e Expansão (commits 40 a 30 atrás):**
Criei o seed 007 (Análise de Risco) com 9 regras e 6 cenários, scripts CLI para PSO e otimização quadrática, presets PSO no frontend e o botão Aplicar Parâmetros que persiste no banco. Refatorei a grid de superfície para `repeat(n,8px)` e a matriz de regras para visual heatmap. Adicionei `FuzzyTermWithVar` para consultas de termo com variável.

**Fase 2 — Correções SonarQube e Refatoração (commits 29 a 10 atrás):**
Foco total em qualidade. Corrigi E2E (Number.isNaN, crypto.randomUUID), substituí `unreachable!()` por `AppError::Validation`, `panic!()` por `Result`, `unwrap()` por `if let Some`. Refatorei todo o migration 009 — extraí labels duplicadas para CONSTANT PL/pgSQL, usei temp tables e JSONB constante para eliminar 30% de duplicação estrutural. Movi `#[allow(dead_code)]` de struct-level para field-level. Gerei `package-lock.json` que estava ausente.

**Fase 3 — Remoção UC21-25 e Finalização (commits 9 a 1 atrás):**
Removi completamente os casos de uso UC21-UC25 (otimização quadrática — Hessiana, gradiente, Cramer) porque a disciplina de Resolução de Problemas Multivariáveis tem projeto próprio separado. Deletei 7 arquivos de código e 3 arquivos de teste. Atualizei todas as documentações com as novas contagens (116 testes server, 20 casos de uso). Corrigi 13 erros de compilação pré-existentes no crate `app` (funções ausentes em server_fns.rs). Finalizei o roteiro de apresentação que estou usando agora.

Total: ~40 commits, 7 arquivos deletados, 25+ arquivos modificados, do seed inicial ao sistema completo para o Sprint 3."

---

## 12. Encerramento (~30s)

"O repositório está em `github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST`. A crate do motor está em `crates.io/crates/logicfuzzy_academic`.

Para rodar: `cargo leptos watch` e acessar `http://127.0.0.1:3000`.

Obrigado pela atenção. Estou aberto a perguntas."
