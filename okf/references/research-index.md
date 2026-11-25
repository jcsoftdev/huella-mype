---
type: Reference
title: Research index and key numbers
description: Where the sourced research lives and the figures the story depends on.
tags: [research, sources]
resource: ../docs/research/research-ocds-data.md
sources:
  - resource: ../docs/research/research-stellar-tech.md
    title: Stellar stack research
  - resource: ../docs/research/research-market.md
    title: Market research
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Files
- [OECE OCDS data](../../docs/research/research-ocds-data.md) — live-tested endpoints, field coverage measured on a full month, metric paths, sanctions data, personal-data analysis.
- [Stellar / Soroban stack](../../docs/research/research-stellar-tech.md) — versions, contract design, frontend/backend, hashing, pitfalls.
- [Market and prior art](../../docs/research/research-market.md) — MYPE credit gap, procurement, Ley 32069, factoring, lenders, competitors.
- `data/fixtures/` — real OECE samples for tests. `spikes/registry-prototype/` — contract prototype (5 native tests pass, soroban-sdk 30).
Every claim there has a URL; unfetched claims are marked UNVERIFIED.

# Numbers for README and video
| Fact | Value | Source file |
|---|---|---|
| Formal Mipyme with formal credit | 27.4% | market |
| State factoring share of the factoring market | < 4% (MEF, Aug 2026) | market |
| OECE dataset | 2.77 M processes, 506 k suppliers, 1.34 M contracts since 2003 | ocds |
| 2025 | 79,397 processes, 61,633 contracts, 37,390 suppliers | ocds |
| Not covered | purchases ≤ 8 UIT (≈24% of 2024 amount, UNVERIFIED) | market |

# Verified during the plan audit (2026-11-25)
- Testnet RPC answers CORS with `access-control-allow-origin: *`: the browser reads the contract directly.
- OECE files are grouped by tender start month (2024-12 holds 4,792 contracts signed in 2025).
- The fixtures were redacted: no RUC 10 ids or names remain.

# Competitors
Cautela (cautela.pe) and ContrataMYPE (GitHub) read procurement data. None issues a tamper-evident, independently verifiable credential. No evidence that lenders already use procurement history: B9 interviews test this.
