#!/usr/bin/env bash
# Gera camada Gold do Fuzzy: regras + parametros PSO + previsoes
set -euo pipefail

DB_URL="postgres://ben:1234@localhost/fuzzysimulated"
API="http://127.0.0.1:3000"
GOLD_DIR="camada_gold_fuzzy"
SYS_NAME="Risco Cibernetico"

mkdir -p "$GOLD_DIR"

# Descobre ID do sistema
SYS_ID=$(psql "$DB_URL" -tA -c "SELECT id FROM fuzzy_systems WHERE name='$SYS_NAME' LIMIT 1")
echo "Sistema: $SYS_NAME ($SYS_ID)"

# 1. Exporta regras + variaveis + termos como JSON (camada Gold - regras)
echo ">>> Exportando regras..."
psql "$DB_URL" -tA -c "
SELECT jsonb_pretty(jsonb_build_object(
  'sistema', '$SYS_NAME',
  'data_geracao', now()::text,
  'variaveis', (
    SELECT jsonb_agg(jsonb_build_object(
      'nome', v.name,
      'papel', v.role,
      'universo_min', v.universe_min,
      'universo_max', v.universe_max,
      'termos', (
        SELECT jsonb_agg(jsonb_build_object(
          'rotulo', t.label,
          'tipo_mf', t.mf_type,
          'parametros', t.params
        ) ORDER BY t.label)
        FROM fuzzy_terms t WHERE t.variable_id = v.id
      )
    ) ORDER BY v.name)
    FROM fuzzy_variables v WHERE v.system_id = '$SYS_ID'
  ),
  'regras', (
    SELECT jsonb_agg(jsonb_build_object(
      'posicao', r.position,
      'regra', r.rule_text,
      'peso', r.weight
    ) ORDER BY r.position)
    FROM fuzzy_rules r WHERE r.system_id = '$SYS_ID'
  )
))" > "$GOLD_DIR/regras_fuzzy.json"

echo "   -> $(jq '.regras | length' "$GOLD_DIR/regras_fuzzy.json") regras exportadas"

# 2. PSO - otimiza parametros das MF com dados de referencia
echo ">>> Rodando PSO..."
PSO_RESULT=$(curl -s "$API/api/systems/$SYS_ID/optimize-pso" \
  -H 'Content-Type: application/json' \
  -d '{
    "target_inputs": [
      {"receita_anual_usd": 1000000, "total_funcionarios": 50, "gravidade_ataque": 20},
      {"receita_anual_usd": 100000000, "total_funcionarios": 5000, "gravidade_ataque": 15},
      {"receita_anual_usd": 5000000, "total_funcionarios": 100, "gravidade_ataque": 85},
      {"receita_anual_usd": 200000000, "total_funcionarios": 40000, "gravidade_ataque": 45},
      {"receita_anual_usd": 900000000, "total_funcionarios": 250000, "gravidade_ataque": 75},
      {"receita_anual_usd": 1000000000, "total_funcionarios": 400000, "gravidade_ataque": 95}
    ],
    "target_outputs": [
      {"impacto_financeiro": 15},
      {"impacto_financeiro": 10},
      {"impacto_financeiro": 55},
      {"impacto_financeiro": 45},
      {"impacto_financeiro": 80},
      {"impacto_financeiro": 95}
    ],
    "population_size": 30,
    "max_iterations": 100
  }')

BEST_FIT=$(echo "$PSO_RESULT" | jq -r '.best_fitness')
echo "   -> best_fitness = $BEST_FIT"
echo "$PSO_RESULT" | jq '.best_position' > "$GOLD_DIR/parametros_pso_otimizados.json"
echo "$PSO_RESULT" | jq '{best_fitness, best_position}' > "$GOLD_DIR/parametros_pso.json"
echo "   -> parametros salvos"

# 3. Aplica parametros otimizados no BD
echo ">>> Aplicando parametros PSO no sistema..."
APPLY_RESULT=$(curl -s "$API/api/systems/$SYS_ID/apply-pso-params" \
  -H 'Content-Type: application/json' \
  -d "$(echo "$PSO_RESULT" | jq '{params: .best_position}')")
echo "   -> $(echo "$APPLY_RESULT" | jq -r '.updated_terms') termos atualizados"

# 4. Gera previsoes para cenarios de teste
echo ">>> Gerando previsoes..."
echo "receita_anual_usd,total_funcionarios,gravidade_ataque,impacto_financeiro_previsto" > "$GOLD_DIR/previsoes_fuzzy.csv"

# Cenario 1: Baixo impacto
curl -s "$API/api/systems/$SYS_ID/simulate" \
  -H 'Content-Type: application/json' \
  -d '{"inputs": {"receita_anual_usd": 1000000, "total_funcionarios": 50, "gravidade_ataque": 20}}' \
  | jq -r '.outputs.impacto_financeiro // "0"' \
  | xargs -I{} echo "1000000,50,20,{}" >> "$GOLD_DIR/previsoes_fuzzy.csv"

# Cenario 2: Medio impacto
curl -s "$API/api/systems/$SYS_ID/simulate" \
  -H 'Content-Type: application/json' \
  -d '{"inputs": {"receita_anual_usd": 5000000, "total_funcionarios": 100, "gravidade_ataque": 85}}' \
  | jq -r '.outputs.impacto_financeiro // "0"' \
  | xargs -I{} echo "5000000,100,85,{}" >> "$GOLD_DIR/previsoes_fuzzy.csv"

# Cenario 3: Alto impacto
curl -s "$API/api/systems/$SYS_ID/simulate" \
  -H 'Content-Type: application/json' \
  -d '{"inputs": {"receita_anual_usd": 900000000, "total_funcionarios": 250000, "gravidade_ataque": 75}}' \
  | jq -r '.outputs.impacto_financeiro // "0"' \
  | xargs -I{} echo "900000000,250000,75,{}" >> "$GOLD_DIR/previsoes_fuzzy.csv"

# Cenario 4: Maximo impacto
curl -s "$API/api/systems/$SYS_ID/simulate" \
  -H 'Content-Type: application/json' \
  -d '{"inputs": {"receita_anual_usd": 1000000000, "total_funcionarios": 400000, "gravidade_ataque": 95}}' \
  | jq -r '.outputs.impacto_financeiro // "0"' \
  | xargs -I{} echo "1000000000,400000,95,{}" >> "$GOLD_DIR/previsoes_fuzzy.csv"

echo "   -> $(tail -n +2 "$GOLD_DIR/previsoes_fuzzy.csv" | wc -l) previsoes geradas"

# 5. Exporta tambem como JSON batch
cat "$GOLD_DIR/previsoes_fuzzy.csv" | python3 -c "
import csv, json, sys
reader = csv.DictReader(sys.stdin)
rows = []
for row in reader:
    rows.append({k: float(v) for k, v in row.items()})
print(json.dumps(rows, indent=2))
" > "$GOLD_DIR/previsoes_fuzzy.json" 2>/dev/null || true

echo ""
echo "=== Gold gerado em $GOLD_DIR/ ==="
ls -lh "$GOLD_DIR/"
echo ""
echo "Resumo:"
echo "  regras_fuzzy.json            - regras + variaveis + termos"
echo "  parametros_pso.json          - fitness + parametros otimizados"
echo "  parametros_pso_otimizados.json - parametros puros (array)"
echo "  previsoes_fuzzy.csv          - predicoes CSV"
echo "  previsoes_fuzzy.json         - predicoes JSON"
