# FUZZY_MODEL.md — FuzzySimulated

> Especificação completa dos modelos fuzzy de demonstração: Mamdani, TSK e otimização por PSO.

---

## Domínio

O sistema-padrão avalia o **risco crítico de incidentes de cibersegurança** cruzando duas dimensões de impacto:

- **Impacto Financeiro** — magnitude da perda econômica direta (evasão de caixa, multas, custos de remediação).
- **Impacto de Mercado** — magnitude dos danos à reputação e percepção pública (perda de clientes, cobertura negativa, queda de confiança).

A saída do sistema, **Risco Crítico**, orienta a severidade da resposta ao incidente.

> A base de regras e os parâmetros das funções de pertinência são totalmente configuráveis pelo usuário via interface. O modelo abaixo é o ponto de partida de demonstração.

---

## Variáveis e Universos de Discurso

| Variável | Papel | Universo | Unidade |
|---|---|---|---|
| `impacto_financeiro` | Antecedente | [0, 100] | Escala normalizada |
| `impacto_mercado` | Antecedente | [0, 100] | Escala normalizada |
| `risco_critico` | Consequente | [0, 100] | Escala normalizada |

---

## Funções de Pertinência

### `impacto_financeiro`

| Termo | Tipo | Parâmetros | Descrição |
|---|---|---|---|
| Baixo | `trapmf` | [0, 0, 20, 40] | Impacto absorvível sem impacto significativo no fluxo de caixa |
| Médio | `trimf` | [25, 50, 75] | Prejuízo relevante, mas dentro da capacidade de resposta operacional |
| Alto | `trapmf` | [60, 80, 100, 100] | Evasão severa de caixa com risco de continuidade |

```
μ
1 ┤████▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Baixo
  ┤░░░░░░░░░░░░░░▓████▓░░░░░░░░░░░░░░░░░░░░░░░░░  Médio
  ┤░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓████████░░  Alto
0 ┼──────────────────────────────────────────────
  0        25        50        75       100
```

### `impacto_mercado`

| Termo | Tipo | Parâmetros | Descrição |
|---|---|---|---|
| Baixo | `trapmf` | [0, 0, 20, 40] | Incidente sem repercussão pública relevante |
| Médio | `trimf` | [25, 50, 75] | Cobertura negativa contida; reputação parcialmente afetada |
| Alto | `trapmf` | [60, 80, 100, 100] | Crise de imagem ampla; perda expressiva de confiança |

### `risco_critico` (consequente)

| Termo | Tipo | Parâmetros | Descrição |
|---|---|---|---|
| Tolerável | `trapmf` | [0, 0, 10, 25] | Incidente isolado; monitoramento padrão suficiente |
| Moderado | `trimf` | [15, 30, 50] | Requer atenção e ações de contenção pontuais |
| Alto | `trimf` | [40, 60, 75] | Acionamento de equipe de resposta a incidentes |
| Crítico | `trimf` | [65, 78, 90] | Acionamento imediato do comitê de crise |
| Severo | `trapmf` | [80, 92, 100, 100] | Resposta emergencial; colapso simultâneo de caixa e reputação |

---

## Base de Regras

A base de regras padrão possui 9 regras cobrindo todas as combinações dos termos dos antecedentes. O operador de conjunção é **AND (mínimo)**. A implicação utiliza o método **mínimo** (Mamdani). Todos os pesos são `1.0` salvo indicação em contrário.

| # | Se `impacto_financeiro` é… | E `impacto_mercado` é… | Então `risco_critico` é… | Observação |
|---|---|---|---|---|
| R01 | Baixo | Baixo | Tolerável | Incidente isolado; monitoramento padrão é suficiente |
| R02 | Baixo | Médio | Moderado | Operação normal, mas requer ação de relações públicas |
| R03 | Baixo | Alto | Crítico | Risco alto de perda de clientes, mesmo sem grande custo direto |
| R04 | Médio | Baixo | Moderado | Custo operacional absorvível, sem alarde público |
| R05 | Médio | Médio | Alto | Prejuízo considerável e danos à reputação simultâneos |
| R06 | Médio | Alto | Crítico | Necessita acionamento imediato do comitê de crise |
| R07 | Alto | Baixo | Alto | Grande evasão de caixa, mesmo que o mercado não tenha precificado |
| R08 | Alto | Médio | Crítico | Perdas financeiras severas vazando para a percepção pública |
| R09 | Alto | Alto | Severo | Colapso simultâneo de caixa e reputação — resposta emergencial |

