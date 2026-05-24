# Testes — FuzzySimulated

| Suite | Qtde | DB | Como rodar |
|---|---|---|---|
| Unit (inline) | 41 | ❌ | `cargo test -p server --lib` |
| HTTP Axum | 66 | ✅ | `cargo test -p server --test axum_api` (serial) |
| Integration | 8 | ✅ | `cargo test -p server --test api_test -- --ignored` |
| **Total** | **115** | | |

## Comandos

### Unitários (41 testes, sem DB)
```bash
cargo test -p server --lib
```

### HTTP Axum (66 testes, serializados, requer DB)
```bash
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test axum_api
```

Inclui `test_e2e_full_pipeline` — 22 operações: criar sistema → variáveis → termos → regras → simular Mamdani → diagnóstico → SVG → TSK → batch → rule-matrix → sweep → surface → cenários CRUD → comparar → duplicar → import/export → status → otimização quadrática → export → PSO → auditoria.

### Integração (8 testes, requer DB)
```bash
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test api_test -- --ignored
```

### Todos de uma vez
```bash
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server
```

### Check compilação
```bash
cargo check -p server -p app -p frontend
```
