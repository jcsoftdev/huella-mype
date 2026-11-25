---
type: Spec
title: Huella document v1 — schema, metrics, hashing, verdicts
description: The seam between data, core, web and chain; frozen at kickoff, any change is schema v2.
tags: [schema, hashing, metrics]
resource: ../docs/research/research-ocds-data.md
sources:
  - resource: ../data/fixtures/sample_record.json
    title: Golden test input
  - resource: ../docs/research/research-stellar-tech.md
    title: JCS hashing notes
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# What is hashed and what is not
A huella file has two parts. Only `document` is hashed and attested; `meta` is never hashed, so rebuilding from the same pinned files always gives the same root.
```jsonc
{
  "document": {
    "schema": "huella-mype/v1",
    "subject": { "ruc": "20604357951", "legalName": "EXAMPLE S.A.C." },
    "window": { "from": "2024-01-01", "to": "2025-12-31" },
    "source": {
      "dataset": "OECE SEACE v3 OCDS", "license": "CC-BY-4.0",
      "publisher": "https://contratacionesabiertas.oece.gob.pe",
      "snapshot": "2026-11-26",
      "files": [ { "period": "2023-07", "sha256": "<inner JSON sha256>" } ],
      "sanctions": { "sha256": "<csv sha256>", "asOf": "2025-10-15", "license": "ODC-By" }
    },
    "metrics": {
      "contractsCount": 12, "awardsCount": 14,
      "totalSignedAmountPenCents": 224166835,
      "distinctBuyers": 5,
      "firstContractDate": "2024-02-10", "lastContractDate": "2025-11-22",
      "awardToContractRateBps": 8571,
      "valueChanges": { "contractsWithFinalValue": 2, "medianChangeBps": 1030 },
      "extensions": { "contractsWithEndDate": 1, "avgExtensionDays": 45 },
      "sanctions": { "listedInCurrentList": false, "contractTerminatedByFault": false },
      "excluded": { "nonPenContracts": 0 }
    },
    "contracts": [ { "ocid": "...", "contractId": "...", "awardId": "...", "buyerId": "...", "buyerName": "...",
                     "dateSigned": "2024-02-10", "amountPenCents": 1000000, "method": "...", "category": "works",
                     "finalValuePenCents": null, "implementationEndDate": null } ]
  },
  "meta": { "builtAt": "2026-11-26T15:00:00Z", "builder": "huella-mype etl <git sha>" }
}
```
`snapshot` is the date the pinned files were fetched (recorded in the manifest), so it is stable for a given snapshot. Contracts sorted by (ocid, contractId). Metrics without data are `null`, never 0.

# Window and input range
Window = contracts **signed** 2024-01-01 to 2025-12-31. OECE files are grouped by **tender start month** ([gotcha](../gotchas/bulk-files-grouped-by-tender-start-month.md)), so the ETL fetches tender months **2023-07 to 2025-12** and filters by `dateSigned`. `buildHuella` takes the window as a parameter.

# Metrics (paths in `records[].compiledRelease`)
Supplier = `awards[].suppliers[0].id` matching `^PE-RUC-20\d{9}$`; contract joined to award by `contracts[].awardID == awards[].id`.
| Metric | Rule | Path |
|---|---|---|
| contractsCount | contracts with dateSigned in window | `contracts[].dateSigned` |
| awardsCount | awards in window | `awards[].date, suppliers[].id` |
| totalSignedAmountPenCents | Σ PEN values × 100, rounded once at ingestion; non-PEN → excluded | `contracts[].value` |
| distinctBuyers | distinct buyer ids | `buyer.id` |
| first/lastContractDate | min/max dateSigned | — |
| awardToContractRateBps | floor(contracts ÷ awards × 10 000) | — |
| valueChanges | count with finalValue; median (final ÷ signed − 1) in bps | `implementation.finalValue` |
| extensions | count with implementation.endDate; floor(mean days after period.endDate) | `implementation.endDate`, `period.endDate` |
| sanctions.listedInCurrentList / contractTerminatedByFault | RUC in current-sanctions CSV / INFRACCION contains "resuelva el contrato" | sanctions CSV |
The size proxy ("posible MYPE") is computed in the UI from `totalSignedAmountPenCents`; it is not part of the document.

# Hash, subject, schema number
- `root = sha256(utf8(JCS(document)))` — one TS implementation ([decision](../decisions/typescript-single-hashing-implementation.md)); integers only ([constraint](../constraints/money-and-rates-are-integers.md)).
- `subject = sha256(utf8("huella-mype:subject:v1:" + ruc))` ([decision](../decisions/subject-key-is-domain-separated-ruc-hash.md)).
- On-chain `schema: u32`: **1 = huella-mype/v1**.

# Verdicts (one call: contract `status(subject, root)`)
| Verdict | Condition | Subtitle |
|---|---|---|
| VERIFICADA | root is the latest version, not revoked | Documento íntegro y atestado; los datos vienen de OECE |
| DESACTUALIZADA | root is an older non-revoked version | Existe una huella más reciente |
| REVOCADA | root's version is revoked | El emisor revocó esta versión |
| ALTERADA | subject has versions, root matches none | El documento fue modificado |
| NO ATESTADA | subject has no versions | Esta empresa aún no tiene huella atestada |

# Test vectors (packages/core, committed)
- Golden: `data/fixtures/sample_record.json` with window 2024-01-01..2024-12-31 → expected metrics and root for supplier 20604357951 (its contract was signed 2024-11-22).
- Tamper (+1 céntimo → different root); order independence; `meta` changes do not change the root; filters (RUC 10, consortia, `PE-RUC-L`); subject key; float guard.
