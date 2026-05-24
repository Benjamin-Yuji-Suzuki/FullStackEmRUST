#!/usr/bin/env bash
# Otimizador quadratico via API
# Uso: ./scripts/otimizar_quadratico.sh <a> <b> <c> <d> <e> <f> <x_min> <x_max> <y_min> <y_max> [system_id]
set -euo pipefail

API_BASE="http://127.0.0.1:3000"

A=${1:-1}
B=${2:-0}
C=${3:-1}
D=${4:-0}
E=${5:-0}
F=${6:-0}
XMIN=${7:--10}
XMAX=${8:-10}
YMIN=${9:--10}
YMAX=${10:-10}
SYS_ID="${11:-}"

echo "f(x,y) = ${A}x² + ${B}xy + ${C}y² + ${D}x + ${E}y + ${F}"
echo "Dominio: x=[$XMIN,$XMAX]  y=[$YMIN,$YMAX]"
echo "---"

BODY=$(cat <<EOF
{
  "function": {"a":$A,"b":$B,"c":$C,"d":$D,"e":$E,"f":$F},
  "domain": {"x_min":$XMIN,"x_max":$XMAX,"y_min":$YMIN,"y_max":$YMAX}
EOF
)

if [ -n "$SYS_ID" ]; then
  BODY+=", \"system_id\": \"$SYS_ID\""
fi
BODY+="}"

curl -s "$API_BASE/api/optimize" \
  -H "Content-Type: application/json" \
  -d "$BODY" | jq .
