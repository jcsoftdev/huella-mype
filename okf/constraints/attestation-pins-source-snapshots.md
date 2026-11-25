---
type: Constraint
title: Every huella pins the sha256 of the OECE files it came from
description: OECE rebuilds bulk files daily; without pinned hashes and archived copies nobody can re-derive an attested root.
tags: [provenance, ocds, reproducibility]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# What must hold
`source.files[]` lists each monthly file with the inner-JSON sha256 published at `/api/v1/file/seace_v3/sha/{YYYY}/{MM}/` (recompute locally and compare), plus `retrievedAt`. We archive the exact files used as a `snapshot-<date>` GitHub release **the same day they are fetched**, so the provenance check is possible after OECE changes them. Build timestamps live in `meta`, outside the hashed `document`, so a rebuild from the same snapshot gives the same root.

# What breaks if not
Bulk ZIPs are regenerated (Last-Modified moved between Nov 22 and 24) and records can be revised. Re-running the ETL next week on fresh files may yield a different root, and we could no longer show the attested document derives from public data — the provenance half of the pitch collapses.
