# Research: problem, market and prior art - "Verifiable track record for MYPE that sell to the State"

Date: 2026-11-24. Method: sources fetched with curl and grepped for the quoted figures. Anything not fetched or only seen in a search snippet is marked UNVERIFIED. Figures from third-party sites are labelled as such.

## 0. Findings that change the product (read first)

1. **The OECE OCDS data cannot give true "fulfillment".** The OCP registry entry for the OECE dataset shows 0 transactions, 0 milestones and 0 amendments, and lists "some contracts do not have a status" and "award information is not linked to contract information, and vice versa" as quality issues (last reviewed Dec 2023). Source: https://data.open-contracting.org/en/publication/135 . So "cumplimiento" must be defined with proxies the data really holds (e.g. contract signed, contract status when present, completion/final amount when present via the OECE extension, absence of sanctions in RNP), and labelled as proxies. See section 5.
2. **Contracts below 3/8/9 UIT are not in the dataset** (same OCP page: "The data does not include contracts without a selection procedure, such as contracts below 3UIT, 8UIT, or 9UIT"). Yet OECE says 24.2% of 2024 procurement (S/ 19,536 M of S/ 80,475 M) was contracts of 8 UIT or less (https://www.businessempresarial.com.pe/oece-implementa-herramienta-digital-para-gestionar-contratos-menores/ , seen in search snippet only: UNVERIFIED by fetch). Many micro suppliers live in that gap, so the history is a floor, not the full record.
3. **Licence is confirmed:** publisher's licence CC BY 4.0, downloads JSON/Excel/CSV, daily update, 2003-Oct 2026, per-year files (2024: 126 MB, 2025: 149 MB, 2026: 155 MB). Attribution to OECE is required. https://data.open-contracting.org/en/publication/135 and https://contratacionesabiertas.oece.gob.pe/descargas
4. **Direct competitors/adjacent projects already exist** (differentiate): Cautela (https://cautela.pe/ , fetched) and ContrataMYPE (https://github.com/tejedasebastian/contratamype , fetched). Details in section 3.
5. **Lender demand is the unproven link.** I found no evidence (Peru or LatAm) of a lender using award history as a formal credit signal (section 3). Treat as a hypothesis to validate with 2-3 cajas/analysts.

## 1. Peru MYPE market statistics

| Fact | Value | Source |
|---|---|---|
| Formal Mipyme in 2024 | 2,331,173 (99.3% of formal firms), +1.6% vs 2023; sales S/ 389,819 M | PRODUCE note https://www.gob.pe/institucion/produce/noticias/1299194-numero-de-mipyme-en-peru-ascendio-a-2-33-millones-al-cierre-del-2024-alcanzando-un-crecimiento-de-1-6 (fetched) |
| Same, report | "La Mipyme en Cifras 2024" (PRODUCE, Nov 2025) | https://www.producempresarial.pe/wp-content/uploads/2025/11/La-Mipyme-en-Cifras-2024_.pdf (fetched) |
| Mipyme with formal credit, Dec 2024 | 27.4% (646,043 of 2,354,940 in the table); credit balance S/ 72,503 M | same PDF, ch. 4 (fetched) |
| Informality of mypes in 2024 | 86.8% (2023: 86.3%); "informal" = no RUC | ComexPerú via https://diarioep.pe/informalidad-empresarial-en-mypes-fue-de-86-8-en-2024/ (fetched; secondary, ComexPerú computes from INEI, not fetched at INEI) |
| Mype owners with at least one formal financial product, 2024 | 54.1%; with a credit product 20.6% (ENAHO, ComexPerú estimate) | https://www.comexperu.org.pe/articulo/inclusion-financiera-de-las-mypes-avances-y-retos (fetched) |
| MSME financing gap | US$ 8.1 billion; MSMEs are 99% of private firms, 42% of GDP, 89% of private employment | IFC release 2025 https://www.ifc.org/en/pressroom/2025/financial-inclusion-at-the-core-of-ifc-and-caja-cusco-s-partnership (fetched) |

Notes: "27.4%" is the share of formal Mipyme with credit; do not combine with the 86.8% informality figure (different denominators). OECD Economic Survey Peru 2025 chapter on SME finance exists (https://www.oecd.org/en/publications/2025/09/oecd-economic-surveys-peru-2025_d9bfa904/full-report/supporting-smes-access-to-finance-to-drive-investment-and-growth_fbcc4a4a.html) but returned HTTP 403 to curl: content UNVERIFIED. No SBS/BCRP primary page was fetched: UNVERIFIED.

Implication: the product only serves the formal segment (has RUC and sells to the State). It is a niche with a real RUC and real contracts, not the 87% informal mass.

## 2. MYPE in public procurement, Ley 32069, factoring, guarantees

**Participation.** Figures conflict; report ranges and cite each.
- ComexPeru citing OECE: in 2025 mypes held 47.8% of the amount awarded in the General Regime; share falls above S/ 10 M. https://www.comexperu.org.pe/articulo/nuevas-medidas-para-las-mypes-que-retos-quedan-pendientes (fetched).
- Infobae (Feb 2026): "en 2024 solo captó el 4,1 % de las adjudicaciones estatales" for MYPE manufacturing. Ambiguous scope; do not quote as total MYPE share. https://www.infobae.com/peru/2026/02/26/contrata-con-el-estado-gobierno-alista-nuevo-registro-para-que-mypes-accedan-al-40-de-compras-publicas/ (fetched). Same article: Ley 31578 requires entities to buy at least 40% of goods budget from national manufacturing MYPE.
- OECE has an official MYPE-participation query service (since 2013): https://www.gob.pe/14268-consultar-la-participacion-de-las-mypes-en-las-contrataciones-del-estado (page fetched, content not extracted: UNVERIFIED for numbers).
- Old baseline (2012-2014) of 90% of providers / 41.5% of amount: only from search snippet: UNVERIFIED and outdated.
- No fetched source gives the number of MYPE suppliers per year; compute it ourselves from OCDS (and note RUC size class is not in OCDS; MYPE status needs a SUNAT/REMYPE lookup or an amount proxy. ContrataMYPE uses a S/ 200,000 amount threshold proxy: https://github.com/tejedasebastian/contratamype/blob/main/SPECS.md ).

**Ley 32069 (Ley General de Contrataciones Publicas).** Official consolidated text: https://leyes.congreso.gob.pe/Documentos/2021_2026/ADLP/Texto_Consolidado/32069-TXM.pdf (fetched, read). Verified provisions:
- Art. 67.7: "Las micro y pequeñas empresas tienen la facultad de emitir facturas negociables por plazos diferidos no mayores de ciento ochenta días al contratar con el Estado. Las entidades contratantes tienen la obligación de reconocer al legítimo tenedor de dichas facturas."
- Guarantee of fiel cumplimiento: MYPE may give "la retención" of payment as guarantee (retention of the total, effected in the first half of payments); the law also lists fideicomiso, carta fianza, seguro and retención de pago as instruments.
- Art. 89.3: for MYPE the penalty (multa) cannot exceed 8% (context not fully read: UNVERIFIED which amount base).
- Art. 67.4/67.5: unjustified payment delay is a serious fault; legal interest is owed on late payment.
- Entry into force 22 Apr 2025 with Reglamento D.S. 009-2025-EF: from third-party guide only (https://licitacionesperu.pe/guia/ley-contrataciones-32069/), UNVERIFIED.

**Registro Nacional de MYPE Acreedoras del Estado (RNMA).** Ley 31677 creates a registry of State debts to MYPE, run by MEF, free, monthly since Jan 2024, regulation D.S. 217-2023-EF: from search snippets only (lpderecho page returned 403): UNVERIFIED. Cautela already cross-references it (https://cautela.pe/ , fetched, "1,354 entidades").

**Factoring.**
- Jan-May 2024: 686,318 invoices, S/ 16,476 M, 16,926 firms, 80.1% mype. https://forbes.pe/actualidad/2024-07-02/operaciones-de-factoring-alcanzaron-los-s-16-476-millones-al-cierre-de-mayo-del-2024 (fetched).
- Aug 2026, MEF director (Oscar Orcon): factoring with State entities "no llega ni a 4%"; 2026 YTD about S/ 27,000 M; MEF working with COFIDE on a new guarantee scheme similar to D.U. 040 (about S/ 1,000 M guaranteed, very low default). https://gestion.pe/economia/factoring-participacion-del-estado-no-llega-ni-al-4-pero-se-alistan-mecanismos-para-ampliar-esto-dice-el-mef-noticia/ (fetched). This is the strongest "why now": a verified supplier history is exactly what a factor/lender needs to price State receivables, and MEF is actively trying to grow that segment.
- FAE-MYPE / Reactiva / Fondo Crecer: COFIDE statements at 30 Jun 2025 say it had no loans under FAE-MYPE at that date (snippet of https://documentos.cofide.com.pe/wp-content/uploads/2026/04/EEFF-Junio-2025.pdf ; not read in full: UNVERIFIED). Reactiva (S/ 60,000 M) was a COVID programme, not active. Treat these as history, not live channels.

## 3. How lenders assess thin-file firms; procurement as a credit signal

- Peru has one public register (SBS Central de Riesgos) and private bureaus: Equifax (formerly Infocorp), Sentinel and DataCredito (both Experian Peru), Xchange. Private bureaus need no licence and are not supervised by SBS; defaults are kept 5 years after payment in private bureaus, no limit in SBS. https://decisimo.com/articles/credit-bureaus-peru (fetched, dated 2026-10-27; commercial blog, cites SBS Res. 6941-2008).
- Bureaus only see debt behaviour; a firm that has never borrowed has no file. Public sales history is therefore a new, non-debt signal. Alternative data (utilities, social, e-commerce) is discussed with 10-20% prediction improvement claims and coverage limits: https://www.begini.co/7-fuentes-de-datos-modernos-para-el-analisis-del-riesgo-de-credito-en-microempresas/ (fetched; vendor blog, cites Birch et al. 2018 / Equifax 2019, not verified).
- SBS (Aug 2026) reform lets new lenders temporarily access the anonymised Reporte Crediticio Consolidado during licensing: not directly relevant to procurement data. https://www.infobae.com/peru/2026/08/27/sbs-permitira-que-nuevas-financieras-y-aseguradoras-accedan-a-historial-crediticio-de-los-clientes-antes-de-abrir/ (fetched).

**Procurement-based tools found (all are lookups/scores, none are lender-integrated attestations):**
| Tool | What it does | Source |
|---|---|---|
| Cautela (Peru) | Free "expedientes" per RUC: 97,172 RUCs, 350,706 adjudications, INFObras, SUNAT condition, sanctions, payer behaviour of 1,354 entities (RNMA); paid factoring manual S/ 149 | https://cautela.pe/ (fetched) |
| ContrataMYPE (Peru, MIT, v1.1.1) | OCDS opportunity finder and market dashboard for MYPE; "bundle verificable"; notes status not informed in 2025 bulk | https://github.com/tejedasebastian/contratamype (fetched) |
| SECOPSCORE / LICITEX (Colombia) | Score per entity (0-100) from SECOP II; 111,947 suppliers, 4,070 entities, 503,478 contracts; not a credit score | https://www.siniapsis.com/secopscore/ (fetched) |
| SECOPCheck (Colombia) | Free contract lookup by NIT from datos.gov.co; "una busqueda es un primer filtro, no una certificacion" | https://secopcheck.co/ (fetched) |
| Xepelin (Chile/Mexico) | SME factoring/working capital; Peru presence UNVERIFIED; old Medium profile (403, not fetched) claimed scoring from government invoice data | https://xepelin.com/blog/noticias/que-es-xepelin (fetched, no Peru content) |

Gaps: no evidence found for Kapital, Tribal, or Mexico Compranet-based credit scoring (search found none): UNVERIFIED/not found. Do not claim lenders already use procurement data; claim the opposite as the opportunity.

Differentiation vs Cautela/ContrataMYPE: they present data to a human; we produce a portable, tamper-evident, signed digest the lender can verify independently of our servers.

## 4. Attestations, verifiable credentials, why hash-only is defensible

- W3C Verifiable Credentials Data Model 2.0 became a W3C Recommendation on 15 May 2025 (press release fetched): https://www.w3.org/press-releases/2025/verifiable-credentials-2-0/ . It allows several securing mechanisms (Data Integrity, JOSE/COSE, SD-JWT) per search summary; spec at https://www.w3.org/TR/vc-data-model-2.0/ (fetched, not read in full). Recommendation: issue the history as a VC-shaped JSON (issuer, subject = RUC, claims, canonical hash) and anchor only the hash on Soroban. Cheap and standards-aligned.
- Ethereum Attestation Service is the reference "attestation" pattern (https://docs.attest.org/docs/welcome , fetched, landing page only). Stellar docs for Soroban: https://developers.stellar.org/docs/build/smart-contracts/overview (fetched). I found no official Stellar attestation standard: UNVERIFIED that none exists.
- Stellar/Soroban prior art are community repos, mostly early-stage or template forks: e.g. stellar-did-credit ("credential attestations ... prove creditworthiness to any lender", https://github.com/Devadakene/stellar-did-credit , fetched, status "in active development") and Soroban-Identity (https://github.com/theladyanina/Soroban-Identity , listed in search, not fetched). They are on-chain scoring/identity; ours differs by anchoring an off-chain, publicly reproducible dataset.
- Privacy design. EDPB Guidelines 02/2025 on blockchain (https://www.edpb.europa.eu/system/files/2025-04/edpb_guidelines_202502_blockchain_en.pdf , fetched): recommends storing personal data off-chain and putting only a cryptographic commitment on-chain, which proves integrity while hiding the content. The same guidelines caution that hashes of personal data may still be personal data and that unsalted/unkeyed hashes give weak protection. Consequences for design: (a) use a salted or keyed commitment (HMAC or salt published to the MYPE and lender off-chain), never hash a bare RUC; (b) on-chain store only attestation id, hash, schema version, timestamp, issuer; (c) no names or amounts. This is EU guidance, not Peruvian law; Peru's Ley 29733 text found (https://www.smv.gob.pe/Uploads/Ley_29733_vigente_2025.pdf) was not read: UNVERIFIED how it treats RUC of personas naturales. Note: awards in OCDS are already public (CC BY), so the privacy risk is linkage/permanence, not secrecy.

## 5. Problem statement, users, stories, risks

**Problem statement.** A Peruvian MYPE that has delivered to the State has a public track record, but it is scattered across SEACE/OCDS files that no credit analyst can read, and the MYPE has no way to prove the summary is authentic. Only 27.4% of formal Mipyme have formal credit, and State factoring is under 4% of the factoring market. HuellaMYPE turns public OCDS records into a per-RUC history, anchors its hash on Stellar, and lets any lender recompute the history and check it matches, without personal data on-chain.

**Users.** (1) MYPE owner (formal, has RUC, sells to the State). (2) Credit analyst at a caja municipal, cooperativa, or fintech/factor. (3) Secondary: cooperativa credit committee, MYPE association/advisor.

**User stories**
- MYPE owner: 1) As an owner I enter my RUC and see my contracts, amounts and status so I know what a lender will see. 2) I generate a signed attestation and a shareable link/QR so I can send it with a loan application. 3) I see which records are incomplete (e.g. contract without status) so I can fix or explain them.
- Credit analyst: 1) I paste a RUC and a hash, and the panel says VERIFIED or NOT MATCHING against the chain. 2) I see counts, total and median amount, years active, repeat buyers, and data-quality flags in one screen. 3) I open the source OCDS records (ocid links) behind every number.
- Cooperativa/caja: 1) I check a batch of RUCs before a committee. 2) I see when an attestation is superseded by a newer one. 3) I can export the verification result for the loan file.

**Key risks and mitigations**
| Risk | Mitigation |
|---|---|
| Data quality (no payments/milestones, missing statuses, unlinked awards, <8 UIT missing) | Show a data-completeness score; call the metric "evidencia de contratos" and fulfilment proxies, never "scoring"; publish counts excluded; use recent years (2023-2026) |
| Does history prove the requester is the supplier? Public data proves only that a RUC won contracts, not that the person asking owns that RUC | The MYPE proves control: attestation requested by signing with a Stellar key bound to the RUC via SUNAT clave SOL or a manual verification step (UNVERIFIED feasibility, no API verified); for the hackathon, state the limitation explicitly and let the lender confirm RUC by SUNAT |
| Oracle trust: who is the attester? | Single issuer key for the demo; publish the pipeline, input file hash (OECE file + date) and code so anyone recomputes the hash; roadmap: multiple attesters / OECE or a gremio as issuer, key rotation and revocation in the contract |
| Hash correctness and reproducibility | Canonical JSON, versioned schema, pinned dataset snapshot id; tests that recompute the hash and fail on a tampered record |
| Privacy of personas naturales (RUC 10) | Salted commitment, nothing personal on-chain, consent step by the MYPE before attesting |
| Lender doesn't care (hypothesis) | Interview 2-3 lenders; position as complement to bureaus, not replacement |
| Licence/attribution | Credit OECE, CC BY 4.0, in the panel footer and each attestation |

## 6. Name ideas (folder: huella-mype)
1. **HuellaMYPE** - keeps the working name, "huella" = footprint and hash. Strongest.
2. **ConstanciaPe** - echoes "constancia" (signed statement), formal tone for lenders.
3. **RastroCierto** - "verified trail".
4. **SelloMYPE** - seal/stamp of authenticity.
5. **Historial Abierto** - emphasises open data and credit history.
(Trademark/domain availability not checked: UNVERIFIED.)

## 7. Sources not retrieved
Hackathon pages (https://stellar.mintedinpe.com/codember/bases, /guia, /ide) were out of scope of this task; /bases returned only a title in my fetch (JS-rendered). Deadline of Dec 4 2026 comes from the task brief, not verified here.
