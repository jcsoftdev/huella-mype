---
type: Decision
title: MVP covers RUC 20 suppliers only
description: Including RUC 10 would publish DNIs and personal names; excluding them leaves out sole-proprietor MYPE, which we state openly.
tags: [privacy, scope, ruc]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Why RUC 20 only
A RUC 10 is "10 + DNI + check digit", and OECE publishes the person's full name for those suppliers. Building a commercial profile for lenders from that is a new purpose for personal data under Ley 29733, even if the source is public. RUC 20 (companies) is business data. In the 2025-01 bulk, 652 suppliers are RUC 20 and 207 are RUC 10.

# Rejected alternative: include RUC 10 masked
Masking still needs the RUC as the lookup key and the subject key would be derived from a DNI. It adds a consent flow (the person signing with their own key) we cannot build and test well in ten days. Cost of our choice: many MYPE are sole proprietors and are left out. Roadmap: RUC 10 with explicit consent signed by the holder.
