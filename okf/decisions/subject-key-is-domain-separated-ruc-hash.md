---
type: Decision
title: Subject key = sha256("huella-mype:subject:v1:" + RUC)
description: Lets any lender verify directly against the chain without asking our server for a secret.
tags: [hashing, privacy, verification]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
sources:
  - resource: ../docs/research/research-stellar-tech.md
    title: "Section 2.1 subject-key tradeoff"
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
The product's claim is "verify without trusting us". With a plain, domain-separated hash of the RUC, the lender recomputes the subject from the RUC printed in the huella and queries the contract directly. Since the MVP covers only RUC 20 (public company identifiers, already listed as State suppliers in open data), the hash being enumerable reveals nothing that OECE does not already publish.

# Rejected alternatives
- **Secret pepper held by our backend**: hides RUCs from enumeration, but then only our server can derive the key, so verification depends on our API — the exact trust we want to remove.
- **Per-MYPE random salt shared with the lender**: private and direct, but adds salt management and a second thing to share for a benefit RUC 20 does not need. Revisit for RUC 10 in a later version. The research files suggest a salted key; this decision supersedes that for RUC 20.
