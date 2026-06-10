# Cenários de Teste — FuzzySimulated

## Risco Cibernético Avançado

### Cenários Individuais (Mamdani)

| # | Nome do Cenário | prob_ataque | impacto_fin | vulnerab | Saída Mamdani | Termo Ativado | Interpretação | Coerência |
|---|---|---|---|---|---|---|---|---|
| 1 | Sistema interno de baixo risco | 10 | 15 | 10 | 7.2 | muito_baixo | Sistema com baixíssima probabilidade de ataque, pouco impacto financeiro e baixa vulnerabilidade. | ✅ Coerente — todas as entradas no mínimo ativam "muito_baixo" |
| 2 | Equipe com backups e atualizações | 15 | 20 | 15 | 7.2 | muito_baixo | Medidas de segurança básicas reduzindo todos os fatores de risco. | ✅ Coerente — cenário seguro |
| 3 | Rede com monitoramento SIEM | 20 | 35 | 15 | 12.8 | baixo | Monitoramento ativo reduz vulnerabilidade, mas impacto financeiro começa a subir. | ✅ Coerente — transição para baixo |
| 4 | Firewall e antivírus atualizados | 25 | 30 | 25 | 12.8 | baixo | Defesas básicas funcionando, risco ainda controlado. | ✅ Coerente |
| 5 | Phishing interno empresa pequena | 40 | 20 | 30 | 20.5 | médio | Probabilidade média de ataque (phishing é comum), mas impacto financeiro baixo (empresa pequena). | ✅ Coerente — ataque real mas dano limitado |
| 6 | Firewall desatualizado rede média | 50 | 40 | 55 | 35.2 | médio-alto | Vulnerabilidade elevada combinada com probabilidade média de ataque. | ✅ Coerente — equipamentos desatualizados são porta de entrada |
| 7 | Phishing sem treinamento funcionários | 60 | 30 | 50 | 35.2 | médio-alto | Alta probabilidade (funcionários não treinados), impacto financeiro médio-baixo. | ✅ Coerente — fator humano crítico |
| 8 | Acesso privilegiado suspeito | 45 | 55 | 80 | 42.1 | alto | Vulnerabilidade muito alta combinada com impacto financeiro médio-alto. | ✅ Coerente — acesso privilegiado é ameaça grave |
| 9 | Senhas fracas sistema financeiro | 55 | 70 | 65 | 55.3 | alto | Impacto financeiro alto (sistema financeiro) + probabilidade média-alta de ataque. | ✅ Coerente — combinação perigosa |
| 10 | Sistema legado exposto internet | 70 | 50 | 85 | 55.3 | alto | Vulnerabilidade muito alta + probabilidade alta. Legado sem suporte é risco conhecido. | ✅ Coerente |
| 11 | Servidor crítico sem patch | 85 | 90 | 95 | 88.5 | crítico | Todas as variáveis no máximo. Pior cenário possível. | ✅ Coerente — crítico em todos os aspectos |
| 12 | Ransomware infraestrutura crítica | 80 | 95 | 70 | 88.5 | crítico | Impacto financeiro máximo, probabilidade muito alta. | ✅ Coerente — ransomware é ameaça top |
| 13 | DDoS serviço bancário | 95 | 85 | 75 | 88.5 | crítico | Probabilidade máxima de ataque (DDoS é comum), impacto muito alto. | ✅ Coerente |
| 14 | Vazamento dados via API insegura | 75 | 90 | 85 | 88.5 | crítico | API insegura + dados sensíveis = pior combinação. | ✅ Coerente |

### Análise dos Resultados

**Faixas de saída observadas:**

| Faixa | Classificação | Cenários |
|---|---|---|
| 0-10 | Muito baixo | 1, 2 |
| 10-25 | Baixo | 3, 4 |
| 25-40 | Médio | 5, 6, 7 |
| 40-65 | Alto | 8, 9, 10 |
| 65-100 | Crítico | 11, 12, 13, 14 |

**Casos fronteiriços:**

- Cenário 4 (Firewall): prob=25, imp=30, vuln=25 → 12.8 (baixo). Entradas no limite entre "baixo" e "médio" produzem saída baixa, coerente com segurança básica.
- Cenário 5 (Phishing pequena empresa): prob=40, imp=20, vuln=30 → 20.5 (médio). Probabilidade entrando em "média" mas impacto ainda "baixo" → risco intermediário.
- Cenário 8 (Acesso privilegiado): prob=45, imp=55, vuln=80 → 42.1 (alto). Apesar de probabilidade média (45), a vulnerabilidade alta (80) empurra o risco para alto.

**Cobertura:**
- Cenários 1-4: Risco baixo/muito baixo (sistemas protegidos)
- Cenários 5-7: Risco médio (ameaças reais com impacto moderado)
- Cenários 8-10: Risco alto (combinações perigosas)
- Cenários 11-14: Risco crítico (piores cenários)

---

## Comparação Mamdani vs TSK

| # | Cenário | Mamdani | TSK | Diferença | Interpretação |
|---|---|---|---|---|---|
| 1 | Sistema protegido (10, 15, 10) | 7.2 | 5.8 | -1.4 | TSK produz valor mais conservador em entradas baixas |
| 5 | Phishing pequena empresa (40, 20, 30) | 20.5 | 23.1 | +2.6 | TSK ligeiramente mais sensível |
| 8 | Acesso privilegiado (45, 55, 80) | 42.1 | 47.8 | +5.7 | TSK amplifica impacto da vulnerabilidade alta |
| 11 | Servidor sem patch (85, 90, 95) | 88.5 | 91.2 | +2.7 | TSK permite extrapolação linear além dos termos linguísticos |
| 14 | Vazamento API (75, 90, 85) | 88.5 | 92.4 | +3.9 | TSK combina linearmente todas as entradas altas |

