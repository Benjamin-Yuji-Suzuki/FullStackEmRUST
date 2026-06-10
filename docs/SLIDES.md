# Slides — FuzzySimulated

> Apresentação para defesa — Inteligência Artificial e Computacional (0700M8)
> Prof. Daniel Leal Souza — CESUPA 01/2026
> Benjamin Yuji Suzuki

---

## Slide 1 — Capa

**FuzzySimulated — Plataforma de Inferência Fuzzy**

Avaliação de Risco em Cibersegurança com Lógica Fuzzy

CESUPA — Inteligência Artificial e Computacional
Benjamin Yuji Suzuki
Opção B (Produto) + C-B (TSK) + Extra PSO

---

## Slide 2 — O Problema

**Avaliação de Risco em Cibersegurança**

- Como classificar o risco de um ataque cibernético?
- Variáveis envolvidas são imprecisas e qualitativas
  - Probabilidade do ataque: "alta", "média", "baixa"
  - Impacto financeiro: "catastrófico", "moderado"
  - Vulnerabilidade: "crítica", "baixa"
- Lógica binária não captura gradações

---

## Slide 3 — Por que Lógica Fuzzy?

| Lógica Clássica | Lógica Fuzzy |
|---|---|
| Seguro **ou** Inseguro | Grau de risco: 0% a 100% |
| Corte abrupto | Transição suave |
| Ignora incerteza | Modela imprecisão |

"A lógica fuzzy permite que um sistema seja **parcialmente seguro** e **parcialmente vulnerável** ao mesmo tempo."

---

## Slide 4 — Modelagem Fuzzy

**Variáveis do Sistema Principal (Risco Cibernético Avançado)**

| Entradas | Saída |
|---|---|
| probabilidade_ataque [0-100] | nivel_risco [0-100] |
| impacto_financeiro [0-100] | → muito_baixo, baixo, medio, alto, critico |
| vulnerabilidade_sistema [0-100] | |

3 termos por entrada (baixa/baixo, media/medio, alta/alto)
5 termos na saída

---

## Slide 5 — Funções de Pertinência

**Exemplo: probabilidade_ataque**

```
  1.0 ┤▄██▄▄▄▄▄▄▄▄▄▄▄▄▄───────▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄██▄
      │  ██  baixa      │  media  │     alta    ██
  0.0 ┼─────────────────┼─────────┼───────────────→
      0                30        50             100
```

- **trimf** (triangular): `[a, b, c]`
- **trapmf** (trapezoidal): `[a, b, c, d]`
- **gaussmf** (gaussiana): `[μ, σ]`

---

## Slide 6 — Base de Regras (12 regras)

**Exemplos:**

| Regra | Condição | Conclusão |
|---|---|---|
| R5 | SE prob_alta E vuln_alta | ENTÃO risco = crítico |
| R9 | SE prob_alta E vuln_media | ENTÃO risco = alto |
| R1 | SE prob_baixa E vuln_baixa | ENTÃO risco = muito_baixo |

---

## Slide 7 — Inferência Mamdani

**Pipeline:**

```
Entradas → Fuzzificação → Agregação (min) → Implicação (min) 
         → Agregação (max) → Defuzzificação (centroide) → Saída
```

- Operador E = `min`
- Defuzzificação = centroide discreto (1000 pontos)

---

## Slide 8 — Inferência TSK (Opção C)

**Regra TSK:**
```
SE probabilidade_ataque é alta E vulnerabilidade_sistema é alta
ENTÃO risco = 0.3×prob + 0.3×imp + 0.4×vuln
```

**Saída final:**
```
Σ(wi × fi(x)) / Σ(wi)
```

**Comparação Mamdani vs TSK:**

| Aspecto | Mamdani | TSK |
|---|---|---|
| Interpretabilidade | Alta (termos linguísticos) | Média (funções matemáticas) |
| Continuidade | Suave | Suave (polinomial) |
| Ajuste | Manual | Por otimização |

---

## Slide 9 — Pontuação Extra: PSO

**Particle Swarm Optimization**

- Enxame de 30 partículas
- 100 iterações máximas
- Otimiza parâmetros das funções de pertinência

**Resultado:**
- Erro (MSE) reduziu de 0.035 → 0.012 (**66% de melhoria**)

---

## Slide 10 — Implementação

**Stack 100% Rust**

```
Frontend: Leptos 0.8 (SSR + WASM)
Backend : Axum 0.8 (REST API)
Banco   : PostgreSQL + SQLx
Motor   : logicfuzzy_academic v0.2.1
```

**Funcionalidades:**
- CRUD de sistemas, variáveis, termos, regras
- Simulação, sweep, superfície 3D, SVG, diagnóstico
- Batch (JSON, CSV, Parquet)
- Auditoria com undo (JSONB snapshots)
- 124 testes automatizados (80% cobertura)

---

## Slide 11 — Demonstração

**Fluxo da aplicação:**

1. Dashboard → seleciona "Risco Cibernético Avançado"
2. Simulador → insere valores (ex: prob=85, impacto=90, vuln=95)
3. Resultado Mamdani: **88.5 — Risco Crítico**
4. Diagnóstico mostra termos ativados e regras disparadas
5. SVG exibe funções de pertinência
6. TSK: **91.2 — Risco Crítico**
7. PSO: otimiza parâmetros automaticamente

> Demonstração ao vivo no navegador.

---

## Slide 12 — Cenários de Teste

**14 cenários validados:**

| Cenário | Risco | Interpretação |
|---|---|---|
| Sistema interno protegido | 7.2 | Muito baixo ✅ |
| Phishing sem treinamento | 35.2 | Médio-alto ✅ |
| Ransomware | 88.5 | Crítico ✅ |
| Servidor sem patch | 88.5 | Crítico ✅ |

Cobertura: casos baixos, médios, altos, críticos, fronteiriços.

---

## Slide 13 — Superfície de Controle

**probabilidade_ataque × vulnerabilidade_sistema → nivel_risco**

- Eixo X: probabilidade_ataque (0-100)
- Eixo Y: vulnerabilidade_sistema (0-100)
- Eixo Z: nivel_risco (0-100)

Superfície cresce monotonicamente com ambas as variáveis.
Inclinação mais acentuada na região médio-alta.

---

## Slide 14 — Conclusão

**O que foi entregue:**
- ✅ Plataforma funcional 100% Rust
- ✅ Motores Mamdani + TSK completos
- ✅ 12 regras, 3 entradas, 5 termos de saída
- ✅ 14 cenários de teste com análise
- ✅ Otimização PSO (pontuação extra)
- ✅ 124 testes, 80% de cobertura
- ✅ GitHub organizado com README e instruções

**Valor acadêmico:** Demonstra aplicação real de lógica fuzzy em domínio crítico, com implementação completa e validação experimental.

---

## Slide 15 — Perguntas?

**Obrigado!**

Repositório: [github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST](https://github.com/Benjamin-Yuji-Suzuki/FullStackEmRUST)
