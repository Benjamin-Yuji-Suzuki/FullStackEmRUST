# Declaração de Uso de IA

**Disciplina:** Inteligência Artificial e Computacional (0700M8)
**Professor:** Prof. Daniel Leal Souza
**Equipe:** Benjamin Yuji Suzuki
**Projeto:** FuzzySimulated — Plataforma de Inferência Fuzzy

Conforme a Seção 9 da lauda do trabalho, declaramos abaixo o uso de ferramentas de IA durante o desenvolvimento do projeto.

## Tabela de Declaração

| Ferramenta | Finalidade | Prompt/Comando Resumido | Revisão Crítica da Equipe |
|---|---|---|---|
| **Claude (opencode)** | Geração de código do motor fuzzy (engine.rs) | "Implemente funções de pertinência trimf, trapmf, gaussmf em Rust" / "Implemente inferência Mamdani completa com fuzzificação, agregação min e defuzzificação centroide" | Código revisado linha a linha; testes unitários validam cada função matemática; parâmetros de MF validados contra valores negativos, NaN e infinito |
| **Claude (opencode)** | Geração de código do motor TSK | "Implemente inferência Takagi-Sugeno-Kang com consequentes polinomiais e média ponderada" | Valores de saída conferidos contra cálculo manual; cobertura de testes específicos para TSK |
| **Claude (opencode)** | Geração de código do PSO | "Implemente otimização PSO para parâmetros de funções de pertinência" | Algoritmo verificado quanto à convergência (66% de redução de erro); splitmix64 PRNG implementado manualmente para evitar dependência externa |
| **Claude (opencode)** | Geração de componentes Leptos (frontend) | "Crie página do simulador com abas Mamdani/TSK/SVG/Diagnóstico" / "Crie dashboard com cards de sistemas e seletor de status" | Interface testada com Playwright E2E (40 testes); componentes validados visualmente |
| **Claude (opencode)** | Geração de rotas Axum (backend) | "Crie rota CRUD para sistemas fuzzy com validação e auditoria" / "Crie rota de simulação com persistência no banco" | Rotas testadas via HTTP Axum (64 testes); respostas HTTP validadas (status codes, formato JSON) |
| **Claude (opencode)** | Geração de testes | "Crie testes HTTP para a rota de variáveis" / "Crie testes unitários para validação de funções de pertinência" | Todos os testes executam e passam; `is_ok()` substituído por `expect()` para melhor depuração |
| **Claude (opencode)** | Geração de migrations SQL | "Crie migration SQL para seed de sistema de risco cibernético com JSONB" | Migrations testadas com rollback em transação; dados seed verificados no banco |
| **Claude (opencode)** | Elaboração de documentação técnica | "Escreva relatório seguindo a estrutura da lauda Seção 10" / "Crie slides de apresentação" | Conteúdo revisado e ajustado para refletir exatamente o que foi implementado |
| **GitHub Copilot** | Autocompletar código durante edição | Sugestões inline contextuais | Cada sugestão revisada antes de ser aceita; trechos incorretos rejeitados ou modificados |

## Detalhamento

### Como a IA foi utilizada

1. **Geração de código estrutural**: A IA foi utilizada para gerar esqueletos de código (rotas Axum, componentes Leptos, funções do motor fuzzy), que foram posteriormente refinados manualmente.

2. **Depuração e correção**: Em diversos momentos, a IA auxiliou na identificação de bugs (ex: validação de parâmetros NaN, ordenação a≤b≤c em trimf, tratamento de UUIDs inválidos).

3. **Otimização e refatoração**: A IA sugeriu refatorações como a substituição de `assert!(result.is_ok())` por `result.expect()`, a organização dos testes em módulos por domínio, e a implementação de helpers compartilhados.

### O que NÃO foi feito por IA

- Definição das variáveis linguísticas e universos de discurso (definidas manualmente com base em domínio de cibersegurança)
- Escolha das funções de pertinência e seus parâmetros (baseada em conhecimento do problema)
- Elaboração da base de regras (12 regras definidas manualmente para cobrir casos típicos, críticos e fronteiriços)
- Validação e interpretação dos resultados experimentais
- Decisões arquiteturais (stack Rust, Leptos, Axum, PostgreSQL)

### Garantia de Qualidade

Todo código gerado ou sugerido por IA foi:
1. Revisado manualmente linha a linha
2. Compilado sem erros (`cargo check`)
3. Testado (124 testes de servidor + 40 testes E2E)
4. Validado quanto à corretude dos resultados matemáticos
5. Verificado quanto a segurança (sem exposição de chaves, sem SQL injection, sem unsafe code desnecessário)

---

**Declaro que o uso de IA foi transparente, revisado e validado, e que todo o material entregue reflete o trabalho e a compreensão da equipe.**

Benjamin Yuji Suzuki
Junho/2026
