---
type: Spec
title: API routes and web panel — pages, states, copy rules
description: The seam between the API owners and the UI, and the UX the video shows.
tags: [api, web, ux]
resource: ../docs/research/research-stellar-tech.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# API (Next.js route handlers)
| Route | Owner | Auth | Response |
|---|---|---|---|
| `GET /api/search?q=` | B | public | ≤ 10 `{ ruc, legalName, contractsCount, sizeProxy }` |
| `GET /api/huella/:ruc` | C | public | `{ huella, subject, root }` (chain status is read by the browser) |
| `POST /api/attest` | C | operator token | `{ txHash, version, root }` — single attest, retry once on `tx_bad_seq` |
| `POST /api/revoke` | C | operator token | `{ txHash }` |
| `POST /api/claim/approve` | A | operator token, or automatic for the Empresa Demo subject only | `{ txHash }` |
| `GET /api/payments/:account` | B | public | linked disbursements (payments to the account with memo hash = a huella root of its subject) |
Batch attestation runs only from `scripts/chain`, never through the API. The browser reads the contract directly (testnet RPC allows any origin): `status`, `claimed_by`, `pending_claim`.
Env: `STELLAR_RPC_URL`, `REGISTRY_CONTRACT_ID`, `ATTESTER_SECRET`, `OPERATOR_TOKEN`, `HUELLAS_RELEASE_URL`. Secrets set in Vercel **Production only** (not Preview). `.env.example` documents all.

# Pages (owner D) — Spanish UI
| Route | Must have |
|---|---|
| `/` | Value prop; search; buttons "Soy empresa" / "Soy analista" / "Probar con Empresa Demo" |
| `/h/[ruc]` | Metrics cards, size proxy, señales with "qué significa" tooltips, null explanations, contracts → OECE, verdict badge, **Solicitar vincular mi cuenta** (wallet: Freighter via Wallets Kit; Friendbot funding prompt if the account is new; wrong-network warning), pending/approved state, **Habilitar USDC** (trustline) once approved, linked disbursements, share link + QR + JSON |
| `/verificar` | Link or JSON → root in browser → `status` → verdict banner + subtitle; details; explorer links; **Enviar desembolso de prueba** (lender wallet, USDC, memo = root) |
| `/operador` | Token; attest one; approve pending claims; revoke |
| all | Footer: "Datos: OECE / SEACE (CC BY 4.0); sancionados: Datos Abiertos (ODC-By)" + GitHub |

# States
Loading; RUC not found; RUC 10 ("Por ahora solo empresas con RUC 20"); chain unreachable; wallet missing; account unfunded; wrong network; claim pending; claim rejected/revoked; trustline missing; payment failed.

# Copy rules
Never "score", "calificación", "riesgo", "limpio". Use "evidencia", "señales", "verificada", "cuenta vinculada". Amounts "S/ 2,241,668.35"; dates es-PE. Rosa/Luis only with Empresa Demo, labeled "caso ilustrativo".
