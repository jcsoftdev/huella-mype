---
type: Decision
title: One TypeScript package computes metrics and hashes everywhere
description: ETL, API and browser share packages/core so the root can never differ between issuer and verifier.
tags: [hashing, architecture, typescript]
resource: ../docs/research/research-stellar-tech.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
The browser must recompute the root for the lender. If the ETL were Python and the browser TypeScript, two JCS implementations would have to agree byte for byte, and a mismatch shows up as "ALTERED" on a valid huella — the worst possible demo failure. One package, one implementation (`canonicalize` 5.1.0), one set of golden vectors.

# Rejected alternative: Python ETL (pandas)
Faster for data exploration and some teammates may prefer it. Cost of rejecting: B writes the ETL in Node; 23–130 MB JSON per month is fine for `JSON.parse`. Python stays fine for ad-hoc analysis notebooks that do not produce attested output.
