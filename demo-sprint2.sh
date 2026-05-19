#!/usr/bin/env bash
# Script de demonstração — Sprint 2 FuzzySimulated
# Uso: bash demo-sprint2.sh
# Requer: servidor rodando em http://127.0.0.1:3000

BASE="http://127.0.0.1:3000"
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
cargo test -p server -- --skip ignored 2>&1 | tail -30
echo ""

# ─── 2. CRUD SISTEMAS ──────────────────────────
echo "▶ 2. CRUD Sistemas"
echo ""

echo "  2a. Listar sistemas"
curl -sf "$BASE/api/systems" 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   (vazio ou servidor offline)"
echo ""

echo "  2b. Criar sistema"
RESP=$(curl -sf -X POST "$BASE/api/systems" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2","description":"Teste ao vivo","defuzz_method":"centroid"}' 2>/dev/null) || RESP=""
SYS_ID=$(echo "$RESP" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('id',''))" 2>/dev/null || echo "")
echo "   ID: ${SYS_ID:-falhou}"
echo ""

if [ -n "$SYS_ID" ]; then

echo "  2c. Visualizar sistema"
curl -sf "$BASE/api/systems/$SYS_ID" 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""

echo "  2d. Editar sistema"
curl -sf -X PUT "$BASE/api/systems/$SYS_ID" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2 (editado)","description":"Atualizado","defuzz_method":"bisector"}' \
  2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""

# ─── 3. CRUD VARIÁVEIS ─────────────────────────
echo "▶ 3. Variáveis e Termos"
echo ""

echo "  3a. Criar variável antecedente"
VAR_RESP=$(curl -sf -X POST "$BASE/api/systems/$SYS_ID/variables" \
  -H "Content-Type: application/json" \
  -d '{"name":"temperatura","role":"antecedent","universe_min":0,"universe_max":50,"resolution":501}' 2>/dev/null)
VAR_ID=$(echo "$VAR_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id',''))" 2>/dev/null || echo "")
echo "   Var ID: ${VAR_ID:-falhou}"

if [ -n "$VAR_ID" ]; then
echo "  3b. Criar termo trimf"
curl -sf -X POST "$BASE/api/variables/$VAR_ID/terms" \
  -H "Content-Type: application/json" \
  -d '{"label":"morno","mf_type":"trimf","params":[15,25,35]}' \
  2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""

echo "  3c. Rejeitar termo inválido (trimf a>b)"
curl -sf -X POST "$BASE/api/variables/$VAR_ID/terms" \
  -H "Content-Type: application/json" \
  -d '{"label":"invalido","mf_type":"trimf","params":[30,20,10]}' \
  2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""
fi

# ─── 4. REGRAS ─────────────────────────────────
echo "▶ 4. Regras"
echo ""

echo "  4a. Criar regra"
curl -sf -X POST "$BASE/api/systems/$SYS_ID/rules" \
  -H "Content-Type: application/json" \
  -d '{"rule_text":"SE temperatura é morno ENTÃO conforto é agradavel","weight":1.0}' \
  2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""

# ─── 5. SIMULAÇÃO ──────────────────────────────
echo "▶ 5. Simulação"
echo ""

echo "  5a. Executar simulação"
curl -sf -X POST "$BASE/api/systems/$SYS_ID/simulate" \
  -H "Content-Type: application/json" \
  -d '{"inputs":{"temperatura":24.0}}' \
  2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""

echo "  5b. Histórico de simulações"
curl -sf "$BASE/api/systems/$SYS_ID/simulations" 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""

# ─── 6. EXTERNAL API ───────────────────────────
echo "▶ 6. OpenWeather"
echo ""

echo "  6a. Buscar clima (Belém)"
curl -sf "$BASE/api/weather?city=Bel\u00e9m" 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro (API key?)"
echo ""

echo "  6b. Cidade inexistente"
curl -sf "$BASE/api/weather?city=Atlantida" 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro esperado"
echo ""

# ─── 7. DUPLICAR / EXPORTAR ────────────────────
echo "▶ 7. Duplicar e Exportar"
echo ""

echo "  7a. Duplicar sistema"
curl -sf -X POST "$BASE/api/systems/$SYS_ID/duplicate" \
  -H "Content-Type: application/json" \
  -d '{"name":"Demo Sprint2 (cópia)"}' \
  2>/dev/null | python3 -c "
import sys,json
d=json.load(sys.stdin)
print(f'   Criado: {d.get(\"name\",\"?\")} (ID: {str(d.get(\"id\",\"\"))[:8]}...)' )
" 2>/dev/null || echo "   erro"
echo ""

echo "  7b. Exportar sistema"
curl -sf "$BASE/api/systems/$SYS_ID/export" 2>/dev/null | python3 -m json.tool 2>/dev/null || echo "   erro"
echo ""

# ─── 8. AUDITORIA ──────────────────────────────
echo "▶ 8. Auditoria"
echo ""

echo "  8a. Timeline"
curl -sf "$BASE/api/systems/$SYS_ID/audit" 2>/dev/null | python3 -c "
import sys,json
d=json.load(sys.stdin)
evts=d.get('events',[])
print(f'   {len(evts)} evento(s) registrado(s)')
for e in evts[:5]:
    print(f'   \u2022 {e.get(\"action_type\",\"?\")} - {e.get(\"entity_type\",\"?\")} ({e.get(\"created_at\",\"?\")[:19]})')
" 2>/dev/null || echo "   erro"
echo ""

# ─── 9. EXCLUIR ────────────────────────────────
echo "▶ 9. Limpeza"
echo "   DELETE /api/systems/$SYS_ID"
curl -sf -o /dev/null -w "   Status: %{http_code}\n" -X DELETE "$BASE/api/systems/$SYS_ID" 2>/dev/null || echo "   erro"
echo ""

fi  # fim do if SYS_ID não vazio

echo "============================================"
echo "  Demo concluída!"
echo "============================================"
