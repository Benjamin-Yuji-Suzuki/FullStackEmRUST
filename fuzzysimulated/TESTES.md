# Testes — FuzzySimulated

| Suite | Qtde | DB | Como rodar |
|---|---|---|---|---|
| Unit (inline) | 30 | ❌ | `cargo test -p server --lib` |
| Unit (tests/) | 16 | ❌ | `cargo test -p server --test all -- unit::` |
| HTTP Axum | 64 | ✅ | `DATABASE_URL=... cargo test -p server --test all -- --skip ignored` |
| Integration | 6 | ✅ | `DATABASE_URL=... cargo test -p server --test all -- --ignored` |
| **Total server** | **116** | | `DATABASE_URL=... cargo test -p server` |

## Comandos

### Unitários (46 testes, sem DB: 30 inline + 16 tests/)
```bash
cargo test -p server --lib
cargo test -p server --test all -- unit::
```

### HTTP Axum (64 testes, serializados, requer DB)
```bash
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test all -- --skip ignored
```

Inclui `test_e2e_full_pipeline` — 20 operações: criar sistema → variáveis → termos → regras → simular Mamdani → diagnóstico → SVG → TSK → batch → rule-matrix → sweep → surface → cenários CRUD → comparar → duplicar → import/export → status → PSO → auditoria.

### Integração (6 testes, requer DB)
```bash
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server --test all -- --ignored
```

### Todos de uma vez
```bash
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server
```

### Check compilação
```bash
cargo check -p server -p app -p frontend
```
