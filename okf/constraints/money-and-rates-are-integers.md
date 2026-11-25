---
type: Constraint
title: Money is integer céntimos and rates are integer basis points in the huella
description: A float in the document makes JCS output differ across runtimes and turns valid huellas into "ALTERED".
tags: [hashing, schema]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# What must hold
Every amount in the hashed document is an integer number of céntimos (`Math.round(amount * 100)` once, at ingestion); every rate is integer bps; dates are ISO strings. No floats anywhere in `metrics` or `contracts`.

# What breaks if not
JCS serializes numbers with ECMAScript rules; floats like 2241668.35 or derived ratios can serialize differently after arithmetic in a different order, so ETL and browser compute different roots. The lender sees ALTERED on an untouched huella. The golden vector test must fail if anyone introduces a float.
