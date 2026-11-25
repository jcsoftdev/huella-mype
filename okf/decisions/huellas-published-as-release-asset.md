---
type: Decision
title: All huellas are published as a GitHub release asset read at build time
description: Jurors must find any RUC 20 company, not just the 50 demo ones, without filling the repo with generated JSON.
tags: [data, deploy]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why
`etl publish` packs every huella plus `index.json` into a release `huellas-<snapshot>`; the Vercel build downloads it. Search and huella pages then cover every RUC 20 supplier in the window, and the repo history stays readable. The raw OECE files go to a separate `snapshot-<date>` release the same day, which makes `etl reproduce` possible after OECE rebuilds its files.

# Rejected alternatives
- **Commit everything to the repo:** tens of thousands of generated files drown the code the jury reads and bloat clones.
- **Database (Supabase/Neon):** another service, credentials and failure point for static, read-only data.
- **Demo set only:** a juror typing any other RUC gets "not found".
