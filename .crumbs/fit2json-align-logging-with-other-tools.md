---
id: fit-5c8
title: "fit2json: align logging with other tools"
status: open
tags: [fit2json, tech-debt, logging]
---

# fit2json: align logging with other tools

fit2json was adapted from an external example and differs from the other three
tools in its logging setup:

- Logging is hardcoded to `LevelFilter::Info` via `env_logger::Builder` directly
- No `-v`/`-q` verbose/quiet flags (all other tools support these via `utilities::build_log()`)
- Uses Clap derive API; others use builder API with a separate `cli.rs` module

**Recommended fix (targeted, not a full rewrite):**

1. Replace the hardcoded log init with `utilities::build_log(&cli_args)`
2. Adjust CLI to pass `cli_args` to `build_log` — may require switching from derive
   to builder API, or keeping derive and extracting verbosity flags separately

**Out of scope:** positional `files` vs `--read`/`-r` naming — fit2json's
positional style is fine for a one-shot conversion tool.

Noted during fit-sdx PR review (PR #137).
