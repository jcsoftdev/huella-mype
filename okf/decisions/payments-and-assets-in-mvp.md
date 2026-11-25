---
type: Decision
title: Linked USDC disbursement is in the MVP
description: Without payments and assets the heaviest criterion is only half covered, and the story stops before the credit.
tags: [stellar, payments, assets, scope]
resource: ../docs/research/research-stellar-tech.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
"Uso de Stellar" (25%) lists contracts, payments, assets and accounts. After a claim, the company's account opens a trustline to testnet USDC (asset, account) and the lender sends a testnet USDC disbursement whose memo hash is the huella root (payment). The panel lists "desembolsos vinculados", closing the story from verified history to credit on the same network. It uses classic operations, no new contract code.

# Rejected alternatives
- **Keep it as stretch:** the audit showed stretch work scheduled late is what gets cut; the criterion would stay half covered.
- **A loan or escrow contract:** a real lending product, far beyond ten days, and close to "promised returns" territory the bases exclude.
Cost accepted: B takes payments (B12, B13), D adds two buttons; Circle testnet USDC availability must be confirmed in the Fri 27 spike, with a clearly named test asset as fallback. Real money is out of scope.
