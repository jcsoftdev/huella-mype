---
type: Decision
title: Target soroban-sdk 30 and stellar-cli 30, not the guide's v26
description: Testnet runs protocol 30; SDK 30 contracts need CLI ≥ 30 and Rust ≥ 1.91.
tags: [stellar, versions]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
soroban-sdk 30.0.0 is current (crates.io, 2026-11-24) and testnet answered protocol 30 on `getNetwork`. The prototype already compiles and passes tests on it.

# Rejected alternative: follow the organizer guide (`soroban-sdk = "26"`)
Matches the tutorials the mentors reference, but is four majors behind; mixing guide snippets with SDK 30 causes API mismatches (events macro, bindings command). See [gotcha](../gotchas/organizer-guide-has-stale-stellar-versions.md).
