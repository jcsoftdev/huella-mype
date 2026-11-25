---
type: Gotcha
title: OECE API returns 403 to default HTTP clients
description: Scripts look broken or blocked unless they send a browser-like User-Agent.
tags: [ocds, etl, http]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Symptom
`curl https://contratacionesabiertas.oece.gob.pe/api/v1/` → HTTP 403. Node `fetch` with no UA behaves the same.

# Why
The server filters by User-Agent. With `-A 'Mozilla/5.0'` every data endpoint returns 200. No documented rate limit (UNVERIFIED), so throttle politely.

# Fix
Always send `User-Agent: Mozilla/5.0 (compatible; huella-mype-etl)` and cache downloads in `data/raw/`. The old host `contratacionesabiertas.osce.gob.pe` no longer resolves (OSCE → OECE, Ley 32069).

# How it was found
Observed by the research agent with curl on 2026-11-24 while mapping the OECE API; details and raw samples in the [OCDS research](../../docs/research/research-ocds-data.md).
