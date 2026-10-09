# fitutils

Rust workspace with CLI tools for activity files from fitness devices: FIT (Flexible and Interoperable Data Transfer), plus GPX and TCX for most tools. Converts, exports, renames, and views them.

## Workspace Members

- `fit2json/` — converts FIT files to JSON
- `fitexport/` — exports FIT, GPX and TCX files to JSON and CSV
- `fitrename/` — renames FIT, GPX and TCX files based on metadata
- `fitview/` — shows metadata from FIT, GPX and TCX files
- `utilities/` — shared library crate

Version is set per-crate in each member's `Cargo.toml` (no workspace-level version).

## Build & Test

| Task | Command |
|------|---------|
| Check for errors | `cargo lcheck --color always` or `just check` |
| Run tests | `cargo nextest run` or `just test` |
| Run tests with output | `cargo nextest run --no-capture` or `just testp` |
| Build debug | `cargo lbuild --color always` or `just build` |
| Build release (local install) | `just release` |
| Install release (Apple ARM64) | `just releasea` |
| Lint | `cargo lclippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used` |
| Format | `cargo fmt -- --emit=files` |
| Update CHANGELOG | `just changelog` (uses `git-cliff`) |

**Prefer `cargo nextest` over `cargo test`. Prefer `cargo lcheck` over building for quick error checks.**

## Release Process

Releases install binaries locally via `cargo install` (no `cargo-dist` / automated GitHub releases).

Release tags (`v{version}`) carry a single repository-level version, separate from the
per-crate versions: bump it by the largest change since the last tag (a feature in any
crate is a minor bump).

1. Bump `version` in each member crate's `Cargo.toml` that changed
2. Run `just changelog` to regenerate `CHANGELOG.md` via `git-cliff`
3. Commit: `git commit -m "chore: release v{version}"`
4. **Check if the tag already exists** before creating: `git tag --list v{version}`
5. Create tag: `git tag v{version}`
6. Push with tags: `git push && git push --tags`
7. Run `just release` to install binaries to `~/.cargo/bin`

## Documents

Documents in `docs/` use the shared frontmatter standard that
[`docs/_Frontmatter.md`](docs/_Frontmatter.md) points to, with revision history in a
`## Revisions` section at the end, not in the frontmatter. New doc IDs are `FIT-NNN`.

## Rust Guidelines

Follow the **Pragmatic Rust Guidelines** at `/Volumes/SSD/Source/Rust/pragmatic rust guidelines.txt`:

- Idiomatic API patterns and strong types (avoid primitive obsession)
- Thorough docs with canonical sections (summary, examples, errors, panics, safety)
- New crates use `anyhow` for application-level error handling; the existing tools return
  `Result<(), Box<dyn Error>>`
- Good test coverage over observable behavior

## Backlog

Work is tracked with crumbs in `.crumbs/` (`index.csv` is gitignored). Start with
`crumbs next` or `crumbs list`. Plans in `docs/plans/` name their crumb in `crumb_id`.

## Commit Style

- Subject line: conventional commits (`feat:`, `fix:`, `chore:`, `docs:`, etc.)
- Refresh author before committing: `git mit es`
- `cargo fmt` runs automatically via `just build`/`just release`
