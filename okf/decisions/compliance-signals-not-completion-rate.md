---
type: Decision
title: Report "señales de cumplimiento", never a completion rate
description: The OCDS data has no contract status, payments, milestones or penalties; claiming "% completed on time" would be false.
tags: [metrics, data-quality, pitch]
resource: ../docs/research/research-ocds-data.md
sources:
  - resource: ../data/fixtures/sample_record.json
    title: "Real record: finalValue and two awards, one contract"
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
Measured on the full 2025-01 bulk (1,089 contracts): contract status 0%, `implementation.endDate` 7.8%, `implementation.finalValue` 17%, transactions/milestones/amendments/penalties never present. So we report volume, breadth, award-to-contract conversion, disclosed value/term changes and presence in the current Tribunal sanctions list (snapshot to 2025-10, shown as "no figura en la lista vigente", never "limpio"), each with explicit `null` when absent.

# Rejected alternative: a single fulfillment percentage or score
It is what lenders would like, and it would make a punchier pitch. It cannot be computed honestly from this data, a juror who knows SEACE would catch it, and it turns the product into an unregulated credit score. Cost: a less flashy headline number.
