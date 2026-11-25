# Research: Peru public-procurement open data (OCDS) for the MYPE track-record product

Date of research: 2026-11-24. All HTTP results below were obtained by actually fetching with curl (browser User-Agent) on that date. Items I could not verify are marked UNVERIFIED. Raw samples are in `data/fixtures/ (subset; bulk files not committed) `.

## 0. TL;DR

- The data is live, free and keyless: `https://contratacionesabiertas.oece.gob.pe/api/v1/...` (200 OK, JSON, OCDS 1.1) plus monthly bulk ZIPs. License CC BY 4.0. Updated daily.
- Requests with the default curl User-Agent to `/api/v1/` root got 403; with `-A 'Mozilla/5.0'` every data endpoint returned 200. Always send a browser-like UA.
- Reliable fields per supplier: RUC, name, award amount/date, contract value, dateSigned, contract period, buyer, procurement method. 
- Fulfillment signals are SPARSE and indirect. There is NO contract status (terminated/cancelled) in bulk data, NO transactions, NO milestones, NO penalties inside OCDS. What exists: `contracts[].implementation.endDate` (8% of contracts), `implementation.finalValue` (17%, always different from the signed value), and award-without-contract / re-award patterns. Sanctions (Tribunal) are a separate CSV keyed by RUC.
- Recommendation: use the monthly bulk JSON ZIP (about 2-13 MB each, one month of tender start = one file), pre-process offline into a per-RUC (RUC 20 only) history, hash that, attest on Soroban. Do not call the API live from the dashboard.

## 1. Where Peru publishes OCDS data

| Source | URL | Status on 2026-11-24 |
|---|---|---|
| OECE open-contracting portal (SPA, Angular) | https://contratacionesabiertas.oece.gob.pe/ | 200, 2.8 KB shell (title "Portal de Contrataciones Abiertas de la Compra Pública de Perú", author OECE) |
| API v1 root | https://contratacionesabiertas.oece.gob.pe/api/v1/ | 403 with plain curl; individual endpoints 200 with browser UA |
| Old domain | https://contratacionesabiertas.osce.gob.pe/ | no response (curl code 000, DNS/connection failure) |
| OCP Data Registry, Peru OECE | https://data.open-contracting.org/en/publication/135 | 200; dateModified 2026-10-04, coverage 2003-01-01/2026-10-03, retrieved monthly (Kingfisher Collect) |
| OCP Registry, PeruCompras (framework agreements) | https://data.open-contracting.org/en/publication/78 | appeared in search; not fetched. UNVERIFIED |
| Plataforma Nacional de Datos Abiertos, OECE group | https://www.datosabiertos.gob.pe/group/organismo-especializado-para-las-contrataciones-p%C3%BAblicas-eficientes-oece | 200 |
| OECE data dictionary (PDF) | https://contratacionesabiertas.oece.gob.pe/downloads/Diccionario%20de%20datos%20OCDS.pdf | 200, 69.9 KB |
| Publication policy (PDF) | https://contratacionesabiertas.oece.gob.pe/downloads/politica_publicacion.pdf | 200, 498 KB, 14 pages |

### OSCE to OECE rename
Ley 32069 (Ley General de Contrataciones Publicas), Vigesima Tercera Disposicion Complementaria Final: every reference to OSCE "debe entenderse como" OECE. Law text (gob.pe PDF): https://cdn.www.gob.pe/uploads/document/file/7594705/6444155-ley-general-de-contrataciones-publicas-con-modificaciones-posteriores-hasta-el-11-12-2024.pdf (fetched, grep-confirmed). The publication policy says Ley 32069 applies from 2025-04-22 (policy PDF, section on legal concepts). The portal moved to `*.oece.gob.pe`; the `osce.gob.pe` portal host does not answer. Practical impact: older blog posts, GitHub repos and tutorials with `osce.gob.pe` or `contratacionesabiertas.gob.pe` URLs are stale. Note that the portal's own embedded Swagger spec still lists server `//contratacionesabiertas.gob.pe/api/v1` but the front-end overrides it with the oece host at runtime (read from the JS bundle). The OCDS JSON still has `publisher.name = "Organismo Supervisor de las Contrataciones del Estado"` (old name). The Tribunal de Contrataciones del Estado is now "Tribunal de Contrataciones Publicas" (same law).

