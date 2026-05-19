#!/usr/bin/env bash
# Script de demonstração — Sprint 2 FuzzySimulated
# Uso: bash demo-sprint2.sh
# Requer: servidor rodando em http://127.0.0.1:3000

BASE="http://127.0.0.1:3000"
DIR="$(dirname "$(readlink -f "$0")")"
SYS_ID=""

echo "============================================"
echo "  FuzzySimulated — Demo Sprint 2"
echo "============================================"
echo ""

# ─── VERIFICAR SERVIDOR ────────────────────────
echo "▶ Verificando servidor em $BASE..."
if curl -s --connect-timeout 2 "$BASE/api/systems" > /dev/null 2>&1; then
    echo "   OK — servidor respondendo"
else
    echo "   ⚠ Servidor não encontrado em $BASE"
    echo "   Inicie com 'cargo leptos watch' em outro terminal"
    echo "   Continuando apenas com testes..."
fi
echo ""

# ─── 1. TESTES ──────────────────────────────────
echo "▶ 1. Testes unitários (16)"
echo "   $ cargo test -p server -- --skip ignored"
echo ""
(cd "$DIR/fuzzysimulated" && cargo test -p server -- --skip ignored 2>&1) | tail -30
echo ""

# ─── 2. CRUD SISTEMAS ──────────────────────────
echo "▶ 2. CRUD Sistemas"
echo ""

echo "  2a. Listar sistemas"
curl -s "$BASE/api/systems" 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   (vazio)"
echo ""

echo "  2b. Criar sistema"
RESP=$(curl -s -X POST "$BASE/api/systems" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2","description":"Teste ao vivo","defuzz_method":"centroid"}') || RESP=""
SYS_ID=$(echo "$RESP" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('id',''))" 2>/dev/null || echo "")
echo "   ID: ${SYS_ID:-falhou}"

if [ -z "$SYS_ID" ]; then echo "   (abortando)"; else

echo "  2c. Visualizar sistema"
curl -s "$BASE/api/systems/$SYS_ID" | python3 -m json.tool
echo ""

echo "  2d. Editar sistema"
curl -s -X PUT "$BASE/api/systems/$SYS_ID" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2 (editado)","description":"Atualizado","defuzz_method":"bisector"}' \
  | python3 -m json.tool
echo ""

# ─── 3. CRUD VARIÁVEIS ─────────────────────────
echo "▶ 3. Variáveis e Termos"
echo ""

echo "  3a. Criar variável antecedente (temperatura)"
VAR_RESP=$(curl -s -X POST "$BASE/api/systems/$SYS_ID/variables" \
  -H "Content-Type: application/json" \
  -d '{"name":"temperatura","role":"antecedent","universe_min":0,"universe_max":50,"resolution":501}')
VAR_ID=$(echo "$VAR_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id',''))" 2>/dev/null || echo "")
echo "   Var ID: ${VAR_ID:-falhou}"

echo "  3b. Criar variável consequente (conforto) — necessária pra simular"
CONS_RESP=$(curl -s -X POST "$BASE/api/systems/$SYS_ID/variables" \
  -H "Content-Type: application/json" \
  -d '{"name":"conforto","role":"consequent","universe_min":0,"universe_max":10,"resolution":501}')
CONS_ID=$(echo "$CONS_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id',''))" 2>/dev/null || echo "")
echo "   Cons ID: ${CONS_ID:-falhou}"

if [ -n "$VAR_ID" ]; then
echo "  3c. Criar termo trimf válido"
curl -s -X POST "$BASE/api/variables/$VAR_ID/terms" \
  -H "Content-Type: application/json" \
  -d '{"label":"morno","mf_type":"trimf","params":[15,25,35]}' \
  | python3 -m json.tool
echo ""

echo "  3d. Rejeitar termo inválido (trimf a>b) — esperado HTTP 422"
HTTP_CODE=$(curl -s -o /tmp/demo_resp.json -w "%{http_code}" -X POST "$BASE/api/variables/$VAR_ID/terms" \
  -H "Content-Type: application/json" \
  -d '{"label":"invalido","mf_type":"trimf","params":[30,20,10]}')
echo "   Status: $HTTP_CODE"
python3 -m json.tool /tmp/demo_resp.json 2>/dev/null || cat /tmp/demo_resp.json
echo ""
fi

# ─── 4. REGRAS ─────────────────────────────────
echo "▶ 4. Regras"
echo ""

echo "  4a. Criar regra"
curl -s -X POST "$BASE/api/systems/$SYS_ID/rules" \
  -H "Content-Type: application/json" \
  -d '{"rule_text":"SE temperatura é morno ENTÃO conforto é agradavel","weight":1.0}' \
  | python3 -m json.tool
echo ""

# ─── 5. SIMULAÇÃO ──────────────────────────────
echo "▶ 5. Simulação"
echo ""

echo "  5a. Executar simulação (temp=24)"
curl -s -X POST "$BASE/api/systems/$SYS_ID/simulate" \
  -H "Content-Type: application/json" \
  -d '{"inputs":{"temperatura":24.0}}' \
  | python3 -m json.tool
echo ""

echo "  5b. Histórico de simulações"
curl -s "$BASE/api/systems/$SYS_ID/simulations" | python3 -m json.tool
echo ""

# ─── 6. EXTERNAL API ───────────────────────────
echo "▶ 6. OpenWeather"
echo ""

echo "  6a. Buscar clima (Belém)"
curl -s "$BASE/api/weather?city=Belem" | python3 -m json.tool
echo ""

echo "  6b. Cidade inexistente (esperado 404)"
HTTP_CODE=$(curl -s -o /tmp/demo_weather.json -w "%{http_code}" "$BASE/api/weather?city=CidadeInexistenteXYZ")
echo "   Status: $HTTP_CODE"
python3 -m json.tool /tmp/demo_weather.json 2>/dev/null || cat /tmp/demo_weather.json
echo ""

# ─── 7. DUPLICAR / EXPORTAR ────────────────────
echo "▶ 7. Duplicar e Exportar"
echo ""

echo "  7a. Duplicar sistema"
curl -s -X POST "$BASE/api/systems/$SYS_ID/duplicate" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2 (cópia)"}' \
  | python3 -c "
import sys,json
d=json.load(sys.stdin)
print(f'   Criado: {d.get(\"name\",\"?\")} (ID: {str(d.get(\"id\",\"\"))[:8]}...)' )"
echo ""

echo "  7b. Exportar sistema"
curl -s "$BASE/api/systems/$SYS_ID/export" | python3 -m json.tool
echo ""

# ─── 8. AUDITORIA ──────────────────────────────
echo "▶ 8. Auditoria"
echo ""

echo "  8a. Timeline"
curl -s "$BASE/api/systems/$SYS_ID/audit" | python3 -c "
import sys,json
d=json.load(sys.stdin)
evts=d.get('events',[])
print(f'   {len(evts)} evento(s) registrado(s)')
for e in evts[:5]:
    print(f'   \u2022 {e.get(\"action_type\",\"?\")} - {e.get(\"entity_type\",\"?\")} ({e.get(\"created_at\",\"?\")[:19]})')"
echo ""

# ─── 9. EXCLUIR ────────────────────────────────
echo "▶ 9. Limpeza"
echo "   DELETE /api/systems/$SYS_ID"
curl -s -o /dev/null -w "   Status: %{http_code}\n" -X DELETE "$BASE/api/systems/$SYS_ID"
echo ""

fi  # fim do if SYS_ID

echo ""
echo "============================================"
echo "  Demo concluída!"
echo "============================================"
