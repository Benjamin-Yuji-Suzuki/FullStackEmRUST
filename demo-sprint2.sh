#!/usr/bin/env bash
# Script de demonstração — Sprint 2 FuzzySimulated
# Uso: bash demo-sprint2.sh
set -e

BASE="http://127.0.0.1:3000"
SYS_ID=""

echo "============================================"
echo "  FuzzySimulated — Demo Sprint 2"
echo "============================================"
echo ""

# ─── 1. TESTES ──────────────────────────────────
echo "▶ 1. Testes unitários (16)"
echo "$ cargo test -p server -- --skip ignored"
cargo test -p server -- --skip ignored 2>&1 | grep "^test "
echo ""

# ─── 2. CRUD SISTEMAS ──────────────────────────
echo "▶ 2. CRUD Sistemas"
echo ""

echo "  2a. Listar sistemas (GET /api/systems)"
curl -s "$BASE/api/systems" | python3 -m json.tool 2>/dev/null || echo "   (vazio — servidor precisa estar rodando)"
echo ""

echo "  2b. Criar sistema (POST /api/systems)"
RESP=$(curl -s -X POST "$BASE/api/systems" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2","description":"Teste ao vivo","defuzz_method":"centroid"}')
echo "   $RESP"
SYS_ID=$(echo "$RESP" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id',''))" 2>/dev/null)
echo ""

echo "  2c. Visualizar sistema (GET /api/systems/$SYS_ID)"
curl -s "$BASE/api/systems/$SYS_ID" | python3 -m json.tool 2>/dev/null
echo ""

echo "  2d. Editar sistema (PUT /api/systems/$SYS_ID)"
curl -s -X PUT "$BASE/api/systems/$SYS_ID" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2 (editado)","description":"Atualizado","defuzz_method":"bisector"}' \
  | python3 -m json.tool 2>/dev/null
echo ""

# ─── 3. CRUD VARIÁVEIS ─────────────────────────
echo "▶ 3. CRUD Variáveis e Termos"
echo ""

echo "  3a. Criar variável antecedente (POST /api/systems/$SYS_ID/variables)"
VAR_RESP=$(curl -s -X POST "$BASE/api/systems/$SYS_ID/variables" \
  -H "Content-Type: application/json" \
  -d '{"name":"temperatura","role":"antecedent","universe_min":0,"universe_max":50,"resolution":501}')
VAR_ID=$(echo "$VAR_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id',''))" 2>/dev/null)
echo "   Variável ID: $VAR_ID"
echo ""

echo "  3b. Criar termo trimf (POST /api/variables/$VAR_ID/terms)"
curl -s -X POST "$BASE/api/variables/$VAR_ID/terms" \
  -H "Content-Type: application/json" \
  -d '{"label":"morno","mf_type":"trimf","params":[15,25,35]}' \
  | python3 -m json.tool 2>/dev/null
echo ""

echo "  3c. Rejeitar termo inválido (POST com parâmetros incoerentes)"
curl -s -X POST "$BASE/api/variables/$VAR_ID/terms" \
  -H "Content-Type: application/json" \
  -d '{"label":"invalido","mf_type":"trimf","params":[30,20,10]}' \
  | python3 -m json.tool 2>/dev/null
echo ""

# ─── 4. CRUD REGRAS ────────────────────────────
echo "▶ 4. CRUD Regras"
echo ""

echo "  4a. Criar regra (POST /api/systems/$SYS_ID/rules)"
curl -s -X POST "$BASE/api/systems/$SYS_ID/rules" \
  -H "Content-Type: application/json" \
  -d '{"rule_text":"SE temperatura é morno ENTÃO conforto é agradavel","weight":1.0}' \
  | python3 -m json.tool 2>/dev/null
echo ""

# ─── 5. SIMULAÇÃO ──────────────────────────────
echo "▶ 5. Simulação"
echo ""

echo "  5a. Executar simulação (POST /api/systems/$SYS_ID/simulate)"
curl -s -X POST "$BASE/api/systems/$SYS_ID/simulate" \
  -H "Content-Type: application/json" \
  -d '{"inputs":{"temperatura":24.0}}' \
  | python3 -m json.tool 2>/dev/null
echo ""

echo "  5b. Histórico de simulações (GET /api/systems/$SYS_ID/simulations)"
curl -s "$BASE/api/systems/$SYS_ID/simulations" | python3 -m json.tool 2>/dev/null
echo ""

# ─── 6. EXTERNAL API ───────────────────────────
echo "▶ 6. API Externa — OpenWeather"
echo ""

echo "  6a. Buscar clima (GET /api/weather?city=Belém)"
curl -s "$BASE/api/weather?city=Belém" | python3 -m json.tool 2>/dev/null
echo ""

echo "  6b. Cidade inexistente (GET /api/weather?city=Atlantida)"
curl -s "$BASE/api/weather?city=Atlantida" | python3 -m json.tool 2>/dev/null
echo ""

# ─── 7. DUPLICAR / EXPORTAR ────────────────────
echo "▶ 7. Duplicar e Exportar"
echo ""

echo "  7a. Duplicar sistema (POST /api/systems/$SYS_ID/duplicate)"
curl -s -X POST "$BASE/api/systems/$SYS_ID/duplicate" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2 (cópia)"}' \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'   Criado: {d.get(\"name\",\"?\")} (ID: {d.get(\"id\",\"\")[:8]}...)' )" 2>/dev/null
echo ""

echo "  7b. Exportar sistema (GET /api/systems/$SYS_ID/export)"
curl -s "$BASE/api/systems/$SYS_ID/export" | python3 -m json.tool 2>/dev/null
echo ""

# ─── 8. AUDITORIA ──────────────────────────────
echo "▶ 8. Auditoria"
echo ""

echo "  8a. Timeline (GET /api/systems/$SYS_ID/audit)"
curl -s "$BASE/api/systems/$SYS_ID/audit" | python3 -c "
import sys,json
d=json.load(sys.stdin)
evts=d.get('events',[])
print(f'   {len(evts)} evento(s) registrado(s)')
for e in evts[:5]:
    print(f'   • {e.get(\"action_type\",\"?\")} - {e.get(\"entity_type\",\"?\")} ({e.get(\"created_at\",\"?\")[:19]})')
" 2>/dev/null
echo ""

# ─── 9. EXCLUIR ────────────────────────────────
echo "▶ 9. Limpeza — excluir sistema de demonstração"
echo "   DELETE /api/systems/$SYS_ID"
curl -s -o /dev/null -w "   Status: %{http_code}\n" -X DELETE "$BASE/api/systems/$SYS_ID"
echo ""

echo "============================================"
echo "  Demo concluída!"
echo "============================================"
