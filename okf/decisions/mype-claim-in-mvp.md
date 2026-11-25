---
type: Decision
title: Two-step, operator-approved claim is in the MVP
description: It adds accounts and consent to the Stellar score while stopping anyone from claiming a company that is not theirs.
tags: [soroban, scope, consent, security]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
The Stellar criterion names "cuentas". A claim links the company's own account to its huella with two signers: the company requests (`request_claim`, its signature) and our attester approves (`approve_claim`). Approval is a human decision in `/operador`, so a juror or anyone else cannot squat a real company's huella. The synthetic Empresa Demo auto-approves so the flow can be tried by anyone.

# Rejected alternatives
- **One transaction with both signatures (server-signed auth entry):** looks more elegant, but needs auth-entry signing with an expiry, re-simulation after signing, and an auto co-sign that anyone could abuse. More work, worse security.
- **Auto-approve every claim:** the multi-party signature becomes decorative and real companies can be squatted.
- **No claim:** leaves "cuentas" unused and no consent.
Limitation stated: approval proves we linked that account, not legal ownership of the RUC; lenders still do KYC. An EIRL's legal name often contains a person's name, so a linked account can identify a person: the claim is opt-in for that reason.
