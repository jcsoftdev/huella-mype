---
type: Plan
title: Timeline, gates and risk register (Nov 25 – Dec 8)
description: Day-by-day plan with go/no-go gates; a missed gate cuts stretch, never the video or README.
tags: [timeline, risks]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Days
| Day | A | B | C | D |
|---|---|---|---|---|
| Wed 25 | Kickoff T3, T4; A1 | T4; B1 (fetch + archive), B8 | T4; C1 | T4; D1 |
| Thu 26 | A2 | B2, B4; book B9 | C2, C3 · pair C+D: D2 hello-world deploy | D2, D3 start |
| Fri 27 | A3, A4 · **pair A+B: claim/USDC CLI spike (A6)** | B3 · pair B+C | C3 done | D3 |
| Sat 28 | A5, A7 · pair A+C | B5, B11 | C5, C6 | D4 · **lane demos** |
| Sun 29 | **Mob integration:** search → huella → attest → status → verify on the deployed app | ← | ← | ← |
| Mon 30 | A7 with D (pair) | B6, B7, B12, B9 | C4, C7 | D5, D7 · **video script (D9)** |
| **Tue Dec 1 — Gate 1** | A8 | B13 · pair B+D | C7 | D6, D8 |
| Wed 2 | A9; fixes | B10; S2/S3 only if Gate 1 passed | **C8 rough cut** | D8, D10 README draft |
| Thu 3 | freeze | freeze | C8 final | D10 · **everyone runs the demo (Gate 2)** |
| **Fri 4** | A8 keep-alive check | — | — | **Submit APEX by 12:00**; then protect main, disable auto-deploy |
| Sat 5 | Demo Day | | | |
| Dec 5–8 | A8 check Dec 6; no deploys | | | |

# Gates
- **Gate 1 (Dec 1, 18:00):** on the deployed app — real huella VERIFICADA, tamper ALTERADA, Empresa Demo claim approved, one linked disbursement shown; CI green. If not: drop stretch; all hands on the missing piece.
- **Gate 2 (Dec 3, 20:00):** video final, README complete, every member ran the demo. If not: Dec 4 morning only for these.

# Risk register
| Risk | Mitigation | Owner |
|---|---|---|
| OECE blocks or changes files | Browser UA; archive snapshot the day of fetch; fallback OCP Registry yearly file | B |
| Claim or USDC flow harder than expected | CLI spike Fri 27; two-step claim avoids auth-entry co-signing; fallback asset if Circle testnet USDC is unavailable | A, B |
| Juror claims a real company | Operator approval required; only Empresa Demo auto-approves | A |
| Hash mismatch browser vs server | One core package, vectors in CI, `meta` not hashed | C |
| Late deploy surprises (monorepo, env, file tracing) | Hello-world Nov 26, deploy on every merge | D, C |
| Testnet reset or broken deploy during judging | Check reset calendar; freeze deploys Dec 4–8; keep-alive check | A, D |
| No interviews | Start B9 Nov 26; fallback: cite sources and competitor gap | B |
| Lane A person unavailable | C reviews A throughout and has key access | C |
| Scope creep | Gate 1 controls stretch | C |
