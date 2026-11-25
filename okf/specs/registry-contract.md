---
type: Spec
title: Registry contract — interface, storage, events, claim
description: The Soroban API every lane codes against; frozen at kickoff.
tags: [soroban, contract]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
sources:
  - resource: ../docs/research/research-stellar-tech.md
    title: Design rationale
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Interface
| Function | Auth | Returns | Effect |
|---|---|---|---|
| `__constructor(admin)` | — | — | Stores admin |
| `set_attester(attester, allowed)` | admin | — | Allow-list (disabling does not invalidate what it already issued; revoke per version) |
| `attest(attester, subject, root, schema)` | attester, allow-listed | `u32` version | Appends `Att(subject,n)`, writes `RootIdx(subject, root) = n`; event `Attested` |
| `revoke(caller, subject, version)` | caller = admin **or** the attester that issued it | — | Marks revoked; extends TTL; event `Revoked` |
| `request_claim(subject, claimant)` | claimant | — | Stores `PendingClaim(subject) = claimant` (overwrites a previous pending one); requires ≥ 1 attestation; event `ClaimRequested` |
| `approve_claim(attester, subject, claimant)` | attester, allow-listed | — | Requires pending == claimant and no approved claim; stores `Claim(subject)`; event `ClaimApproved` |
| `revoke_claim(caller, subject)` | admin or attester | — | Removes claim; event `ClaimRevoked` |
| `status(subject, root)` | — | `Status` | One-call verdict (see below) |
| `count`, `get`, `latest`, `claimed_by`, `pending_claim` | — | views | |
| `verify(subject, root)` | — | `bool` | `status == Verified` (kept for simple integrations) |
`Attestation { root, schema, attester, ledger_time, revoked }`.
`Status { kind: StatusKind, version: u32, latest: u32, attestation: Option<Attestation>, claimant: Option<Address> }`, `StatusKind = Verified | Outdated | Revoked | Altered | NotAttested`.
Errors: `NotAttester=1, NotFound=2, AlreadyClaimed=3, NotAttested=4, NoPendingClaim=5, NotAuthorized=6`.

Why a two-step claim and not one transaction with both signatures: a single tx needs the server to sign an auth entry, re-simulate and race an expiry while a human signs; two independent transactions keep two signers (the multi-party point) with ordinary wallet flows, and the operator approval stops anyone from claiming a company that is not theirs ([decision](../decisions/mype-claim-in-mvp.md)).

# Storage and TTL
Admin in `instance`; `Attester`, `Count`, `Att`, `RootIdx`, `PendingClaim`, `Claim` in `persistent`; never `temporary`. **Every write**, including revoke and claims, extends touched entries and the instance ([constraint](../constraints/extend-ttl-on-every-write.md)).

# Events
`#[contractevent]`: `Attested{subject(topic), version, root}`, `Revoked{subject(topic), version}`, `ClaimRequested{subject(topic), claimant}`, `ClaimApproved{subject(topic), claimant}`, `ClaimRevoked{subject(topic)}`.

# Tests (cargo test, snapshots committed)
Attest/versioning and RootIdx; unauthorized attester; revoke by admin, by issuing attester, rejected for another attester; status for all 5 kinds; set_attester(false) blocks new attests only; request_claim needs an attestation; approve_claim needs matching pending; second approval fails; revoke_claim; TTL extended on every write.

# Deploy and config
```bash
stellar keys generate huella-admin --network testnet --fund
stellar keys generate huella-attester --network testnet --fund
stellar contract build
stellar contract deploy --wasm target/wasm32v1-none/release/registry.wasm \
  --source huella-admin --network testnet -- --admin $(stellar keys address huella-admin)
stellar contract invoke --id $ID --source huella-admin --network testnet -- set_attester \
  --attester $(stellar keys address huella-attester) --allowed true
npx @stellar/stellar-sdk generate --contract-id $ID --network testnet --output-dir packages/registry-client --contract-name registry
```
Apps read `REGISTRY_CONTRACT_ID` from env and pass it to the client, so a redeploy does not require regenerating bindings unless the interface changed. Key custody: A holds admin and attester secrets; both are also stored in the team's shared password manager (A and C have access). Prototype without claim/status: `spikes/registry-prototype`.
