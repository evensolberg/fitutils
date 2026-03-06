---
id: fit-fef
title: Convert TCX activity start_time to DateTime
status: closed
type: task
priority: 1
tags:
- tcx2csv
- in-progress
created: 2026-03-05
updated: 2026-03-06
closed_reason: TCX start_time converted to DateTime<Local> with trackpoint fallback
dependencies: []
description: 'Read id into DateTime. If conversion fails, read first Trackpoint.time. GH #88'
---

# Convert TCX activity start_time to DateTime

Read id into DateTime. If conversion fails, read first Trackpoint.time. GH #88
