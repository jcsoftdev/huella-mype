---
type: Gotcha
title: 'PE-RUC-16xxxxx supplier ids are consortia, not RUCs'
description: Treating them as RUCs creates fake companies and wrong totals.
tags: [ocds, data-quality]
resource: ../docs/research/research-ocds-data.md
sources:
  - resource: ../data/fixtures/sample_record.json
    title: "Real record with PE-RUC supplier ids"
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Symptom
Supplier ids in the `PE-RUC-` namespace with 7 digits (about 271 in 2025-01), named "CONSORCIO …", plus odd prefixes (15, 17, 18, 99) and `PE-RUC-L…`.

# Why
SEACE assigns internal ids to consortia and some foreign suppliers and OECE publishes them under the same scheme label.

# Fix
Accept only `^PE-RUC-20\d{9}$`. Count excluded ids in ETL logs. A consortium's members are not attributed in the MVP (a known gap: MYPE often bid in consortia).

# How it was found
Observed by the research agent with curl on 2026-11-24 while mapping the OECE API; details and raw samples in the [OCDS research](../../docs/research/research-ocds-data.md).
