---
id: fit-tw6
title: Create fitdownload to download FIT files from Garmin
status: open
type: feature
priority: 2
tags:
- fitdownload
- garmin
created: 2026-03-05
updated: 2026-10-09
phase: ''
---

# Create fitdownload to download FIT files from Garmin

Need Garmin APIs, reqwest, config file (TOML/JSON/KDL), tokio. See wiki. See also ../ambient_tools.

[2026-10-09] No longer blocked, and renamed from fitget to fitdownload. Plan: docs/plans/2026-07-01-fitdownload.md (PLAN-2026-001, revision 1.1). The official Garmin API still needs developer-programme approval; the plan works around it by authenticating through WebDriver browser automation, with browser-exported cookies as a fallback. Start at Task 1 of the plan.
