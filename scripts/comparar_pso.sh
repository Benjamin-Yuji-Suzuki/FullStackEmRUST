#!/usr/bin/env bash
# Compara PSO com diferentes configs no mesmo sistema
# Uso: ./scripts/comparar_pso.sh <sistema>
set -euo pipefail

DB_URL="postgres://ben:1234@localhost/fuzzysimulated"
API_BASE="http://127.0.0.1:3000"

case "${1:-}" in
  conforto)
    SYS_ID=$(psql "$DB_URL" -tA -c "SELECT id FROM fuzzy_systems WHERE name='Conforto Térmico' LIMIT 1")
    INPUTS='[{"temperatura":10,"umidade":20},{"temperatura":24,"umidade":55},{"temperatura":35,"umidade":85}]'
    OUTPUTS='[{"conforto":2},{"conforto":8},{"conforto":1}]'
    ;;
  risco)
    SYS_ID=$(psql "$DB_URL" -tA -c "SELECT id FROM fuzzy_systems WHERE name='Analise de Risco' LIMIT 1")
    INPUTS='[{"probabilidade":10,"impacto":10},{"probabilidade":50,"impacto":50},{"probabilidade":90,"impacto":90}]'
    OUTPUTS='[{"risco":10},{"risco":50},{"risco":90}]'
    ;;
  *)
    echo "Uso: $0 conforto|risco"
    exit 1
    ;;
esac

for config in "10 20" "20 50" "30 100" "50 200"; do
  pop=$(echo $config | cut -d' ' -f1)
  iters=$(echo $config | cut -d' ' -f2)
  echo "--- Pop=$pop  Iters=$iters ---"
  curl -s "$API_BASE/api/systems/$SYS_ID/optimize-pso" \
    -H "Content-Type: application/json" \
    -d "$(cat <<EOF
{
  "target_inputs": $INPUTS,
  "target_outputs": $OUTPUTS,
  "population_size": $pop,
  "max_iterations": $iters
}
EOF
)" | jq -r '"  fitness=\(.best_fitness)  params=\(.best_position | join(\", \"))"'
  echo ""
done
