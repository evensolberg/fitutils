---
id: fit-642
title: Better way to serialize Lap and Record structs
status: closed
type: task
priority: 1
tags:
- fit2csv
- backlog
created: 2026-03-05
updated: 2026-03-06
closed_reason: Custom serde for FITLap/FITRecord UOM types; records CSV uses auto headers
dependencies: []
description: 'Call serde::serialize on Vec<Lap/Record> directly instead of writing header then iterating. GH #103'
---

# Better way to serialize Lap and Record structs

Call serde::serialize on Vec<Lap/Record> directly instead of writing header then iterating. GH #103
