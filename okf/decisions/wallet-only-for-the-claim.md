---
type: Decision
title: Wallets only for actions that move state; verifying never needs one
description: The most-tried flow, verification, has zero wallet friction.
tags: [ux, stellar]
resource: ../docs/research/research-stellar-tech.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
Verification is a read (`status` via simulation), so lenders and jurors install nothing to verify. A wallet appears only where a signature is the point: the company requesting a claim and enabling USDC, and the lender sending a disbursement.

# Rejected alternative: wallet login for everyone
Adds steps and failure modes (no extension, wrong network, unfunded account) to the flow a juror is most likely to try.
