# Roteiro de Apresentação — Sprint 2 (5 min)

---

## 0. Abertura (30s)

> "Bom dia. Esta é a Sprint 2 do FuzzySimulated — plataforma web full-stack em Rust
> para construção e simulação de Sistemas de Inferência Fuzzy."
> "Os requisitos: telas CRUD, back-end com lógica de negócio, persistência,
> API externa, testes unitários e documentação."

---

## 1. Testes — Terminal (30s)

```bash
cargo test -p server -- --skip ignored
```

Mostra os 16 testes passando. Diz:

> "16 testes unitários validando regras de negócio:
> nome do sistema, método de defuzzificação, parâmetros das funções
> de pertinência trimf, trapmf e gaussmf."
> "6 testes de integração esboçados que rodam com banco separado."

---

## 2. Dashboard — Navegador (1min)

Abra `http://localhost:3000`.

Mostra os cards de sistemas. Diz:

> "CRUD completo de sistemas fuzzy:
> criar, listar, editar, excluir, duplicar, exportar e importar JSON."
> "Persistência em PostgreSQL com 7 tabelas, índices e ON DELETE CASCADE."

Ações:
- Clique num sistema existente para entrar
- Ou clique "Novo Sistema", preencha nome, confirme

---

## 3. Editor de Variáveis (1min)

Navegue para `/vars?=SISTEMA_ID`.

Diz:

> "CRUD de variáveis e termos linguísticos.
> Cada variável tem papel (antecedente ou consequente), universo de discurso
> e termos com funções de pertinência trimf, trapmf ou gaussmf."

Ações:
- Mostre a lista de variáveis
- Clique "Adicionar Variável de Entrada", preencha, confirme
- Mostre o termo criado

---

## 4. Editor de Regras (30s)

Navegue para `/rules?=SISTEMA_ID`.

Diz:

> "CRUD de regras no formato SE...ENTÃO com peso.
> O back-end valida regras duplicadas e consequente obrigatório."

Ações:
- Mostre a lista de regras
- Clique "Nova Regra", digite `SE temperatura é frio ENTÃO conforto é desconfortavel`, confirme

---

## 5. Simulador (1min)

Navegue para `/sim?=SISTEMA_ID`.

Diz:

> "Tela de simulação Mamdani com sliders para cada variável antecedente,
> campo de busca climática via OpenWeather API e execução da inferência."

Ações:
- Arraste o slider da temperatura
- Digite "Belem" no campo de clima, clique "Buscar Clima"
- Mostre os valores preenchidos (temp, umidade)
- Clique "Executar Simulação"
- Mostre o resultado defuzzificado

---

## 6. Encerramento (30s)

Diz:

> "Sprint 2 completa:
> - 3 telas CRUD (mínimo 2 exigido)
> - Back-end REST com 24 endpoints, validações e auditoria
> - PostgreSQL com migrations automáticas e seed data
> - OpenWeather API integrada
> - 16 testes unitários ✅ / 43 casos documentados
> - Proximo: Sprint 3 — inferência em lote, análise e otimização PSO"

---
