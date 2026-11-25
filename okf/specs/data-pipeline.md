---
type: Spec
title: Data pipeline — from OECE bulk files to published huellas
description: Lane B builds exactly this; it feeds core, the deployed app and the demo.
tags: [etl, ocds, data]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Commands (packages/etl)
| Command | Does | Output |
|---|---|---|
| `etl fetch --from 2023-07 --to 2025-12` | Download `/api/v1/file/seace_v3/json/{Y}/{M}/` with a browser User-Agent; get `/sha/{Y}/{M}/`; recompute; fail on mismatch | `data/raw/{Y}-{M}.json`, `data/raw/manifest.json` (sha256, fetch date = snapshot) |
| `etl archive` | Upload the raw ZIPs and manifest **the day they are fetched** as release `snapshot-<date>` | GitHub release asset |
| `etl sanctions` | Download sanctions CSV (latin1, `;`), sha256 | manifest entry |
| `etl build --window 2024-01-01:2025-12-31` | Parse → normalize → RUC 20 filter → join → céntimos → core `buildHuella` per RUC | `data/out/huellas/{ruc}.json`, `data/out/index.json` |
| `etl report` | Records per month, excluded ids by kind, non-PEN, **number of RUC 20 suppliers with contracts in window** (for README/video) | `data/out/report.md` (committed) |
| `etl publish` | Pack `huellas/` + `index.json` as release `huellas-<snapshot>` | GitHub release asset read by the web build |
| `etl demo` | Copy the curated real set + generate **Empresa Demo** synthetic huella | `data/out/demo/` (committed) |
| `etl reproduce --ruc X` | Rebuild one huella from the archived snapshot and print its root | stdout |

# How the deployed app gets data
The web build (Vercel) downloads the latest `huellas-<snapshot>` release asset into the build output; `/api/search` and `/api/huella` read from it. The repo commits only fixtures, the demo set and the report. ([decision](../decisions/huellas-published-as-release-asset.md))

# Rules
- Never query the records API per RUC ([gotcha](../gotchas/oece-api-ignores-ruc-filter.md)); always send a UA ([gotcha](../gotchas/oece-api-403-without-browser-user-agent.md)).
- Supplier id must match `^PE-RUC-20\d{9}$` ([gotcha](../gotchas/consortium-ids-are-not-rucs.md)).
- `implementation.*` are neutral signals ([gotcha](../gotchas/implementation-enddate-is-not-completion.md)).
- Pin and archive the snapshot ([constraint](../constraints/attestation-pins-source-snapshots.md)).
- Fixtures contain no RUC 10 names or ids (redacted); new fixtures must be redacted the same way.
- Tests run on fixtures only.
