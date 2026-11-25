---
okf_version: "0.2"
---

# HuellaMYPE — knowledge bundle

Everything the team needs to build and win Stellar Codember Perú 2026, in [Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf) v0.2. Sourced research lives in `../docs/research/`.

## Read in this order (everyone, ~30 min)
1. [How we win](strategy/how-we-win.md) — the jury scores only repo, testnet and video.
2. [Hackathon rules and dates](references/hackathon-rules.md)
3. [Product definition](product/product-definition.md) and [demo, video and README plan](product/demo-and-video.md)
4. [Team lanes and workflow](plans/team-and-workflow.md), [backlog](plans/backlog.md), [timeline and gates](plans/timeline.md)

## Then your lane
| Lane | Read |
|---|---|
| A — Stellar | Registry contract, architecture, claim and payments decisions, TTL and on-chain data constraints |
| B — Data, payments, evidence | Data pipeline, huella v1, payments decision, all gotchas, research index |
| C — Core, integration, video edit | Huella v1, API and web, architecture, hashing decisions and constraints |
| D — Web and story | API and web, demo and video, product, demo-company decision |

## Strategy and product
- [How we win — the jury scores only repo, testnet and video](strategy/how-we-win.md)
- [Demo script, video plan and README-for-jury checklist](product/demo-and-video.md)
- [HuellaMYPE — product definition](product/product-definition.md)

## Plans
- [Backlog — every task with owner, dependency, criterion and acceptance](plans/backlog.md)
- [Team lanes, ownership and workflow](plans/team-and-workflow.md)
- [Timeline, gates and risk register (Nov 25 – Dec 8)](plans/timeline.md)

## Specs (seams — frozen at kickoff)
- [API routes and web panel — pages, states, copy rules](specs/api-and-web.md)
- [Architecture, repo layout, stack, tests and CI](specs/architecture.md)
- [Data pipeline — from OECE bulk files to published huellas](specs/data-pipeline.md)
- [Huella document v1 — schema, metrics, hashing, verdicts](specs/huella-document-v1.md)
- [Registry contract — interface, storage, events, claim](specs/registry-contract.md)

## References
- [Stellar Codember Peru 2026 — rules, dates, deliverables](references/hackathon-rules.md)
- [Research index and key numbers](references/research-index.md)

## Decisions
- [Report "señales de cumplimiento", never a completion rate](decisions/compliance-signals-not-completion-rate.md)
- [The interactive demo uses a synthetic company, clearly labeled](decisions/demo-company-is-synthetic.md)
- [Flat JCS hash for the MVP; Merkle root only as stretch](decisions/flat-hash-before-merkle.md)
- [All huellas are published as a GitHub release asset read at build time](decisions/huellas-published-as-release-asset.md)
- [Two-step, operator-approved claim is in the MVP](decisions/mype-claim-in-mvp.md)
- [Build huellas offline from monthly bulk files, not live API calls](decisions/offline-monthly-bulk-etl.md)
- [Linked USDC disbursement is in the MVP](decisions/payments-and-assets-in-mvp.md)
- [MVP covers RUC 20 suppliers only](decisions/scope-ruc-20-only.md)
- [One attester key in the demo, but an allow-list in the contract](decisions/single-attester-with-allow-list.md)
- [Subject key = sha256("huella-mype:subject:v1:" + RUC)](decisions/subject-key-is-domain-separated-ruc-hash.md)
- [One TypeScript package computes metrics and hashes everywhere](decisions/typescript-single-hashing-implementation.md)
- [Target soroban-sdk 30 and stellar-cli 30, not the guide's v26](decisions/use-soroban-sdk-30.md)
- [Wallets only for actions that move state; verifying never needs one](decisions/wallet-only-for-the-claim.md)

## Constraints
- [Every huella pins the sha256 of the OECE files it came from](constraints/attestation-pins-source-snapshots.md)
- [Extend TTL on every write and run keep-alive before judging](constraints/extend-ttl-on-every-write.md)
- [Money is integer céntimos and rates are integer basis points in the huella](constraints/money-and-rates-are-integers.md)
- [Nothing personal or identifying goes on-chain](constraints/no-personal-data-on-chain.md)

## Gotchas
- [OECE monthly files are grouped by tender start, not by signing date](gotchas/bulk-files-grouped-by-tender-start-month.md)
- [Monthly file sizes vary a lot; that is seasonality, not missing data](gotchas/bulk-month-counts-look-incomplete.md)
- [PE-RUC-16xxxxx supplier ids are consortia, not RUCs](gotchas/consortium-ids-are-not-rucs.md)
- [implementation.endDate and finalValue do not mean "completed](gotchas/implementation-enddate-is-not-completion.md)
- [OECE API returns 403 to default HTTP clients](gotchas/oece-api-403-without-browser-user-agent.md)
- [OECE records API silently ignores a RUC filter](gotchas/oece-api-ignores-ruc-filter.md)
- [The organizer guide mixes outdated Stellar versions and paths](gotchas/organizer-guide-has-stale-stellar-versions.md)

## Concept types
| Type | Answers | Folder |
|---|---|---|
| Strategy | how we score | `strategy/` |
| Product | what we build and show | `product/` |
| Plan | who, what, when | `plans/` |
| Spec | interfaces lanes build against | `specs/` |
| Reference | external facts we depend on | `references/` |
| Decision | why this way, and the rejected alternative | `decisions/` |
| Constraint | what must hold, and what breaks if not | `constraints/` |
| Gotcha | a trap and how it was found | `gotchas/` |

## Keeping it honest
`project-brain okf validate` (conformance) · `project-brain okf audit` (anchors and staleness). Every PR that reveals a non-obvious why adds or updates a concept here.
