---
id: fit-ce6
title: Migrate utilities/ from chrono to jiff once fitparser drops chrono
status: open
type: task
priority: 3
tags:
- deps
- maintenance
created: 2026-07-04
updated: 2026-07-04
phase: ''
---

# Migrate utilities/ from chrono to jiff once fitparser drops chrono

fitparser 0.11.0 still depends on chrono (verified 2026-07-04 via Cargo.lock). Migrating our own utilities/ code while fitparser keeps chrono would add a second time library without removing the first -- net increase in complexity with no benefit.

When fitparser cuts a release without chrono in its deps, migrate utilities/ (~10 files, all scoped to utilities/src/) to jiff (not time). Reasons for jiff: BurntSushi authorship, richer IANA timezone model via Zoned, serde support on SignedDuration (may let us remove the Duration(f64) newtype workaround), more active development than time 0.3.

Full API mapping and file-by-file change list in: ~/.claude/plans/please-investigate-with-the-bubbly-mango.md

[2026-07-04] Correction (2026-07-04): rustyfit 0.5.0 does NOT depend on chrono. If it uses time or jiff natively, switching parsers would remove chrono from the tree entirely -- a stronger case than a utilities-only migration. Still deferring: rustyfit API paradigm is streaming (vs fitparsers eager Vec), adoption is ~13x lower, and write support is not needed. Revisit if fitparser stalls or rustyfit matures significantly.

[2026-07-04] Further correction (2026-07-04): rustyfit Cargo.lock confirms it has ZERO time library dependencies -- timestamps are returned as raw numeric values for the consumer to convert. Switching to rustyfit + jiff would cleanly remove chrono from the tree with no imposed time library. Still deferring due to streaming API paradigm shift and maturity, but this is a meaningful long-term option.
