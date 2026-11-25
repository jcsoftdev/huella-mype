---
type: Plan
title: Backlog — every task with owner, dependency, criterion and acceptance
description: The complete list of work; a task not here is not done before Gate 1.
tags: [backlog, tasks]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
sources:
  - resource: ../docs/research/research-ocds-data.md
    title: Data tasks input
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

Criterion codes: **US** Uso de Stellar · **FQ** Funcionalidad · **PI** Problema e impacto · **IN** Innovación · **PX** Presentación y UX. One GitHub issue per task, same ID.

# T — Everyone
| ID | Task | Crit | Acceptance |
|---|---|---|---|
| T1 | Repo public, collaborators added | FQ | Done |
| T2 | Repo registered in the Hub | — | **Done** |
| T3 | Kickoff 1 h: read [How we win](../strategy/how-we-win.md), assign lanes, freeze the 4 seams, create issues | all | okf/log.md "seams frozen" |
| T4 | Toolchain on every machine | FQ | `stellar version`, `node -v`, `cargo -V` |

# A — Stellar
| ID | Task | Dep | Crit | Acceptance |
|---|---|---|---|---|
| A1 | Prototype → `contracts/registry`, WASM build | T1 | FQ | wasm built |
| A2 | Add `RootIdx`, `status`, revoke by admin/issuing attester (+TTL), two-step claim, new errors/events | A1 | US | Matches [spec](../specs/registry-contract.md) |
| A3 | Full test list + snapshots | A2 | FQ | cargo test green |
| A4 | Deploy testnet, set attester, bindings, key custody | A3 | US | Contract on stellar.expert; client importable; keys in password manager |
| A5 | `scripts/chain`: deploy, set-attester, batch-attest (retry on bad seq), keep-alive | A4 | FQ | Each runs, documented |
| A6 | CLI spike of request_claim → approve_claim, with B (trustline + payment) | A4 | US | Round trip on testnet, commands in README draft |
| A7 | `/api/claim/approve`: operator-token approval; auto-approve only for Empresa Demo subject | A4, C1 | US | Tests; approval visible on explorer |
| A8 | Batch-attest demo set and real set (Dec 1); keep-alive check Dec 4 and 6 | B6, A5 | FQ | All demo RUCs VERIFICADA |
| A9 | README "Cómo usamos Stellar" + tx examples | A4 | US | Merged by D |

# B — Data, payments, evidence
| ID | Task | Dep | Crit | Acceptance |
|---|---|---|---|---|
| B1 | `etl fetch` 2023-07..2025-12 + manifest + sha check; `etl archive` same day | T1 | FQ | Mismatch fails; snapshot release exists |
| B2 | Parse, normalize, RUC 20 filter, exclusion log | B1, C1 | FQ | Fixture tests |
| B3 | Join, céntimos, window by dateSigned, non-PEN exclusion | B2 | FQ | Golden window test |
| B4 | `etl sanctions` | T1 | FQ | Test with known rows |
| B5 | `etl build` + `index.json` + `etl report` (coverage number) | B3, B4, C2 | PI | One command; report committed |
| B6 | Demo set (30–50 real RUC 20) + **Empresa Demo** synthetic huella | B5 | PX | `data/out/demo/README.md` with reasons |
| B7 | `etl publish` release + `etl reproduce` | B5 | IN | Web build pulls it; reproduce matches attested root |
| B8 | 15 min: check CONOSCE for a penalidades dataset | — | PI | Note in okf/log.md |
| B9 | 2–3 interviews (caja/cooperativa analyst, factoring fintech, MYPE) with one quote each | — | PI | `docs/validation.md` |
| B10 | README "Problema" and "Datos, licencias y limitaciones" | B5, B9 | PI | Merged by D |
| B11 | `/api/search` over index | B5, C1 | PX | Top 10 by RUC or name |
| B12 | USDC: confirm testnet issuer/faucet; `lib/payments`: trustline tx, disbursement tx (memo hash = root) | A6 | US | Both txs succeed from a test wallet |
| B13 | `/api/payments/:account` linked-disbursement reader | B12 | US | Lists only payments whose memo is a root of that subject |

# C — Core, integration, video edit
| ID | Task | Dep | Crit | Acceptance |
|---|---|---|---|---|
| C1 | pnpm workspace, TS, vitest, lint, Actions (cargo + pnpm + web build) | T1 | FQ | CI green |
| C2 | Core types + `buildHuella(window)` + metrics (document/meta split) | C1 | FQ | Golden + order + meta tests |
| C3 | `canonicalRoot` (Web Crypto), `subjectKey`, float guard, `MockRegistry` | C1 | IN | Vectors committed; tamper test |
| C4 | `/api/huella` from published release | C2, B7 | FQ | Returns real huella |
| C5 | `/api/attest`, `/api/revoke` (single, retry once) | A4 | US | Attest a demo RUC from curl |
| C6 | Verdict mapping from `status` (5 kinds) shared by web and tools | C3, A4 | IN | Unit test per kind |
| C7 | `tools/verify`: standalone verifier (no server) + API tests + Playwright E2E | C6, D4 | FQ | In CI; README command works |
| C8 | Video edit: rough cut Dec 2, final Dec 3 | D9 script | PX | Team approves |

# D — Web and story
| ID | Task | Dep | Crit | Acceptance |
|---|---|---|---|---|
| D1 | Wireframes `/`, `/h/[ruc]`, `/verificar`, `/operador` | T3 | PX | Approved Nov 26 |
| D2 | **Hello-world on Vercel Nov 26** (with C), deploy on every merge | C1 | FQ | Public URL works |
| D3 | Huella page (metrics, proxy, signals, tooltips, contracts → OECE, share, QR, JSON) | D2, C2 | PX | Works on fixture |
| D4 | Verify page (link/JSON → C6 verdict, 5 banners + subtitles, explorer) | D2, C6 | IN | Tamper → ALTERADA |
| D5 | Wallet UI: connect, Friendbot prompt, network check, **request claim**, pending/approved | D3, A7 | US | Claim from UI with Empresa Demo |
| D6 | Disbursement UI: trustline button (MYPE), send button (lender), linked list | D5, B12, B13 | US | Payment appears linked |
| D7 | Operator page | D2, C5, A7 | FQ | Attest, approve, revoke work |
| D8 | All states + copy rules + attribution footer | D3–D7 | PX | Each state reachable |
| D9 | Video script and screen recordings (script Nov 30) | D5 | PX | Script approved |
| D10 | README assembled + APEX submission by Dec 4 12:00 + Demo Day slides | A9, B10, C7 | FQ | APEX confirmation |

# Stretch (only if Gate 1 passes)
S2 Merkle proof per contract (C). S3 batch verification (D).

# Critical paths
1. T1 → C1 → C3 → C6 → D4 → Gate 1 → C8 (video).
2. A1 → A2 → A4 → A7 → D5 → D6 (with B12) → Gate 1 → C8.
3. B1 → B5 → B7 → C4 (real data in the deployed app).
