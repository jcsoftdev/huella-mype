---
type: Constraint
title: Nothing personal or identifying goes on-chain
description: Ledger data is permanent and public; one leaked name or RUC 10 cannot be removed.
tags: [privacy, soroban]
resource: ../spikes/registry-prototype/src/lib.rs#Registry
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# What must hold
On-chain state and events contain only: subject key (hash), root (hash), schema number, attester address, ledger time, revoked flag, and the claimant's Stellar account address (a pseudonymous key the MYPE chose to link, see [claim decision](../decisions/mype-claim-in-mvp.md)). Never names, RUCs, amounts, buyers or contract ids. Note: an EIRL's legal name often contains a person's name, so a linked account can point to a person off-chain; that is why the claim is opt-in.

# What breaks if not
Stellar storage and transaction history are public and immutable. A name or DNI-bearing RUC 10 written once cannot be erased, which violates the product promise, the pitch, and potentially Ley 29733; EDPB Guidelines 02/2025 recommend exactly this commitment-only pattern. Review every `attest` argument and event field in PRs.
