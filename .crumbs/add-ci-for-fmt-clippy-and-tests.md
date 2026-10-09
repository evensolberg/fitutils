---
id: fit-wid
title: Add CI for fmt, clippy and tests
status: open
type: task
priority: 2
tags:
- ci
- testing
created: 2026-10-09
updated: 2026-10-09
phase: ''
---

# Add CI for fmt, clippy and tests

There is no CI for formatting, linting or tests on PRs, so the Rust 1.99 clippy errors fixed in #140 went unnoticed. The only workflow is .github/workflows/code_coverage.yml: tarpaulin on every push, using actions/checkout@v2 and codecov-action@v2, which are outdated. Add a workflow for PRs and pushes to master that runs cargo fmt --all --check, cargo clippy --workspace --all-targets -- -D warnings, and cargo nextest run --workspace. Pin the toolchain if you want stable results (rust-toolchain.toml), and decide whether to keep or update the coverage job. Dependabot's github-actions updates (added alongside this crumb) will propose bumps for the outdated actions.
