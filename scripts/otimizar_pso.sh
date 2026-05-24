#!/usr/bin/env bash
# Uso: ./scripts/otimizar_pso.sh <sistema> [populacao] [iteracoes]
#   sistema: "conforto" | "risco" | <uuid>
#   populacao: padrao 20
#   iteracoes: padrao 50
set -euo pipefail

DB_URL="postgres://ben:1234@localhost/fuzzysimulated"
API_BASE="http://127.0.0.1:3000"
POP=${2:-20}
ITERS=${3:-50}

case "${1:-}" in
  conforto)
    SYS_ID=$(psql "$DB_URL" -tA -c "SELECT id FROM fuzzy_systems WHERE name='Conforto Térmico' LIMIT 1")
    echo "=== Conforto Térmico ==="
    TARGET_INPUTS='[{"temperatura":10,"umidade":20},{"temperatura":24,"umidade":55},{"temperatura":35,"umidade":85}]'
    TARGET_OUTPUTS='[{"conforto":2},{"conforto":8},{"conforto":1}]'
    ;;
  risco)
    SYS_ID=$(psql "$DB_URL" -tA -c "SELECT id FROM fuzzy_systems WHERE name='Analise de Risco' LIMIT 1")
    echo "=== Analise de Risco ==="
    TARGET_INPUTS='[{"probabilidade":10,"impacto":10},{"probabilidade":50,"impacto":50},{"probabilidade":90,"impacto":90}]'
    TARGET_OUTPUTS='[{"risco":10},{"risco":50},{"risco":90}]'
    ;;
  *)
    SYS_ID="${1:-}"
    if [ -z "$SYS_ID" ]; then
      echo "Uso: $0 <sistema> [populacao] [iteracoes]"
      echo "  sistema: \"conforto\" | \"risco\" | <uuid>"
      echo "  Ex: $0 conforto 30 100"
      echo "  Ex: $0 risco 20 50"
      exit 1
    fi
    TARGET_INPUTS='[{"x":10,"y":10},{"x":50,"y":50},{"x":90,"y":90}]'
    TARGET_OUTPUTS='[{"z":10},{"z":50},{"z":90}]'
    ;;
esac

echo "Populacao: $POP | Iteracoes: $ITERS"
echo "Target Inputs:  $(echo "$TARGET_INPUTS" | jq -c .)"
echo "Target Outputs: $(echo "$TARGET_OUTPUTS" | jq -c .)"
echo "---"

curl -s "$API_BASE/api/systems/$SYS_ID/optimize-pso" \
  -H "Content-Type: application/json" \
  -d "$(cat <<EOF
{
  "target_inputs": $TARGET_INPUTS,
  "target_outputs": $TARGET_OUTPUTS,
  "population_size": $POP,
  "max_iterations": $ITERS
}
EOF
)" | jq .
