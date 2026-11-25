---
type: Product
title: Demo script, video plan and README-for-jury checklist
description: The three scored artifacts are designed here before code.
tags: [video, demo, readme]
resource: ../docs/research/research-market.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Demo data
- **Empresa Demo S.A.C.** (synthetic, fake RUC, labeled "datos de demostración"): carries the Rosa story, the claim and the disbursement.
- **Real RUC 20 suppliers** (30–50, picked by B): search and verification of real public data. In the video, a real company is shown only in the verification scene and never as "sanctioned"; the sanctions example uses the demo company or a blurred name.

# Video (≤ 2:55, Spanish, subtitles, 1080p) — script and screens D, edit C
| Time | Scene | Criterion |
|---|---|---|
| 0:00–0:25 | Problem: credit gap figure; "N empresas RUC 20 firmaron contratos con el Estado en 2024–2025" from our data; "caso ilustrativo" on screen | Problema |
| 0:25–0:40 | Quote from a real analyst or MYPE (B9) | Problema |
| 0:40–1:15 | Rosa (Empresa Demo) searches, sees her huella, **requests the claim** with her wallet; operator approves; "cuenta vinculada" | Uso de Stellar, UX |
| 1:15–1:50 | Luis opens the shared link → **VERIFICADA** (one contract call), explorer links; a real company's huella verified too, contract linked to OECE | Innovación |
| 1:50–2:15 | Luis sends a testnet USDC disbursement with the huella as memo → appears under "desembolsos vinculados"; trustline and payment on stellar.expert | Uso de Stellar |
| 2:15–2:35 | Tamper: edit one amount → **ALTERADA**; revoke → **REVOCADA** | Funcionalidad |
| 2:35–2:55 | Architecture slide; "verificable sin pasar por nuestro servidor, reproducible desde OECE"; tests green; repo link | Funcionalidad |
Milestones: script Nov 30 (D), rough cut Dec 2 (C), final Dec 3 (C), everyone reviews against [How we win](../strategy/how-we-win.md).

# README (final) — assembled by D, sections from owners
1. Title, pitch, **links: video · live app · contract on stellar.expert**, CI badge.
2. **Verify in 1 minute** for jurors: open the demo link → VERIFICADA; download JSON, edit an amount, verify → ALTERADA; or run the standalone verifier.
3. **Try it yourself** with Empresa Demo: claim with a testnet wallet (Friendbot link), receive a disbursement.
4. Problem (sourced), users, coverage number.
5. Solution with screenshots.
6. How Stellar is used: contract functions, two-signer claim, root index, events, TTL; trustline and payment.
7. Architecture diagram and repo layout.
8. Run locally and run tests (counts).
9. Testnet deployment: contract ID, admin/attester addresses, example transactions.
10. Data: sources, licences (OECE CC BY 4.0; sanctions ODC-By), metric definitions, limitations, reproduction steps.
11. Team (names, GitHub, lane).
12. **Uso de IA y código previo:** "La investigación, la documentación (okf/), el prototipo del contrato y partes del código fueron asistidos por IA (Claude) entre el 24 de noviembre y el 4 de diciembre de 2026; todo fue revisado por el equipo. No hay código previo al periodo." Plus dependency list.
13. License (MIT).

# Demo Day pitch
Same structure, live, 3 minutes; any member can give it.
