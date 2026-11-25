---
type: Plan
title: Team lanes, ownership and workflow
description: Four lanes that never edit each other's files, joined by frozen seams, with pairing so everyone understands the whole system.
tags: [team, workflow, git]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Team
Lanes are assigned at kickoff; each lane is described well enough for anyone to pick it. **Recommended:** lane A goes to whoever is strongest in Rust/backend.
| GitHub | Name | Lane |
|---|---|---|
| jcsoftdev | Juan Carlos | TBD |
| taniausca | Tania Usca | TBD |
| tatianaticona05 | Tatiana | TBD |
| geraldin21l | Geraldine (invite pending) | TBD |

# Lanes
| Lane | Scope | Scores mainly | Ideal profile | Tasks |
|---|---|---|---|---|
| **A — Stellar** | Contract (status, revoke, two-step claim), deploy, keys, claim approval API, chain scripts | Uso de Stellar | Rust or willing to learn, backend | 9 |
| **B — Data, payments, evidence** | ETL, publish, search, demo company, **USDC trustline + disbursement (F7)**, interviews | Problema, Uso de Stellar | TypeScript, data, talks to users | 13 (several light) |
| **C — Core, integration, video edit** | Hashing, verdicts, attest/huella API, CI, standalone verifier, merges; edits the video from Dec 1 | Funcionalidad, Innovación | Solid TypeScript, organised | 8 |
| **D — Web and story** | Pages, UX states, wallet UI, deploy, README, video script and screens, APEX | Presentación, Problema | React/Next.js, design, storytelling | 10 |

# Ownership (only the owner edits; see paths in [architecture](../specs/architecture.md))
- One owner per shared file: root `package.json`/`pnpm-workspace.yaml` → C; `apps/web/package.json`, `next.config`, `layout`, `lib/`, `.env.example` → D, with C as mandatory reviewer.
- `pnpm-lock.yaml`: anyone may change it; on conflict, rebase and regenerate, never hand-edit.
- Secrets: A holds admin and attester keys, shared with C via the team password manager; D sets Vercel Production env with A present.

# Seams (frozen at kickoff; change = PR approved by every affected owner)
| Seam | Producer | Consumers | Stand-in |
|---|---|---|---|
| [Huella v1](../specs/huella-document-v1.md) + core functions | C | B, D | types day 1; fixture huella |
| [Registry](../specs/registry-contract.md) client | A | B, C, D | `MockRegistry` in core |
| [API routes](../specs/api-and-web.md) | A, B, C | D | static JSON from fixtures |
| Published huellas [pipeline](../specs/data-pipeline.md) | B | C, D | demo set |

# Git
Branch `<lane>/<task-id>-<slug>`; never push to main; PR < ~300 lines; Conventional Commits; CI green; one approval from the assigned reviewer; squash merge; rebase every morning. Commit from your own account every working day.

# Reviewers (cross-lane)
| Author | Reviewer until Nov 28 | Reviewer from Nov 29 |
|---|---|---|
| A | C | C (A's backup must know Rust: always C) |
| B | D | A |
| C | A | B |
| D | B | C |

# Everyone knows 100% — pair at every seam
Driver writes, navigator reviews live, swap halfway. Each member co-builds at least two parts outside their lane.
| Date | Pair | Built together |
|---|---|---|
| Wed Nov 26 | C + D | Hello-world deploy + core types wired in the web app |
| Thu Nov 27 | B + C | ETL producing huellas through core (B5 with C2) |
| Fri Nov 27 | A + B | Claim + trustline + payment round trip on testnet with the CLI (spike) |
| Sat Nov 28 | A + C | Attest API + `status` against the deployed contract |
| Mon Nov 30 | A + D | Claim request/approve in the UI with a wallet |
| Tue Dec 1 | B + D | Disbursement UI and linked payments list |

# Rituals
Standup 15 min daily (merged, next, blocked, seam changes). Lane demos Sat Nov 28 (10 min each). Mob integration Sun Nov 29 (search → huella → attest → verify; claim and payments join Mon–Tue). Final check Dec 3: everyone runs the full demo from the README and explains the architecture slide.

# Done (per task)
Acceptance observed, tests in CI, package README updated, OKF concept if a non-obvious why appeared, reviewed, merged, deployed.
