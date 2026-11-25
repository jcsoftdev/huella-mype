---
type: Decision
title: Build huellas offline from monthly bulk files, not live API calls
description: The OECE API has no supplier filter, so per-RUC history is only possible from bulk files.
tags: [etl, ocds, data]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
`GET /api/v1/file/seace_v3/json/{YYYY}/{MM}/` gives a whole month (2–13 MB zipped) with a verifiable inner sha256 at `/sha/{YYYY}/{MM}/`. The records API pages 20 at a time and silently ignores `ruc=`. A batch ETL is also reproducible (pinned files), which the provenance story needs.

# Rejected alternative: query the API live from the panel
Impossible per RUC without scanning everything, slow, and the panel would break whenever OECE is down. Backup if OECE blocks us: OCP Data Registry `2025.jsonl.gz` (149 MB, monthly snapshot).
