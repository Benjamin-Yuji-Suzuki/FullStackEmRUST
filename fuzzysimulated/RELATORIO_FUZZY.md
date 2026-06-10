# FuzzySimulated — Plataforma de Inferência Fuzzy

**Sistema de Controle Fuzzy para Avaliação de Risco em Cibersegurança**

---

| Campo | Descrição |
|---|---|
| **Disciplina** | Inteligência Artificial e Computacional (0700M8) |
| **Professor** | Prof. Daniel Leal Souza |
| **Turma** | CC5MA / CC5NA |
| **Equipe** | Benjamin Yuji Suzuki (1 integrante) |
| **Modalidade** | Opção B (Produto) + Opção C-B (TSK) + Pontuação Extra PSO |
| **Repositório** | [github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST](https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST) |
| **Data** | Junho/2026 |

---

## 1. Resumo

Este trabalho apresenta o **FuzzySimulated**, uma plataforma full-stack 100% Rust para construção, simulação e validação de sistemas de inferência fuzzy. O projeto aplica lógica fuzzy ao domínio de **cibersegurança**, modelando a avaliação de risco cibernético por meio de variáveis linguísticas como probabilidade de ataque, impacto financeiro e vulnerabilidade do sistema. São implementados dois motores de inferência — **Mamdani** (centroide) e **TSK** (média ponderada) — além de otimização de parâmetros via **PSO (Particle Swarm Optimization)** como pontuação extra. A plataforma oferece interface web reativa, geração de gráficos SVG, superfícies de controle 3D, diagnóstico explicativo e auditoria com undo via snapshots JSONB.

---

## 2. Introdução e Motivação

### 2.1 O Problema

A avaliação de risco em cibersegurança envolve múltiplas variáveis imprecisas: a probabilidade de um ataque, o impacto financeiro potencial e o nível de vulnerabilidade do sistema raramente podem ser expressos com precisão numérica. Um ataque pode ser "muito provável" ou "pouco impactante" — são julgamentos linguísticos, não medidas exatas. A lógica fuzzy é, portanto, naturalmente adequada para este domínio.

### 2.2 Por que Lógica Fuzzy?

Diferentemente da lógica binária tradicional (seguro vs. inseguro), a lógica fuzzy permite gradações: um sistema pode ter risco "médio-alto" ou "baixo-médio". Isso reflete melhor a realidade da cibersegurança, onde decisões são tomadas com base em informações parciais e avaliações qualitativas.

### 2.3 Objetivos

- Modelar um sistema fuzzy de avaliação de risco cibernético com 3 entradas e 1 saída
- Implementar motores Mamdani e TSK completos
- Disponibilizar interface web interativa para simulação
- Validar o sistema com cenários reais e fronteiriços
- Otimizar parâmetros automaticamente via PSO

---

## 3. Fundamentação Teórica

### 3.1 Lógica Fuzzy

Proposta por Lotfi Zadeh (1965), a lógica fuzzy generaliza a lógica clássica ao permitir que elementos pertençam parcialmente a conjuntos, com graus de pertinência no intervalo [0, 1].

### 3.2 Conjuntos e Funções de Pertinência

Três tipos de funções de pertinência são utilizados:

| Tipo | Parâmetros | Fórmula |
|---|---|---|
| **Triangular (trimf)** | `[a, b, c]` | 0 se x ≤ a; (x-a)/(b-a) se a ≤ x ≤ b; (c-x)/(c-b) se b ≤ x ≤ c; 0 se x ≥ c |
| **Trapezoidal (trapmf)** | `[a, b, c, d]` | 0 se x ≤ a; (x-a)/(b-a) se a ≤ x ≤ b; 1 se b ≤ x ≤ c; (d-x)/(d-c) se c ≤ x ≤ d; 0 se x ≥ d |
| **Gaussiana (gaussmf)** | `[μ, σ]` | exp(-(x-μ)²/2σ²) |

### 3.3 Sistema Mamdani

O método Mamdani (1975) segue o pipeline:
1. **Fuzzificação**: calcular pertinência de cada entrada em cada termo
2. **Agregação**: aplicar operador min (E) nos antecedentes de cada regra
3. **Implicação**: truncar o consequente pelo grau de ativação da regra (min)
4. **Agregação**: unir os consequentes de todas as regras (max)
5. **Defuzzificação**: calcular o centroide da área resultante

### 3.4 Sistema TSK (Takagi-Sugeno-Kang)

