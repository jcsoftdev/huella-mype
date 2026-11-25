---
type: Gotcha
title: OECE records API silently ignores a RUC filter
description: A per-supplier query looks like it works but returns the same unfiltered page.
tags: [ocds, api]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Symptom
`/api/v1/records?ruc=20604357951` returns 200 with 20 records that have nothing to do with that RUC.

# Why
The OpenAPI only supports `page, order, sourceId, startDate, endDate, dataSegmentationID, tenderId, ocid, tenderTitle, mainProcurementCategory`; unknown parameters are dropped. `dataSegmentationID=2025-01` (tender start month) does work.

# Fix
Per-RUC history comes from the monthly bulk files ([decision](../decisions/offline-monthly-bulk-etl.md)).

# How it was found
Observed by the research agent with curl on 2026-11-24 while mapping the OECE API; details and raw samples in the [OCDS research](../../docs/research/research-ocds-data.md).
