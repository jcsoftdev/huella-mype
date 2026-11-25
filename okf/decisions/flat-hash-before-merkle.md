---
type: Decision
title: Flat JCS hash for the MVP; Merkle root only as stretch
description: Merkle proofs add a second proof format, tests and UI for little judging value in ten days.
tags: [hashing, scope]
resource: ../docs/research/research-stellar-tech.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why flat hash
One root over the canonical document is enough for integrity and the tamper demo, and keeps one verification path to test.

# Rejected (for now): Merkle root over contracts
Would let a lender verify a single contract without seeing the rest — a real privacy feature. It needs domain-separated leaf hashing (e.g. OpenZeppelin merkle-tree), inclusion-proof UI and more vectors. Do it only after the definition of done is met on Dec 1; it would bump the schema to v2.