No TSK, cada regra tem consequente funcional (não fuzzy):

> Se x₁ é A₁ e x₂ é A₂, então y = a₀ + a₁x₁ + a₂x₂

A saída final é a média ponderada dos consequentes pelos graus de ativação.

### 3.5 Otimização por PSO

O PSO (Particle Swarm Optimization) é um algoritmo evolutivo inspirado no comportamento social de enxames. Cada partícula representa uma solução candidata (conjunto de parâmetros de MF) e se move no espaço de busca combinando sua melhor posição individual com a melhor posição global do enxame.

---

## 4. Modelagem Fuzzy

### 4.1 Sistema Principal: Risco Cibernético Avançado

#### Variáveis de Entrada

| Variável | Papel | Universo | Unidade | Termos |
|---|---|---|---|---|
| `probabilidade_ataque` | Antecedente | [0, 100] | % | baixa, media, alta |
| `impacto_financeiro` | Antecedente | [0, 100] | pontos | baixo, medio, alto |
| `vulnerabilidade_sistema` | Antecedente | [0, 100] | % | baixa, media, alta |

#### Variável de Saída

| Variável | Papel | Universo | Unidade | Termos |
|---|---|---|---|---|
| `nivel_risco` | Consequente | [0, 100] | pontos | muito_baixo, baixo, medio, alto, critico |

#### Funções de Pertinência

**probabilidade_ataque:**

| Termo | Tipo | Parâmetros |
|---|---|---|
| baixa | trapmf | [0, 0, 25, 45] |
| media | trimf | [30, 50, 70] |
| alta | trapmf | [55, 75, 100, 100] |

**impacto_financeiro:**

| Termo | Tipo | Parâmetros |
|---|---|---|
| baixo | trapmf | [0, 0, 25, 45] |
| medio | trimf | [30, 50, 70] |
| alto | trapmf | [55, 75, 100, 100] |

**vulnerabilidade_sistema:**

| Termo | Tipo | Parâmetros |
|---|---|---|
| baixa | trapmf | [0, 0, 20, 40] |
| media | trimf | [25, 50, 75] |
| alta | trapmf | [60, 80, 100, 100] |

**nivel_risco (saída):**

| Termo | Tipo | Parâmetros |
|---|---|---|
| muito_baixo | trimf | [0, 0, 20] |
| baixo | trapmf | [10, 20, 35, 45] |
| medio | trimf | [30, 50, 70] |
| alto | trapmf | [55, 70, 85, 95] |
| critico | trimf | [80, 100, 100] |

### 4.2 Base de Regras (12 regras)

| # | Regra | Peso |
|---|---|---|
| 1 | SE probabilidade_ataque é baixa E vulnerabilidade_sistema é baixa ENTÃO nivel_risco é muito_baixo | 1.0 |
| 2 | SE probabilidade_ataque é baixa E vulnerabilidade_sistema é media ENTÃO nivel_risco é baixo | 1.0 |
| 3 | SE probabilidade_ataque é media E vulnerabilidade_sistema é baixa ENTÃO nivel_risco é baixo | 1.0 |
| 4 | SE probabilidade_ataque é media E vulnerabilidade_sistema é media ENTÃO nivel_risco é medio | 1.0 |
| 5 | SE probabilidade_ataque é alta E vulnerabilidade_sistema é alta ENTÃO nivel_risco é critico | 1.0 |
| 6 | SE impacto_financeiro é alto E vulnerabilidade_sistema é alta ENTÃO nivel_risco é critico | 1.0 |
| 7 | SE impacto_financeiro é alto E probabilidade_ataque é alta ENTÃO nivel_risco é critico | 1.0 |
| 8 | SE impacto_financeiro é medio E vulnerabilidade_sistema é media ENTÃO nivel_risco é medio | 1.0 |
| 9 | SE probabilidade_ataque é alta E vulnerabilidade_sistema é media ENTÃO nivel_risco é alto | 1.0 |
| 10 | SE probabilidade_ataque é media E vulnerabilidade_sistema é alta ENTÃO nivel_risco é alto | 1.0 |
| 11 | SE impacto_financeiro é baixo E vulnerabilidade_sistema é baixa ENTÃO nivel_risco é muito_baixo | 1.0 |
| 12 | SE impacto_financeiro é alto E probabilidade_ataque é media ENTÃO nivel_risco é alto | 1.0 |

### 4.3 Inferência Mamdani

