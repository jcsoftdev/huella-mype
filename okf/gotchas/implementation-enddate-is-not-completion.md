---
type: Gotcha
title: implementation.endDate and finalValue do not mean "completed"
description: Reading them as delivery date or final payment invents a fulfillment metric the data does not have.
tags: [ocds, metrics]
resource: ../docs/research/research-ocds-data.md
sources:
  - resource: ../data/fixtures/sample_record.json
    title: "finalValue +14.8% over signed value"
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Symptom
`implementation.endDate` looks like a completion date, `implementation.finalValue` like the amount paid.

# Why
In 85 contracts with endDate, it was never earlier than `period.endDate` (median +45 days): it most likely records an extended deadline. `finalValue` appears only when it differs from the signed value (155 higher, 33 lower, 0 equal): additions or deductions. Neither is documented in the OECE data dictionary. Semantics UNVERIFIED.

# Fix
Report them as "contracts with term extensions" and "contracts with value changes", neutral in tone (an adicional can be legal and normal). See [decision](../decisions/compliance-signals-not-completion-rate.md).

# How it was found
Observed by the research agent with curl on 2026-11-24 while mapping the OECE API; details and raw samples in the [OCDS research](../../docs/research/research-ocds-data.md).
