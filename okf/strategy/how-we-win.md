---
type: Strategy
title: How we win — the jury scores only repo, testnet and video
description: The page every member reads first; every task exists to raise one of these five scores.
tags: [strategy, judging]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
sources:
  - resource: ../docs/research/research-market.md
    title: Problem evidence
  - resource: ../docs/research/research-ocds-data.md
    title: Data limits
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# The rule that drives everything
The jury scores **three artifacts only**: the public GitHub repo, the testnet deployment, and a video of at most 3 minutes. The Demo Day pitch scores nothing. Each criterion is scored 0–5 and weighted. Work that leaves no visible trace in those three artifacts earns zero points.

# Scorecard
| Criterion | Weight | A 5/5 shows | Our evidence | Owner |
|---|---|---|---|---|
| Uso de Stellar | 25 | Stellar is essential and uses **contracts, accounts, payments and assets** well | Soroban registry (constructor, allow-listed attesters, versioning, revocation, root index with a one-call `status`, events, TTL). **Accounts:** the MYPE claims its huella with its own account (request by MYPE, approval by attester: two signers). **Assets + payments:** the claimed account opens a USDC trustline and the lender sends a testnet USDC disbursement whose memo is the huella root, so the panel shows "desembolsos vinculados a esta huella". Everything linked on stellar.expert | A (contract, claim), B (payments) |
| Funcionalidad y calidad técnica | 25 | Works on testnet at first click; clean, tested, documented | Live URL deployed since day 1, contract ID, CI badge with counts, package READMEs, standalone verifier script, architecture diagram, this bundle | C |
| Problema e impacto | 20 | Real problem, named user, evidence the user wants it | Two lender types (caja/cooperativa analyst, factoring fintech) and the MYPE owner; sourced figures; "N empresas RUC 20 firmaron contratos con el Estado en 2024–2025" computed by our ETL; 2–3 real interviews quoted | B, D |
| Innovación | 15 | An approach jurors have not seen | Official procurement data → portable, tamper-evident credential; verifiable **without going through our server** and **reproducible from the official files**; zero personal data on-chain; credential → linked disbursement on the same network | C, D |
| Presentación y UX | 15 | Clear 3-minute video; app usable without instructions | Spanish UI, three flows (MYPE, analista, demo sandbox), one-click verdict, tamper demo, scripted video | D (script, screens), C (edit) |

# The story in one breath
"Una empresa que le vende al Estado ya tiene historial, pero quien presta no lo ve. HuellaMYPE lo convierte, desde los datos abiertos oficiales de OECE, en una huella que la empresa reclama con su cuenta Stellar, que cualquier prestamista verifica sin pasar por nuestro servidor y que puede recibir un desembolso vinculado, sin datos personales en la cadena."

# Wording rules (a crypto-literate juror will check)
- Say "verificable sin pasar por nuestro servidor y reproducible desde los archivos oficiales", never "sin confiar en nosotros": we are the attester.
- Say "proveedores RUC 20"; "MYPE" only with the size proxy shown (see [product](../product/product-definition.md)).
- Personas Rosa and Luis are **illustrative** and only appear with the synthetic demo company, never with a real company's name.
- Say "evidencia" and "señales", never "score" or "% cumplimiento".

# What would cost points
Overclaims above; a credit score; speculative tokens; a wallet prompt to read data; a broken link on Dec 5–7; showing a real company as "sanctioned" on video.

# Read next
[Product](../product/product-definition.md) → [Team and workflow](../plans/team-and-workflow.md) → [Backlog](../plans/backlog.md) → [Timeline](../plans/timeline.md) → your lane's spec.