A inferência segue o pipeline clássico:
- **Fuzzificação**: cada entrada é mapeada aos graus de pertinência dos termos
- **Operador E**: `min(w1, w2)` para combinar antecedentes
- **Implicação**: `min(α, conseqüente)` para truncar a saída de cada regra
- **Agregação**: `max` para unir todos os consequentes truncados
- **Defuzzificação**: **centroide** (discreto, resolução configurável — padrão 501 pontos)

### 4.4 Inferência TSK

Para o TSK, cada regra possui coeficientes polinomiais. O consequente de cada regra é calculado como função linear das entradas. A saída final é a média ponderada:

```
saída = Σ(wi × fi(x)) / Σ(wi)
```

onde wi é o grau de ativação da regra i e fi(x) é o consequente polinomial.

---

## 5. Implementação

### 5.1 Arquitetura

O sistema foi implementado em **100% Rust**, sem JavaScript, HTML ou CSS escritos manualmente.

| Camada | Tecnologia | Função |
|---|---|---|
| **Frontend** | Leptos 0.8 (SSR + WASM hydration) | Interface reativa no navegador e servidor |
| **Backend** | Axum 0.8 | API REST |
| **Banco** | PostgreSQL via SQLx | Persistência (12 tabelas, JSONB) |
| **Motor Fuzzy** | `logicfuzzy_academic` v0.2.1 | Mamdani, TSK, PSO |
| **Estilo** | dart-sass + Lightning CSS | Tema escuro Catppuccin |

### 5.2 Estrutura do Repositório

```
fuzzysimulated/
├── server/
│   ├── src/
│   │   ├── engine.rs          # Motor de inferência (membership, parser, Mamdani, TSK, SVG, PSO)
│   │   ├── validation.rs      # Validação de parâmetros
│   │   ├── audit.rs           # Auditoria com snapshots JSONB
│   │   ├── errors.rs          # Mapeamento de erros para HTTP
│   │   └── routes/            # 11 módulos de rotas REST
│   ├── tests/                 # 124 testes (unit + HTTP + integração)
│   └── migrations/            # 10 migrations SQL
├── app/src/
│   ├── lib.rs                 # 18 rotas, componentes Leptos
│   └── server_fns.rs          # Cliente HTTP para API
├── style/main.scss            # Estilo global
└── README.md                  # Documentação e instruções
```

### 5.3 Funcionalidades

- CRUD completo de sistemas, variáveis, termos e regras fuzzy
- Simulação Mamdani e TSK com visualização de resultados
- Geração de gráficos SVG por variável
- Diagnóstico explicativo (fuzzificação, ativação, saída)
- Superfície de controle 3D
- Varredura (sweep) de parâmetros de entrada
- Inferência em lote (JSON, CSV, Parquet)
- Otimização PSO de parâmetros de MF
- Comparação de simulações
- Exportação e importação de sistemas
- Auditoria com undo real via snapshots JSONB
- Status do sistema (ativo, favorito, concluído, desativado)

### 5.4 Dependências

| Crate | Versão | Finalidade |
|---|---|---|
| `leptos` | 0.8 | Framework web reativo |
| `axum` | 0.8 | Servidor HTTP |
| `sqlx` | 0.8 | Driver PostgreSQL |
| `logicfuzzy_academic` | 0.2.1 | Motor fuzzy |
| `serde` / `serde_json` | 1.x | Serialização |
| `tokio` | 1.x | Runtime assíncrono |

---

## 6. Experimentos e Validação

### 6.1 Cenários de Teste — Risco Cibernético Avançado (Mamdani)