---

## Inferência Mamdani — Pipeline

```
Inputs crips
  impacto_financeiro = x₁
  impacto_mercado    = x₂
        ↓
1. Fuzzificação
   μ_Baixo(x₁), μ_Médio(x₁), μ_Alto(x₁)
   μ_Baixo(x₂), μ_Médio(x₂), μ_Alto(x₂)
        ↓
2. Avaliação de regras (AND = mínimo)
   α_Ri = min(μ_antecedente1(x₁), μ_antecedente2(x₂))
        ↓
3. Implicação (corte mínimo no consequente)
   μ'_consequente_Ri(y) = min(α_Ri, μ_consequente(y))
        ↓
4. Agregação (máximo entre todos os consequentes)
   μ_agregado(y) = max(μ'_R01(y), ..., μ'_R09(y))
        ↓
5. Defuzzificação (centroide — padrão)
   y* = ∫ y · μ_agregado(y) dy / ∫ μ_agregado(y) dy
        ↓
Output crisp: risco_critico ∈ [0, 100]
```

---

## Cenários de Teste

Os cenários abaixo cobrem os extremos e casos intermediários da base de regras, úteis para validação manual e testes unitários.

| Cenário | `impacto_financeiro` | `impacto_mercado` | Saída esperada (aprox.) | Classificação |
|---|---|---|---|---|
| Incidente mínimo | 5 | 5 | ~10 | Tolerável |
| Custo sem repercussão | 70 | 10 | ~58 | Alto |
| Reputação sem custo | 10 | 85 | ~75 | Crítico |
| Crise moderada | 50 | 50 | ~60 | Alto |
| Crise total | 90 | 90 | ~94 | Severo |
| Custo médio + mercado alto | 50 | 85 | ~78 | Crítico |

> Os valores de saída são aproximações baseadas na defuzzificação por centroide. Variações de ±3 pontos são esperadas dependendo da resolução do universo (padrão: 501 pontos).

---

---

## Modelo TSK (Takagi-Sugeno-Kang)

> A modalidade **Opção C-B** exige a implementação de TSK como motor de inferência alternativo, com consequentes polinomiais.

### Diferenças para Mamdani

| Característica | Mamdani | TSK |
|---|---|---|
| Consequente | Conjunto fuzzy (ex: "Risco é Alto") | Função polinomial (ex: `y = a₀ + a₁x₁ + a₂x₂`) |
| Interpretabilidade | Alta (regras legíveis) | Média (consequentes matemáticos) |
| Saída | Defuzzificação (centroide) | Média ponderada dos consequentes |
| Continuidade | Suave | Suave (com coeficientes contínuos) |
| Custo computacional | Maior (defuzzificação) | Menor (cálculo direto) |

### Pipeline TSK

```
Inputs crisp: x₁, x₂
     ↓
1. Fuzzificação (mesma do Mamdani)
   μ_Baixo(x₁), μ_Médio(x₁), μ_Alto(x₁)
     ↓
2. Grau de ativação de cada regra
   α_Ri = min(μ_ant1(x₁), μ_ant2(x₂))   [AND = mínimo]
     ↓
3. Consequente polinomial
   y_Ri = a₀_i + a₁_i·x₁ + a₂_i·x₂
     ↓
4. Média ponderada
   y* = Σ(α_Ri · y_Ri) / Σ(α_Ri)
     ↓
Output crisp
```

### Exemplo de Regra TSK

