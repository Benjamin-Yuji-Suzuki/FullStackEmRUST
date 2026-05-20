#!/usr/bin/env bash
# Gera cobertura de testes com cargo-llvm-cov e envia pro Codecov.
# Pré-requisitos: rustup component add llvm-tools-preview
#                 cargo install cargo-llvm-cov --locked
# Uso: CODECOV_TOKEN=<token> ./coverage.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT/fuzzysimulated"

echo ">>> Gerando cobertura (lcov)..."
cargo llvm-cov --lcov --output-path "$ROOT/lcov.info" -p server -- --skip ignored

echo ""
echo ">>> Resumo de cobertura:"
cargo llvm-cov report -p server 2>/dev/null || true

echo ""
if [ -n "${CODECOV_TOKEN:-}" ]; then
    echo ">>> Enviando pro Codecov..."
    if command -v codecov &>/dev/null; then
        codecov --token "$CODECOV_TOKEN" --file "$ROOT/lcov.info"
    else
        # Fallback: uploader universal do codecov (bash)
        curl -s https://codecov.io/bash \
            | CODECOV_TOKEN="$CODECOV_TOKEN" bash -s -- -f "$ROOT/lcov.info" -Z
    fi
    echo "✔ Enviado ao Codecov"
else
    echo "⚠ CODECOV_TOKEN não definido. Pulando upload."
fi

echo ""
echo "✔ lcov.info salvo em $ROOT/lcov.info"