### Endpoints I called (all 200, application/json unless noted)
Base: `https://contratacionesabiertas.oece.gob.pe/api/v1/`

| Endpoint | Size | Notes |
|---|---|---|
| `files` | 3.5 KB per page | Lists bulk files, 326 total (66 pages of 5), newest first, with links for csv, csv_es, xlsx, xlsx_es, json, sha |
| `file/{source}/{type}/{year}/{month}/` | 2.3-12.6 MB | Bulk ZIP. source = seace_v3 or seace_v2; type = json, csv, xlsx, sha (+ `es` suffix for Spanish headers) |
| `releases?page=N` | 77 KB per 20 | releasePackage, 20 per page |
| `records?page=N` | 405 KB per 20 | recordPackage with compiledRelease, 20 per page |
| `records?dataSegmentationID=2025-01&page=1` | 20 records | Filter works (year-month of tender start) |
| `release/seace_v3/{tenderId}` | 38 KB | e.g. 999983 gives all 44 releases of one process |
| `indexCountData` | 2.4 KB | Counts by year (see section 3) |
| Other (from JS bundle, not all tested): `record/{ocid}`, `records`, `search`, `buyers`, `entities`, `process`, `processHistory`, `contractsSuppliersTop10Dashboard`, `recordsSuppliersTop10Dashboard` | | UNVERIFIED individually |

Query parameters for /records and /releases (from embedded OpenAPI): `page, order, sourceId, startDate, endDate, dataSegmentationID, tenderId, ocid, tenderTitle, mainProcurementCategory`. The doc says the search pagination is "from/size" for under 10,000 results, so the API cannot enumerate everything. There is NO filter by supplier RUC: `records?ruc=...` returned 200 with the same 405 KB unfiltered page (parameter silently ignored). Therefore per-supplier history must come from bulk files.

Bulk file integrity: `sha/2025/01/` returns a 64-hex SHA-256 of the inner JSON (`2025-01_seace_v3.json`, 23,602,906 bytes). I recomputed it locally: 960520200ee4809a... matches. The ZIP itself has sha256 f8e4f5bd3f08... (this will change on each daily rebuild; ZIP Last-Modified was 2026-11-22). Bulk ZIPs are rebuilt (Last-Modified varies 22-24 Nov), so the hash of a bulk file is not stable across days, which matters for the attestation design (hash the derived per-RUC history, not the raw file).

### Alternative: OCP Data Registry (download URLs, HEAD-verified)
- `https://data.open-contracting.org/en/publication/135/download?name=full.jsonl.gz` 1,968,276,530 bytes (about 1.97 GB)
- `...name=2025.jsonl.gz` 148,719,573 bytes; `...name=2025.csv.tar.gz` 196,958,802 bytes (each 302 then 200)
- `...name=2025.xlsx` 404 (no xlsx for that year in my test; xlsx exists for some years per page links, e.g. 1900/2003/2004)
- Snapshot dated 2026-10-04, monthly refresh, so it is older than the OECE portal (daily). Good as backup if the OECE API is down. The registry's own list of known quality issues (page text, last reviewed Dec 2023): duplicate tenderer identifiers; some contracts without status; undocumented documentType codes; awards not linked to contracts and contracts not linked to awards.

## 2. What the data actually contains (verified on real samples)

Samples saved:
- `ocds-sample/releases_p1.json` (20 releases, 77 KB), `ocds-sample/records_p1.json` (20 records, 405 KB)
- `ocds-sample/sample_record.json` (one full record, ocid `ocds-dgv273-seacev3-2024-424-9`, 44 releases, 90 KB; award + contract + implementation)
- `ocds-sample/bulk_2025-01/2025-01_seace_v3.json` (1,314 records, 23.6 MB, the analysis corpus below)
- `ocds-sample/sanc.csv` (Tribunal sanctions snapshot)

