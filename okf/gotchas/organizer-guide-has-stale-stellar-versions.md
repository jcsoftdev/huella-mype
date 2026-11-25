---
type: Gotcha
title: The organizer guide mixes outdated Stellar versions and paths
description: Copying its snippets fails on SDK 30 / CLI 30.
tags: [stellar, setup]
resource: ../docs/research/research-stellar-tech.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Symptom
- Guide pins `soroban-sdk = "26"`; current is 30.0.0.
- `SetUp_Dev.md` installs target `wasm32v1-none` but deploys from `target/wasm32-unknown-unknown/release/…` → "file not found".
- `stellar contract bindings typescript` prints a deprecation notice.
- Old snippets use `SorobanRpc` and `env.events().publish`.

# Fix
Use `stellar contract build` and `target/wasm32v1-none/release/*.wasm`; generate bindings with `npx @stellar/stellar-sdk generate`; use `rpc.Server` and `#[contractevent]`.

# How it was found
Comparing the organizer repo (github.com/Gabrululu/Stellar-Build-PE) with crates.io, the stellar-cli docs and a live testnet `getNetwork` call on 2026-11-24; see the [Stellar research](../../docs/research/research-stellar-tech.md).