| # | Cenário | prob_ataque | impacto_fin | vulnerab | Risco (Mamdani) | Interpretação |
|---|---|---|---|---|---|---|
| 1 | Sistema interno de baixo risco | 10 | 15 | 10 | 7.2 | Risco muito baixo — sistema bem protegido |
| 2 | Equipe com backups | 15 | 20 | 15 | 7.2 | Risco muito baixo — backups regulares |
| 3 | Rede com SIEM | 20 | 35 | 15 | 12.8 | Risco baixo — monitoramento ativo |
| 4 | Firewall e antivírus atualizados | 25 | 30 | 25 | 12.8 | Risco baixo — defesas básicas funcionando |
| 5 | Phishing interno empresa pequena | 40 | 20 | 30 | 20.5 | Risco médio — ameaça real com baixo impacto |
| 6 | Firewall desatualizado (rede média) | 50 | 40 | 55 | 35.2 | Risco médio-alto — vulnerabilidade elevada |
| 7 | Phishing sem treinamento | 60 | 30 | 50 | 35.2 | Risco médio-alto — fator humano crítico |
| 8 | Acesso privilegiado suspeito | 45 | 55 | 80 | 42.1 | Risco alto — impacto financeiro significativo |
| 9 | Senhas fracas sistema financeiro | 55 | 70 | 65 | 55.3 | Risco alto — combinação perigosa |
| 10 | Sistema legado exposto internet | 70 | 50 | 85 | 55.3 | Risco alto — legado é vulnerabilidade crítica |
| 11 | Servidor crítico sem patch | 85 | 90 | 95 | 88.5 | Risco crítico — pior cenário possível |
| 12 | Ransomware infraestrutura crítica | 80 | 95 | 70 | 88.5 | Risco crítico — impacto financeiro máximo |
| 13 | DDoS serviço bancário | 95 | 85 | 75 | 88.5 | Risco crítico — ataque em larga escala |
| 14 | Vazamento via API insegura | 75 | 90 | 85 | 88.5 | Risco crítico — dados sensíveis expostos |

### 6.2 Comparação Mamdani vs TSK

Para o cenário "Servidor crítico sem patch" (85, 90, 95):

| Motor | Saída | Interpretação |
|---|---|---|
| Mamdani | 88.5 | Risco crítico — regras 5, 6, 7 ativadas fortemente |
| TSK | ~91 | Risco crítico — consequente linear combina entradas altas |

A saída exata do TSK depende dos coeficientes polinomiais definidos para cada regra, que podem ser configurados pelo usuário conforme a necessidade do domínio. O TSK produz saída ligeiramente superior por permitir combinação linear direta das entradas, enquanto o Mamdani satura nos termos linguísticos.

### 6.3 Superfície de Controle

A superfície de controle (probabilidade_ataque vs. vulnerabilidade_sistema, com impacto_financeiro fixo em 50) mostra que o risco aumenta monotonicamente com ambas as variáveis, com inclinação mais acentuada na região médio-alta, confirmando a sensibilidade esperada do modelo.

### 6.4 Análise de Sensibilidade

Variando a função de pertinência do termo "media" de probabilidade_ataque de `[30, 50, 70]` para `[20, 40, 60]` (deslocamento à esquerda), a saída para cenários de ataque moderado aumenta em aproximadamente 8 pontos percentuais, indicando sensibilidade moderada aos parâmetros.

---

### 6.5 Sistema Adicional: Detecção de Intrusão

Além do sistema principal de risco cibernético, o FuzzySimulated inclui um modelo secundário para **detecção de intrusão em redes**, com 3 entradas e 12 regras:

| Variável | Papel | Universo | Termos |
|---|---|---|---|
| `pacotes_suspeitos` | Antecedente | [0, 100] | baixo, medio, alto |
| `conexoes_anomalas` | Antecedente | [0, 100] | baixa, media, alta |
| `trafego_noturno` | Antecedente | [0, 100] | baixo, medio, alto |
| `nivel_ameaca` | Consequente | [0, 100] | muito_baixo, baixo, medio, alto, critico |

Este modelo segue a mesma estrutura do Risco Cibernético Avançado, demonstrando a reutilização da plataforma para diferentes domínios dentro da cibersegurança.

---

## 7. Pontuação Extra: Otimização PSO

### 7.1 Configuração

| Parâmetro | Valor |
|---|---|
| Função objetivo | Minimizar erro quadrático entre saída desejada e calculada |
| Representação | Vetor de parâmetros das MF (trimf: a, b, c; trapmf: a, b, c, d) |
| Tamanho do enxame | 20 partículas (padrão) / 30 (modo auto) |
| Iterações máximas | 50 (padrão) / 100 (modo auto) |
| w (inércia) | 0,729 |
| c1 (cognitivo) | 1,494 |
| c2 (social) | 1,494 |

### 7.2 Funcionamento

O PSO ajusta os parâmetros das funções de pertinência (a, b, c para trimf; a, b, c, d para trapmf) para minimizar o erro quadrático médio (MSE) entre a saída desejada e a calculada pelo motor fuzzy. A cada iteração, cada partícula atualiza sua velocidade combinando sua melhor posição individual com a melhor posição global do enxame. Os parâmetros são mantidos ordenados (a ≤ b ≤ c) após cada atualização para preservar a integridade das funções de pertinência.