Structure: recordPackage with `records[].compiledRelease` plus `releases[]` (history; tags progress planning, tender, award, contract, implementation). OCDS version 1.1. ocid format `ocds-dgv273-seacev3-<year>-<entity>-<n>` (prefix registered with OCP on 2022-01-31 per the publication policy). Extensions declared include contract_completion, itemstatus, exchangeRate, datasegmentation (partitions by tender start year-month), currencyname, department.

### Example (real) of the key path set, from sample_record.json
- `buyer.id = "PE-CONSUCODE-424"`, name MUNICIPALIDAD DISTRITAL DE ALTO SELVA ALEGRE; in `parties[]` the buyer also has `additionalIdentifiers[{scheme:"PE-RUC", id:"20311886879"}]`
- `tender.procurementMethod = "open"`, `tender.procurementMethodDetails = "Adjudicación Simplificada"`, `tender.mainProcurementCategory = "works"`, `tender.value.amount` (and `amount_PEN`)
- `awards[]`: two awards for one tender (first awardee ARCONST ... 2024-06-28, second FAPECONS ... 2024-10-30). Only the second has a contract. This is the "buena pro" lost or not signed pattern.
- `contracts[0]`: `awardID`, `title`, `value.amount 2,241,668.35 PEN`, `dateSigned 2024-11-22`, `period {startDate 2024-11-23, endDate 2025-05-21, durationInDays 179}`, `implementation.finalValue.amount 2,572,653.56` (+14.8% vs signed value)
- `parties[]`: supplier = `{id:"PE-RUC-20604357951", identifier:{scheme:"PE-RUC", id:"20604357951", legalName}, roles:["tenderer","supplier"], contactPoint:{email:"Ver datos en Ficha del Proveedor"}}` (contact data is masked by the publisher)

### Field population in the 2025-01 bulk file (1,314 records = processes whose tender started Jan 2025)
Measured by script over the whole file:

| Path | Result |
|---|---|
| records with award and contract | 1,026 (78%); awards with no contract 68 (5%); no award at all 220 (17%) |
| `awards[].suppliers[].id`, `awards[].value.amount` | present when award exists (a few awards lack `suppliers`: my first script crashed on one) |
| `contracts[]` total | 1,089; every one has awardID, title, period.startDate/endDate, value, dateSigned, documents |
| `contracts[].period.durationInDays` | 1,068/1,089 (98%) |
| `contracts[].status` / `statusDetails` | **0 of 1,089** in bulk 2025-01. In the first 20 API records: 3 of 13 contracts had `status:"active", statusDetails:"EJECUCION"`. Never terminated/cancelled seen. The publication policy itself lists "contracts without status" as a known data-quality issue |
| `contracts[].implementation` | 227 of 1,089 (21%): `finalValue` only 142, `endDate` + `finalValue` 46, `endDate` only 39 |
| `implementation.endDate` | 85 of 1,089 (7.8%). vs `period.endDate`: 74 later (median +45 days, mean 62.5, max 365), 11 equal, **0 earlier**. Never earlier suggests it records the extended end (ampliacion de plazo) rather than the real completion date. The dictionary does not document it. Semantics UNVERIFIED |
| `implementation.finalValue` | 188 contracts (17%): 155 higher, 33 lower than signed value, 0 equal. Median +10.3%. It appears only when the value changed (adenda, adicional, deductivo). Semantics not documented. UNVERIFIED |
| `implementation.transactions`, `implementation.milestones`, `amendments` | **absent** (never present) |
| Penalties (penalidades), resolution of contract, liquidation | **absent from OCDS**. Not even in the OECE data dictionary (grep for penal/liquid/resoluci/adenda in the dictionary text gave nothing) |
| Contract documents | types: contractSigned (1,341), contractAnnexe (652); tender docs: biddingDocuments, awardNotice, evaluationReports, clarifications. Document URLs exist; some documentType missing (305) |
| `awards[].items[].statusDetails` | CONTRATADO 1,121, CONSENTIDO 66, ADJUDICADO 55 (the last = award with no signed contract yet) |
| currency | PEN 1,277, USD 33, EUR 4 (`tender.value.amount_PEN` helps normalise) |
| methods | Adjudicación Simplificada 628, Subasta Inversa Electrónica 228, Convenio 97, Contratación Directa 80, Comparación de Precios 74, Licitación Pública 73, Concurso Público 68 ... |
| ocids with 2+ awards | about 70 (e.g. 34 with 2 awards/1 contract) |

