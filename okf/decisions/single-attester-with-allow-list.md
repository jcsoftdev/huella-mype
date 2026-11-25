---
type: Decision
title: One attester key in the demo, but an allow-list in the contract
description: Keeps the demo simple while showing the path to OECE, gremios or auditors as independent issuers.
tags: [soroban, trust, auth]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
The oracle question ("who attests?") is the first thing a juror asks. The contract supports many attesters via an admin-managed allow-list with `require_auth`, so the pitch can say "today our pipeline signs; tomorrow OECE or a MYPE association can be added as an attester without redeploying". The demo uses one server-held key. An attester can revoke what it issued; disabling an attester stops new attestations but does not invalidate past ones (revoke per version).

# Rejected alternative: hard-coded single issuer
Simpler by a few lines, but makes the trust model look final and centralised, and costs points on Uso de Stellar.
