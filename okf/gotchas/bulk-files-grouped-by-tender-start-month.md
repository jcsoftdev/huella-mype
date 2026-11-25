---
type: Gotcha
title: OECE monthly files are grouped by tender start, not by signing date
description: Fetching only the window's months silently drops most contracts signed early in the window.
tags: [ocds, etl, data-quality]
resource: ../docs/research/research-ocds-data.md
status: stable
generated: { by: "agent:claude-opus-5-5", at: 2026-11-25T04:45:00Z }
---

# Symptom
A huella for 2025 built from the 2025-01..2025-12 files has far fewer contracts than the company really signed in 2025.

# Why
Files are partitioned by `dataSegmentationID` = tender start month. The 2024-12 file holds 6,241 contracts, 4,792 of them signed in 2025.

# Fix
Fetch tender months from about six months before the window starts (2023-07 for a 2024-01 window) and filter by `contracts[].dateSigned`.

# How it was found
The plan audit (2026-11-25) downloaded the 2024-12 and 2025-06 files and counted signing years.