Supplier identifier forms seen in `awards[].suppliers[].id` (1,098 supplier role parties):
- RUC 20xxxxxxxxx (companies) 652; RUC 10xxxxxxxxx (natural persons) 207; **PE-RUC-16xxxxx (7-digit, consortia: "CONSORCIO ...")** about 271+; other odd prefixes (17, 99, 15, 18) a handful; ids like `PE-RUC-L...` 28.
- Consortia use internal short ids in the `PE-RUC-` namespace that are NOT real RUCs. They must be excluded or treated separately.
- Supplier `name` for RUC 10 is the person's full name (e.g. "[full personal name, redacted]"), i.e. personal data.

### Reliability summary
Reliably populated: ocid, buyer id/name, tender title/category/method, award supplier RUC+name+date+amount (when award exists), contract value, dateSigned, period start/end, documents.
Sparse: contract status (0%), implementation.endDate (8%), finalValue (17%), number of tenderers (5/20 in releases sample).
Never populated: transactions, milestones, amendments, penalties, termination reason.
Coverage limit: only procurements with a selection procedure; below 3/8/9 UIT purchases (ordenes de compra directas, the typical MYPE micro-purchases) are NOT included (publication policy, "Alcance de los datos"; also the OCP registry description). Framework-agreement purchase orders (Peru Compras) are a separate dataset.

### Other open datasets usable as a fulfillment signal
1. **Providers sanctioned by the Tribunal (inhabilitados)**, Plataforma Nacional de Datos Abiertos: https://www.datosabiertos.gob.pe/dataset/proveedores-sancionados-2025 . CSV downloaded (931 KB, 2,414 rows, latin1, semicolon). Columns: `INH;NRO;RAZON_SOCIAL;RUC;RESOLUCION;PERIODO_INH;DESDE;HASTA;INFRACCION;OTRA_INFRACCION`. Counts: Temporal 1,320, Definitivo 1,094; RUC prefix 20: 1,801, 10: 587. 169 rows cite the infraction "ocasionar que la Entidad resuelva el contrato" (contract terminated for provider fault), which is a true non-fulfillment signal. Caveats: the dataset page lists publisher "Gobierno Regional de La Libertad" (not OECE), license "Open Data Commons Attribution License", frequency quarterly, modified 2025-12-30, and the newest DESDE in the file is 2025-10-15, so it is stale by about 13 months. Direct CSV URL: `https://www.datosabiertos.gob.pe/sites/default/files/Relaci%C3%B3n%20de%20proveedores%20sancionados%20por%20el%20Tribunal%20de%20Contrataciones%20P%C3%BAblicas%20con%20sanci%C3%B3n%20vigente_0.csv`.
2. **OECE/RNP live lookup** (last five years of sanctions, HTML form by name/razon social): https://www.rnp.gob.pe/consultasenlinea/inhabilitados/busqueda_vnv.asp (200, 327 KB). Official and fresher, but a search form, not a bulk dataset. Scraping feasibility UNVERIFIED.
3. **CONOSCE open data portal** (bi.seace.gob.pe, Pentaho): https://bi.seace.gob.pe/pentaho/api/repos/:public:portal:datosabiertos.html/content?userid=public&password=key returns 200 but the page is rendered by JavaScript; I could not enumerate datasets. A search snippet said OECE open data has sections on sanciones, inhabilitaciones y penalidades. Contents UNVERIFIED. Worth 15 minutes for one teammate to open in a browser and check for a penalidades dataset.
4. Monthly Excel "otros datos de contrataciones" (PAC, ordenes de compra, contratos, proveedores) mentioned in the publication policy; URL not captured. UNVERIFIED.

