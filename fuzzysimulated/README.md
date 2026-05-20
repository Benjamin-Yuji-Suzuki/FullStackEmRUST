<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# FuzzySimulated — Inference Platform

Plataforma full-stack 100% Rust para construção e simulação de sistemas de inferência fuzzy (Mamdani e TSK), com otimização por PSO e cálculo de ponto ótimo de funções objetivo multivariáveis.

**Projeto acadêmico** — CESUPA 02/2026  
**Disciplinas:** Qualidade e Projeto de Software · Inteligência Artificial e Computacional · Ciência de Dados · Resolução de Problemas Multivariáveis

---

## Stack

- **Frontend:** Leptos 0.8 (SSR + WASM hydrate)
- **Backend:** Axum 0.8 (REST API)
- **Banco:** PostgreSQL via SQLx (queries compile-checked)
- **Build:** cargo-leptos

## Telas

| Tela | Rota | Status |
|---|---|---|
| Dashboard | `/` | ✅ |
| Variáveis & Termos | `/vars?s=` | ✅ |
| Editor de Regras | `/rules?s=` | ✅ |
| Simulador | `/sim?s=` | ✅ |
| Histórico | `/hist?s=` | ✅ |
| Auditoria | `/audit?id=` | ✅ |
| Batch | `/batch` | ❌ Placeholder |
| Análise | `/analysis` | ❌ Placeholder |
| Otimizador | `/opt` | ✅ Novo |

## Casos de Uso

25 casos de uso implementados ou em andamento, incluindo UC21–UC25 para otimização de função objetivo multivariável.

## Desenvolvimento

```bash
# Watch mode (porta 3000)
cargo leptos watch

# Unit tests
cargo test -p server -- --skip ignored

# Todos os testes (requer DB)
DATABASE_URL=postgres://postgres@localhost/fuzzysimulated_test cargo test -p server
```