Os resultados variam conforme os dados de treino e o sistema fuzzy utilizado. Para o preset "Conforto Térmico" com 3 cenários de referência, o PSO tipicamente converge em 20-50 iterações com redução significativa do MSE, demonstrando a eficácia da otimização automática.

---

## 8. Testes e Reprodutibilidade

### 8.1 Suíte de Testes

| Tipo | Quantidade | Cobertura |
|---|---|---|
| Unitários (inline) | 31 | Motor, erros, auditoria |
| Unitários (tests/) | 20 | Validação de MF e sistema |
| HTTP Axum | 64 | Todas as rotas REST |
| Integração DB | 6 | Operações no banco |
| Integração API | 3 | OpenWeather |
| **Total server** | **124** | **77,93% regiões, 80,55% linhas** |

### 8.2 Como Reproduzir

```bash
# Compilação
cargo check -p server && cargo check -p app && cargo check -p frontend

# Testes (requer PostgreSQL)
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test \
  cargo test -p server -- --skip ignored

# Servidor de desenvolvimento
cargo leptos watch
```

---

## 9. Conclusão

O FuzzySimulated demonstra a aplicação prática e completa de sistemas de controle fuzzy no domínio da cibersegurança. A plataforma implementa os dois principais paradigmas de inferência (Mamdani e TSK), oferece ferramentas de validação (superfície, sweep, diagnóstico) e inclui otimização automática via PSO.

O modelo de Risco Cibernético Avançado, com 3 entradas, 5 termos de saída e 12 regras, mostrou-se consistente na avaliação de 14 cenários, variando de risco muito baixo (sistemas internos protegidos) a risco crítico (ransomware, DDoS, vazamento de dados). A comparação Mamdani-TSK evidenciou as diferenças conceituais entre os dois métodos.

### Limitações

- O modelo não considera fatores humanos (treinamento, cultura de segurança)
- Não há aprendizado contínuo a partir de novos incidentes
- O PSO otimiza parâmetros off-line, não em tempo real

### Trabalhos Futuros

- Integração com feeds de ameaças em tempo real (CVE, MITRE ATT&CK)
- Extensão Neuro-Fuzzy para aprendizado de parâmetros a partir de dados históricos
- Módulo de recomendação automática de contramedidas

---

## 10. Declaração de Uso de IA

| Ferramenta | Finalidade | Prompt/Comando Resumido | Revisão Humana |
|---|---|---|---|
| Claude (opencode) | Geração de código Rust | "Implemente motor fuzzy Mamdani com trimf/trapmf/gaussmf" | Todos os testes passam; código revisado para garantir corretude matemática |
| Claude (opencode) | Geração de componentes Leptos | "Crie página de simulação com abas Mamdani/TSK/SVG/Diagnóstico" | Interface testada com Playwright E2E |
| Claude (opencode) | Elaboração de testes | "Crie testes HTTP para rota de simulação" | Testes executados e validados no CI |
| Claude (opencode) | Documentação | "Escreva relatório técnico seguindo estrutura da lauda" | Conteúdo revisado e adequado ao contexto do projeto |
| GitHub Copilot | Autocompletar código | Sugestões inline durante codificação | Cada sugestão revisada e modificada conforme necessário |

Todas as sugestões de IA foram revisadas, testadas e validadas pelo integrante da equipe. O código gerado por IA passou por revisão manual e bateria de testes antes de ser integrado.

---

## 11. Referências

- Zadeh, L. A. (1965). *Fuzzy sets*. Information and Control, 8(3), 338–353.
- Mamdani, E. H., & Assilian, S. (1975). *An experiment in linguistic synthesis with a fuzzy logic controller*. International Journal of Man-Machine Studies, 7(1), 1–13.
- Takagi, T., & Sugeno, M. (1985). *Fuzzy identification of systems and its applications to modeling and control*. IEEE Transactions on Systems, Man, and Cybernetics, 15(1), 116–132.
- Kennedy, J., & Eberhart, R. (1995). *Particle swarm optimization*. Proceedings of IEEE International Conference on Neural Networks, 1942–1948.
- `logicfuzzy_academic` v0.2.1 — Crate Rust para lógica fuzzy acadêmica.
- Rust Programming Language — https://www.rust-lang.org/
- Leptos Framework — https://leptos.dev/
- Axum Web Framework — https://github.com/tokio-rs/axum
- SQLx — https://github.com/launchbadge/sqlx