## 3. Volume, frequency, license

From `GET /api/v1/indexCountData` (2026-11-24): 2,772,237 ocids total; 3,318 buyers; 506,181 suppliers; 2,021,536 awards; 1,336,776 contracts. Per year (tender-start year):

| Year | ocids | awards | contracts | suppliers |
|---|---|---|---|---|
| 2022 | 79,127 | 64,495 | 65,711 | 34,778 |
| 2023 | 76,632 | 62,621 | 67,561 | 32,423 |
| 2024 | 80,261 | 70,446 | 66,594 | 36,643 |
| 2025 | 79,397 | 67,306 | 61,633 | 37,390 |
| 2026 (to date) | 68,829 | 58,880 | 50,642 | 33,376 |

Files: 326 monthly bulk entries (SEACE v2 2003-2017 loaded 2023-12-06; SEACE v3 2013-present, rebuilt on a rolling basis). Size examples (Content-Length): 2025-01 JSON ZIP 2.36 MB (23.6 MB unzipped); 2024-03 JSON 12.6 MB; 2026-11 JSON 5.8 MB; CSV 2025-01 2.6 MB; XLSX 5.1 MB. Note the 2025-01 JSON has only 1,314 records versus about 6,600 ocids per month implied by the yearly index (UNVERIFIED reason; maybe partial month or rolling rebuild). Check record counts before relying on a month being complete.

Update frequency: the policy says the ETL runs daily from 08:15 Lima time (GMT-5) and refreshes both the API and bulk files. The `files` timestamps (2026-11-24) are consistent.

