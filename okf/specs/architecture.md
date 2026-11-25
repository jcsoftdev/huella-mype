---
type: Spec
title: Architecture, repo layout, stack, tests and CI
description: How the pieces fit and what CI must prove; the diagram goes into the README.
tags: [architecture, monorepo, testing, ci]
resource: ../docs/research/research-stellar-tech.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Flow
```
OECE monthly bulk (tender months 2023-07..2025-12, sha256) + sanctions CSV
      │ packages/etl: fetch → archive snapshot (release) → build → publish huellas (release)
      ▼
packages/core: buildHuella → JCS → sha256 = root ; subjectKey(ruc) ; verdict mapping   ◄── same code in browser and tools/verify
      │
apps/web (Vercel, build pulls huellas release)
  ├─ pages: search, huella, verificar, operador ──► RPC (browser): status, claimed_by, pending_claim
  ├─ api: attest, revoke, claim/approve (attester key, Production env only)
  └─ wallet (Wallets Kit/Freighter): request_claim, USDC trustline, lender disbursement (memo = root)
      ▼
Soroban registry (testnet)  +  classic USDC trustline/payments (testnet)
scripts/chain: deploy, set-attester, batch-attest, keep-alive
tools/verify: standalone verifier (no dependency on our server)
```

# Repo layout and owners
```
contracts/registry/  packages/registry-client/  scripts/chain/  apps/web/app/api/claim/      A
packages/etl/  data/  apps/web/app/api/search/  apps/web/app/api/payments/  apps/web/lib/payments/   B
packages/core/  tools/verify/  apps/web/app/api/ (attest, revoke, huella)  .github/  root configs   C
apps/web pages, components, layout, next.config, lib/ (except payments), apps/web/package.json, .env.example, Vercel   D
okf/   everyone (seams need every affected owner)
```

# Stack (checked Nov 24, 2026)
Rust ≥ 1.91 + `wasm32v1-none`, soroban-sdk 30.0.0, stellar-cli ≥ 30.0.1, testnet protocol 30, RPC https://soroban-testnet.stellar.org (CORS open). Node ≥ 22.12, pnpm, TypeScript, `@stellar/stellar-sdk` 17.4.0, `@creit.tech/stellar-wallets-kit` 2.8.0, `canonicalize` 5.1.0, Next.js on Vercel, vitest, Playwright. USDC: Circle testnet USDC (issuer and faucet to confirm in A/B spike); fallback a team-issued test asset clearly named as test.

# Tests and CI
| Layer | Minimum |
|---|---|
| Contract | every case in [registry spec](registry-contract.md) |
| Core | every vector in [huella spec](huella-document-v1.md) |
| ETL | parse, filters, join, window, sanctions, demo generation — fixtures only |
| API | attest, revoke, claim approve (auth + demo auto-approve only for the demo subject), payments filter — mocked chain |
| Tools | verify script against a mocked status |
| E2E | Playwright: demo huella → VERIFICADA; tampered JSON → ALTERADA |
GitHub Actions on every PR and push: `cargo test`, `pnpm -r lint test`, `pnpm --filter web build`. README shows badge and counts.

# Deploy policy
Hello-world deployed **Wed Nov 26**; every merge to main deploys. On Dec 4 after submission: protect main and disable auto-deploy until Dec 8. Check announced testnet reset dates.
