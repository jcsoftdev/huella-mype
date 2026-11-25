---
type: Product
title: HuellaMYPE — product definition
description: What we build, for whom, and the exact MVP scope; anything not listed here is not built.
tags: [product, scope, mvp]
resource: ../docs/research/research-market.md
sources:
  - resource: ../docs/research/research-ocds-data.md
    title: What the data supports
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# One line
**HuellaMYPE** convierte el historial público de contratos con el Estado de una empresa en una **huella verificable**: un documento con sus contratos, montos y señales de cumplimiento, cuyo hash queda atestado en Stellar, que la empresa reclama con su cuenta y que un prestamista verifica y puede usar para un desembolso vinculado.

# Problem
Many MYPE are denied formal credit for lack of history (27.4% of formal Mipyme have formal credit). Those that sell to the State already have an official, public record in SEACE, published by OECE as OCDS open data (CC BY 4.0), but it is spread across monthly files no analyst reads, and a company cannot prove a summary it shows is authentic. Factoring of State invoices is under 4% of the factoring market even though Ley 32069 lets MYPE issue negotiable invoices to the State.

# Users
| Persona (illustrative) | Job to be done |
|---|---|
| **Rosa**, owner of a small RUC 20 company that supplies municipalities | Prove what she has delivered to the State when asking for working capital |
| **Luis**, credit analyst at a caja municipal / cooperativa | Confirm in one minute the record is authentic and see its source |
| **A factoring fintech** | Check a supplier's State track record before buying its invoices |

# Who is covered
RUC 20 suppliers with signed contracts in the window. "MYPE" is not a field in OCDS, so the panel shows a **size proxy**: "posible MYPE" when total signed amount in the window is ≤ 1,700 UIT per year (the small-enterprise sales ceiling; UIT value to confirm at implementation). Larger suppliers are shown as "proveedor RUC 20" without the label.

# MVP (everything else is out)
| ID | Capability |
|---|---|
| F1 | Search by RUC or name → huella: contracts, total signed amount, buyers, first/last contract, compliance signals, completeness notes, size proxy, each contract linked to OECE |
| F2 | Attestation of the huella root on Stellar; versioned; revocable by the issuing attester or admin |
| F3 | **Claim**: the company requests with its wallet (its signature); the operator approves (attester signature). Shown as "cuenta vinculada" |
| F4 | Share: link, QR, JSON download |
| F5 | Verify: link or JSON → browser recomputes root → one contract call → **VERIFICADA / DESACTUALIZADA / REVOCADA / ALTERADA / NO ATESTADA** with version, date, attester, linked account, explorer links |
| F6 | Operator console: attest one, approve claims, revoke |
| F7 | **Linked disbursement (assets + payments)**: the claimed account adds a USDC trustline; the lender sends testnet USDC with memo = huella root; the huella and verify pages list "desembolsos vinculados" |
| F8 | **Demo sandbox**: a synthetic company "Empresa Demo S.A.C." (clearly labeled, fake RUC, synthetic data) whose claims are auto-approved so anyone, including jurors, can try claim + disbursement end to end |
| F9 | Standalone verifier: one script/HTML file in the repo that verifies a huella JSON against the contract with no dependency on our server |
Stretch (after Gate 1 only): S2 Merkle proof per contract; S3 batch verification.

# Out of scope (README "Limitaciones")
RUC 10 personas naturales; credit scoring; proof of legal ownership of a RUC (the claim proves an approved link to an account; lenders still do KYC); purchases ≤ 8 UIT (not in OCDS); consortium members; mainnet; real money.

# Honest framing
- "Evidencia de contratos con el Estado", "señales de cumplimiento"; never "score".
- The history is a floor, not the full record.
- VERIFICADA means "documento íntegro y atestado; los datos vienen de OECE", shown as the subtitle.
- Sanctions: "No figura en la lista de sancionados vigentes (corte 2025-10)", never "limpio".
- Attribution: OECE / SEACE (CC BY 4.0) and the sanctions dataset (ODC-By) in footer, README and every huella page.

# Related
[Demo and video](demo-and-video.md) · [Huella spec](../specs/huella-document-v1.md) · [Claim decision](../decisions/mype-claim-in-mvp.md) · [Payments decision](../decisions/payments-and-assets-in-mvp.md) · [Demo company decision](../decisions/demo-company-is-synthetic.md)
