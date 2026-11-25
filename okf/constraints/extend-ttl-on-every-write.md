---
type: Constraint
title: Extend TTL on every write and run keep-alive before judging
description: Archived persistent entries make verify return false on the day jurors test the app.
tags: [soroban, ttl, ops]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# What must hold
Every write — attest, revoke, claim request/approval/revocation — extends the TTL of the entries it touches and of the instance. With entries bumped to ~120 days the demo outlives judging; the keep-alive script is a cheap check on Dec 4 and Dec 6, and the bigger risks (testnet reset, broken deploy) are handled in the [timeline](../plans/timeline.md).

# What breaks if not
Persistent entries past their TTL are archived. Simulated reads can restore them only when the caller builds a restore, and a plain `verify` from the panel may fail or return false — a valid huella shows NOT ATTESTED in front of the jury. Testnet max TTL is UNVERIFIED; check with `stellar network settings`.