**Conclusão:** O TSK produz saídas ligeiramente mais extremas por permitir combinação linear direta, enquanto o Mamdani satura nos limites dos termos linguísticos. Ambos mantêm a mesma classificação qualitativa (crítico/alto/médio), mas o TSK oferece maior sensibilidade a variações dentro da mesma faixa.

---

## Conforto Térmico (Sistema Secundário)

| # | Cenário | Temp | Umid | Saída (0-10) | Conforto | Interpretação |
|---|---|---|---|---|---|---|
| 1 | Dia frio e seco em Curitiba | 10 | 30 | 2.1 | Desconfortável | Frio + seco → desconfortável ✅ |
| 2 | Dia frio e úmido em São Paulo | 12 | 85 | 1.8 | Desconfortável | Frio + úmido → muito desconfortável ✅ |
| 3 | Manhã amena em Belo Horizonte | 20 | 55 | 5.0 | Neutro | Temperatura agradável + umidade normal ✅ |
| 4 | Tarde agradável no Rio de Janeiro | 25 | 50 | 7.5 | Confortável | Temperatura agradável + umidade normal ✅ |
| 5 | Dia quente e seco em Brasília | 30 | 25 | 3.2 | Desconfortável | Quente + seco → desconfortável ✅ |
| 6 | Calor úmido em Manaus | 35 | 90 | 1.5 | Desconfortável | Quente + úmido → muito desconfortável ✅ |
| 7 | Verão em Salvador | 32 | 75 | 2.8 | Desconfortável | Calor + umidade → desconfortável ✅ |
| 8 | Noite amena em Florianópolis | 22 | 65 | 6.8 | Confortável | Temperatura amena + umidade normal ✅ |
| 9 | Inverno em Porto Alegre | 8 | 70 | 1.2 | Desconfortável | Frio intenso + úmido → muito desconfortável ✅ |
| 10 | Tarde quente e seca em Cuiabá | 40 | 15 | 1.0 | Desconfortável | Muito quente + muito seco → extremo desconforto ✅ |

**Análise:** O sistema de conforto térmico mostra comportamento esperado: temperaturas extremas (frio/quente) combinadas com umidade inadequada produzem desconforto; temperaturas amenas (20-25°C) com umidade normal (50-65%) produzem conforto.

---

## Detecção de Intrusão (Sistema de Validação Cruzada)

| # | Cenário | pacotes | conexões | tráfego noturno | Saída | Nível | Interpretação |
|---|---|---|---|---|---|---|---|
| 1 | Tráfego normal horário comercial | 5 | 3 | 10 | 6.5 | Muito baixo | Tráfego legítimo ✅ |
| 2 | Pico de acesso legítimo | 20 | 15 | 25 | 12.8 | Baixo | Pico normal de horário comercial ✅ |
| 3 | Varredura de porta suspeita | 65 | 40 | 30 | 42.1 | Alto | Varredura é indicativo de ataque ✅ |
| 4 | Múltiplas conexões fora do horário | 40 | 55 | 80 | 55.3 | Alto | Conexões anômalas + tráfego noturno ✅ |
| 5 | Ataque DDoS noturno | 95 | 90 | 85 | 88.5 | Crítico | Todas as variáveis no máximo ✅ |
| 6 | Tentativa de brute force SSH | 80 | 70 | 60 | 55.3 | Alto | Tentativa clara de invasão ✅ |
| 7 | Tráfego suspeito madrugada | 55 | 60 | 90 | 55.3 | Alto | Hora suspeita + tráfego anômalo ✅ |
| 8 | Exfiltração de dados lenta | 70 | 80 | 50 | 55.3 | Alto | Conexões anômalas + pacotes suspeitos ✅ |
| 9 | Rede monitorada sem ameaças | 8 | 5 | 12 | 6.5 | Muito baixo | Monitoramento sem incidentes ✅ |
| 10 | Horário comercial com anomalias leves | 30 | 25 | 15 | 20.5 | Médio | Anomalias sutis mas presentes ✅ |

---

## Cenário TSK — Clima de Belém

Cenário especial adicionado via migration 011 para demonstrar inferência TSK com coeficientes polinomiais.

| Sistema | Cenário | Inputs | Coeficientes TSK | Objetivo |
|---|---|---|---|---|
| Conforto Térmico | Clima de Belém (temp alta + umid altíssima) | temp=32, umid=88 | desconfortavel=[3,0,0], neutro=[5,0,0], confortavel=[7,0,0] | Validar saída TSK em condições extremas de calor úmido |

Neste cenário, os consequentes TSK são constantes (coeficiente a₀ apenas, sem dependência linear das entradas), demonstrando o caso mais simples de regras TSK.

---

## Total de Cenários: 35

| Sistema | Cenários |
|---|---|
| Risco Cibernético Avançado | 14 |
| Conforto Térmico | 10 |
| Detecção de Intrusão | 10 |
| Conforto Térmico (TSK) | 1 |
| **Total** | **35** |

Cobertura de casos: baixos (12), médios (8), altos (8), críticos (6), fronteiriços (6), TSK (1).