```
SE impacto_financeiro É Alto E impacto_mercado É Alto
ENTÃO risco_critico ⇒ [80.0, 0.3, 0.5]
→ y = 80.0 + 0.3·x₁ + 0.5·x₂
```

### Base de Regras TSK (sistema de demonstração)

A mesma base de 9 regras do Mamdani, porém com consequentes lineares:

| # | Se `impacto_financeiro` é… | E `impacto_mercado` é… | Então (coeficientes [bias, c₁, c₂]) |
|---|---|---|---|
| R01 | Baixo | Baixo | [5.0, 0.1, 0.1] |
| R02 | Baixo | Médio | [15.0, 0.2, 0.3] |
| R03 | Baixo | Alto | [30.0, 0.1, 0.5] |
| R04 | Médio | Baixo | [20.0, 0.4, 0.2] |
| R05 | Médio | Médio | [35.0, 0.3, 0.3] |
| R06 | Médio | Alto | [50.0, 0.2, 0.4] |
| R07 | Alto | Baixo | [40.0, 0.5, 0.1] |
| R08 | Alto | Médio | [55.0, 0.4, 0.3] |
| R09 | Alto | Alto | [70.0, 0.3, 0.5] |

---

## Otimização por PSO (Particle Swarm Optimization)

> Pontuação extra: otimização de hiperparâmetros das funções de pertinência ou pesos das regras usando o algoritmo PSO implementado no `logicfuzzy-academic`.

### Funcionamento

O PSO ajusta automaticamente os parâmetros das MF (ex: vértices das trimf/trapmf, mean/sigma das gaussmf) para minimizar uma função objetivo (ex: erro quadrático médio entre saída esperada e obtida).

### Parâmetros Configuráveis

| Parâmetro | Descrição | Padrão |
|---|---|---|
| `population_size` | Número de partículas no enxame | 30 |
| `max_iterations` | Número máximo de iterações | 200 |
| `inertia_weight` | Peso da inércia (velocidade anterior) | 0.729 |
| `cognitive_coefficient` | Coeficiente cognitivo (melhor individual) | 1.494 |
| `social_coefficient` | Coeficiente social (melhor global) | 1.494 |
| `bounds` | Limites de busca por parâmetro | Definido por MF |
| `tolerance` | Tolerância para convergência | 1e-8 |
| `patience` | Iterações sem melhora para early stopping | 50 |

### O que pode ser otimizado

1. **Parâmetros das MF**: vértices de trimf/trapmf, mean/sigma de gaussmf
2. **Pesos das regras**: weight de cada regra (0.0 a 1.0)
3. **Coeficientes TSK**: bias e coeficientes dos consequentes polinomiais

### Exemplo de Uso (via `logicfuzzy-academic`)

```rust
let config = PsoConfig {
    population_size: 30,
    max_iterations: 200,
    bounds: vec![(0.0, 100.0); 6],  // 6 parâmetros para otimizar
    seed: Some(42),
    ..Default::default()
};
let mut optimizer = PsoOptimizer::new(config);
let (best_params, best_fitness, _) = optimizer.optimize(|params| {
    // função objetivo: MSE entre saída esperada e obtida
    mse(params)
});
```

---

## Configuração pelo Usuário

O sistema-padrão acima é apenas um ponto de partida. Pela interface do FuzzySimulated o usuário pode criar sistemas fuzzy para domínios completamente distintos, definindo:

- **Variáveis e universos** — nome, papel (antecedente/consequente), `universe_min` e `universe_max` conforme a escala real dos dados.
- **Funções de pertinência** — tipo (`trimf`, `trapmf`, `gaussmf`) e parâmetros de cada termo linguístico.
- **Base de regras** — adicionar, remover ou reordenar regras; ajustar pesos individuais.
- **Método de defuzzificação** — centroide (padrão), bissetor ou outros métodos implementados no `logicfuzzy_academic`.
- **Renomeação de colunas** — ao carregar um Parquet, colunas com caracteres especiais ou nomes abreviados podem ser renomeadas antes do mapeamento para as variáveis do sistema.
