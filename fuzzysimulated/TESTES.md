# Testes — FuzzySimulated

## Unitários (22 testes, sem DB)
```bash
cargo test -p server -- --skip ignored
```

## Integração (8 testes, requer DB)
```bash
DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server -- --include-ignored
```

## Todos de uma vez
```bash
cargo test -p server && DATABASE_URL=postgres://ben:1234@localhost/fuzzysimulated_test cargo test -p server -- --include-ignored
```
