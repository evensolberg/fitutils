---
id: fit-d03
title: 'Track RUSTSEC-2026-0173: proc-macro-error2 unmaintained (transitive via defmt/assay). Monitor whether upstream resolves; not actionable in-repo.'
status: open
type: task
priority: 2
tags:
- security
- dependencies
created: 2026-07-01
updated: 2026-07-01
phase: ''
---

# Track RUSTSEC-2026-0173: proc-macro-error2 unmaintained

`proc-macro-error2` is pulled in transitively via `assay` → `defmt-macros` → `defmt`.
It was flagged unmaintained on 2026-06-07.

**Not actionable in this repo** — we do not depend on it directly.

## What to monitor

- `assay` upstream: https://github.com/de-vri-es/assay
- `defmt` upstream: https://github.com/knurling-rs/defmt
- Advisory: https://rustsec.org/advisories/RUSTSEC-2026-0173.html

When `assay` or `defmt` drops `proc-macro-error2` (or the advisory is withdrawn),
close this crumb and run `cargo update` + `cargo audit` to verify.
