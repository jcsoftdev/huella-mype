---
type: Gotcha
title: "Monthly file sizes vary a lot; that is seasonality, not missing data"
description: "A small month (2025-01: 1,314 records) looks like data loss but is not; do not spend time chasing it."
tags: [ocds, data-quality]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Symptom
`2025-01_seace_v3.json` has 1,314 records while the yearly index implies about 6,600 processes per month.

# Why
Files are grouped by tender start month ([gotcha](bulk-files-grouped-by-tender-start-month.md)) and procurement is seasonal: 2025-06 has 5,910 records and 2024-12 has 8,126. January is a slow month for new tenders.

# Fix
`etl report` still prints records per month and the yearly totals versus `indexCountData`, so a real gap would show at year level.

# How it was found
Research agent (2026-11-24) noticed the low January count; the plan audit (2026-11-25) compared other months and found the seasonal pattern.