License: CC BY 4.0 (policy PDF "Licencia" section; `license` field in every package: https://creativecommons.org/licenses/by/4.0/ ; OCP registry confirms). Allows commercial use; requires attribution/link to source. Suggested attribution text (our wording): "Contains public procurement data from the Organismo Especializado para las Contrataciones Públicas Eficientes (OECE), SEACE, published at https://contratacionesabiertas.oece.gob.pe under CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/). Data was filtered and aggregated by this project; changes were made." The sanctions CSV is under a different license (Open Data Commons Attribution License) and needs its own attribution. The OECE policy also carries a disclaimer: data comes from SEACE as-is and OECE does not modify errors in source data.

## 4. Personal data concerns

- RUC 10 = persona natural. Per SUNAT guidance (search result summary; the SUNAT page itself did not render text in my curl, so UNVERIFIED directly) a RUC 10 is "10 + DNI (8 digits) + check digit". So publishing a RUC 10 effectively publishes the DNI. In the OCDS data, `name` and `legalName` for those are the person's full name.
- Ley 29733 art. 2 numeral 4: personal data is "toda información sobre una persona natural que la identifica o la hace identificable". Art. 2 numeral 11 defines "fuentes accesibles para el público". Art. 14 lists limits to consent (public-source data and data on creditworthiness "conforme a ley" are among them). Text fetched from https://cdn.www.gob.pe/uploads/document/file/272360/Ley%20N%C2%BA%2029733.pdf.pdf . New regulation DS 016-2024-JUS (published 2024-11-30, in force from about 2025-03-30 per secondary sources, UNVERIFIED on official text): https://lpderecho.pe/reglamento-ley-proteccion-datos-personales-decreto-supremo-016-2024-jus/ . I am not a lawyer; the point is that "it is public" does not automatically permit a new commercial profile with a new purpose (credit scoring for lenders). Treat as a risk item for the pitch, not a blocker for a testnet hackathon.
- Law 27806 (transparency) is the OECE's stated basis for publication (policy PDF).

Recommendation:
1. Scope the product to **RUC 20 only** (companies: S.A.C., E.I.R.L., S.R.L. etc.). This removes DNI exposure. Many MYPE are RUC 10 sole proprietors, so say so openly as a limitation and offer a "phase 2: explicit consent from the RUC 10 holder (sign with their own Stellar key)" path.
2. On-chain: store only `hash = SHA-256(canonical JSON of the per-RUC aggregate + schema version + period)`, ideally salted-hash of the RUC as the subject key (`H(salt || RUC)`, salt known to lender at verification time) so the chain is not a lookup table of who sells to the State. A plain RUC 20 is public business data and low risk, but a salted key also blocks bulk enumeration.
3. Off-chain panel: show company legal name and aggregates for RUC 20; for RUC 10 show nothing or masked ("persona natural, no disponible").
4. Never ingest contact points, addresses of persons, or document URLs for RUC 10.
5. Exclude consortia ids (PE-RUC with 7 digits / not 11 digits): they are not RUCs.

## 5. Concrete recommendation

### Dataset and fetch plan (offline, 4 people)
1. Primary: OECE monthly bulk JSON, `GET https://contratacionesabiertas.oece.gob.pe/api/v1/file/seace_v3/json/{YYYY}/{MM}/` (send a browser UA). Verify with `.../sha/{YYYY}/{MM}/` (SHA-256 of the inner JSON file; verified for 2025/01).
2. Hackathon subset: 12 months 2025-01 to 2025-12 is about 12 files of roughly 2-13 MB (about 40 to 80 MB zipped, estimate). Faster for a demo: 3 months (e.g. 2025-01, 2025-02, 2025-03). Region filter is not available in the API; derive region from `parties[buyer].address.department` (present on all buyers) and filter by buyer department (e.g. one department such as LAMBAYEQUE or AREQUIPA) in your own ETL. The 2025 contracts hold implementation info that has had time to mature; 2026 data is still mostly in execution.
3. Backup if the OECE API blocks us: OCP Registry `2025.jsonl.gz` (149 MB, snapshot 2026-10-04).
4. Cache the downloads in the repo's data folder (git-ignored) and commit a tiny fixture (e.g. the 20-record sample) for automated tests so tests do not depend on the network.
5. Fulfillment enrichment: join the sanctions CSV on RUC (plus optional manual check against RNP).

### Proposed metric definition (per supplier RUC 20; paths are in `records[].compiledRelease`)
Unit of analysis: a signed contract, i.e. `contracts[]` joined to `awards[]` by `contracts[].awardID == awards[].id`; supplier = `awards[].suppliers[0].id` with prefix `PE-RUC-` + 11 digits starting with 20.

| Metric | Definition | OCDS paths | Reliability |
|---|---|---|---|
| contracts_count | count of contracts with `dateSigned` | `contracts[].id, dateSigned` | high |
| awards_count | count of awards | `awards[].id, suppliers[].id` | high |
| total_signed_amount_pen | sum of contract value, converted to PEN (amount if currency=PEN; else exclude or use exchange rate) | `contracts[].value.amount, value.currency` | high (96% PEN) |
| distinct_buyers | count distinct buyers | `buyer.id` | high |
| first_contract_date / last_contract_date | min/max dateSigned | `contracts[].dateSigned` | high |
| award_to_contract_rate | contracts / awards for that supplier (an award with no signed contract = lost or unsigned buena pro) | `awards[].id` vs `contracts[].awardID`; `awards[].items[].statusDetails` in {CONTRATADO, CONSENTIDO, ADJUDICADO} | medium; ADJUDICADO may simply be pending |
| value_change_rate | share of contracts with `implementation.finalValue` and mean (finalValue / value - 1) | `contracts[].implementation.finalValue.amount` | medium-low: only 17% have it, meaning unknown, cannot read as "bad" (can be a legal adicional) |
| extension_rate and avg_extension_days | share of contracts with `implementation.endDate` and mean(endDate - period.endDate) | `contracts[].implementation.endDate`, `period.endDate` | low: 8%; semantics UNVERIFIED (never earlier than period end) |
| sanction_flag | RUC present in Tribunal sanctions CSV with DESDE in the window; sub-flag if INFRACCION mentions "resuelva el contrato" | CSV `RUC, DESDE, HASTA, INFRACCION` | medium; snapshot stale to 2025-10 |
| status_counts | counts of `contracts[].status` where present (EJECUCION etc.) | `contracts[].status, statusDetails` | very low: 0% in bulk; report only as "n/a" |

**Honest framing for the pitch:** do NOT claim "% completed on time". The open data cannot support that. What it supports is a verifiable record of (a) volume and amounts adjudicated and signed, (b) breadth (buyers, years), (c) signed-contract conversion, (d) disclosed value/term adjustments, (e) absence/presence in the sanctions list. Suggested name for the fulfillment component: "senales de cumplimiento" (compliance signals), with explicit nulls where data is missing, and a data-completeness field per metric. Optional composite "track record tier" (e.g. none / emerging / established) computed only from contracts_count, distinct_buyers, years active, and sanction_flag; keep the formula pure and unit-tested.

### Attestation payload suggestion (supports the on-chain design)
`{ schema:"huella-mype/v1", subject: H(salt||RUC), window:{from,to}, source:{dataset:"OECE seace_v3", files:[{name,sha256_inner}], retrievedAt}, metrics:{...above, with nulls}, sanctionsSnapshot:{sha256, asOf} }`; canonicalise (sorted keys, integers as céntimos to avoid float drift) then SHA-256; the contract stores `(subjectHash, historyHash, issuer, issuedAt, schemaVersion)`. The lender recomputes from the same published files and compares. Caveat to design for: OECE rebuilds bulk files daily and can revise records, so the attestation must pin the file SHA-256 values and the retrieval date, and the verifier needs the exact derived JSON (store it off-chain in the panel's database, with downloadable proof).

## 6. Risks / open items
- API access blocked for default curl UA (403): use browser UA; throttle politely (no documented rate limit; UNVERIFIED).
- The 2025-01 bulk has fewer records than the yearly index implies; validate counts per month.
- implementation.* semantics undocumented; do not over-interpret.
- Sanctions CSV is stale (to 2025-10) and published by a regional government account, not OECE.
- RUC 10 and consortia are not cleanly handled by the data; scope to RUC 20.
- No MYPE flag in OCDS. Identifying "MYPE" would require REMYPE (Ministerio de Trabajo) or SUNAT size data, not verified here; for the hackathon define the audience as "suppliers to the State identified by RUC 20" and optionally label size by contract amounts. UNVERIFIED availability of REMYPE open data.
- Bulk ZIP URLs are not versioned; capture hashes at fetch time.

## 7. Sources (all fetched unless noted)
- https://contratacionesabiertas.oece.gob.pe/ and /api/v1/{files,releases,records,indexCountData,release/seace_v3/999983}
- https://contratacionesabiertas.oece.gob.pe/api/v1/file/seace_v3/{json,csv,xlsx,sha}/2025/01/
- https://contratacionesabiertas.oece.gob.pe/downloads/politica_publicacion.pdf
- https://contratacionesabiertas.oece.gob.pe/downloads/Diccionario%20de%20datos%20OCDS.pdf
- https://data.open-contracting.org/en/publication/135 and its /download?name=... links
- https://www.datosabiertos.gob.pe/dataset/proveedores-sancionados-2025 and group page for OECE
- https://www.rnp.gob.pe/consultasenlinea/inhabilitados/busqueda_vnv.asp
- https://bi.seace.gob.pe/pentaho/api/repos/:public:portal:datosabiertos.html/content?userid=public&password=key
- https://cdn.www.gob.pe/uploads/document/file/7594705/6444155-ley-general-de-contrataciones-publicas-con-modificaciones-posteriores-hasta-el-11-12-2024.pdf (Ley 32069)
- https://cdn.www.gob.pe/uploads/document/file/272360/Ley%20N%C2%BA%2029733.pdf.pdf (Ley 29733)
- Web search only (not fetched): https://lpderecho.pe/reglamento-ley-proteccion-datos-personales-decreto-supremo-016-2024-jus/ ; https://centrovirtual.sunat.gob.pe/tramites/inscribete-ruc ; https://data.open-contracting.org/en/publication/78
