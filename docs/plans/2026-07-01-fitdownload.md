---
tags:
  - fitutils
  - garmin
  - download
  - planning
aliases:
  - fitdownload
  - garmin-download
doc_id: PLAN-2026-001
doc_name: fitdownload-plan
crumb_id:
document_title: "fitdownload — Garmin Connect FIT File Downloader"
created_date: 2026-07-01
synopsis: >
  Implementation plan for a new fitutils workspace member that authenticates
  with Garmin Connect using browser-exported session cookies and downloads FIT
  files for activities. Direct OAuth login is blocked by Cloudflare TLS
  fingerprinting since March 2026; the cookie-import approach is the current
  reliable method. The official Garmin Activity API would be preferable but
  requires developer programme approval, which is not currently being granted.
status: draft
type: plan
revision: 1
review_date:
reviewed_by: []
completed_date:
comments:
revision_history:
  - date: 2026-07-01
    author: even.solberg@gmail.com
    change: Initial creation
---

# fitdownload — Garmin Connect FIT Downloader Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `fitdownload` CLI crate to the fitutils workspace that authenticates with Garmin Connect using browser-exported session cookies and downloads FIT activity files.

**Architecture:** Two-subcommand CLI: `auth` imports a Netscape-format cookie file (exported from a real browser via the "Get cookies.txt" extension) into a persisted session at `~/.config/fitutils/garmin-session.json`; `download` uses that session to list activities via the unofficial Garmin Connect JSON API and stream FIT files to a local directory, with pagination, date/type filtering, exponential-backoff retry on 429s, and progress bars.

**Why cookie-based auth:** As of March 2026 Garmin's SSO endpoints are protected by Cloudflare TLS fingerprinting. Direct `reqwest` / `curl` login is blocked with 403. The only reliable headless approach is to use cookies captured from a real browser session. This is a deliberate architectural choice, not a gap. The `garmin_client` Rust crate uses the old (now-blocked) OAuth SSO flow and should not be used.

**Tech Stack:** Rust 2024, `tokio` async runtime, `reqwest 0.12` (rustls-tls, cookies, stream), `reqwest_cookie_store`, `cookie_store`, `scraper` (CSRF HTML fallback), `serde`/`serde_json`, `clap 4`, `indicatif`, `anyhow`, `chrono`, `log`/`env_logger`, `zip` (FIT-in-zip handling), `mockito` (tests).

## Global Constraints

- Rust edition 2024; `cargo lclippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used` must be clean.
- Use `anyhow::Result` throughout (per CLAUDE.md); `main()` calls `std::process::exit(1)` on error.
- No hardcoded credentials or hostnames as literals — use named `const` in `client.rs`.
- All network tests use a `mockito` mock server; never hit real Garmin servers.
- Tests run via `cargo nextest run`; unit tests live in `#[cfg(test)]` modules in the same file.
- Commit convention: `feat:`, `fix:`, `chore:`, etc. with `Co-Authored-By: Claude <noreply@anthropic.com>`.
- Run `git mit es` before every commit (skip silently if not installed).

---

## File Map

**New files (create):**

- `fitdownload/Cargo.toml` — crate manifest
- `fitdownload/src/main.rs` — async entry point, dispatch
- `fitdownload/src/cli.rs` — clap subcommand definitions
- `fitdownload/src/session.rs` — `Session` struct, Netscape cookie parsing, persistence
- `fitdownload/src/client.rs` — `GarminClient`, HTTP with cookie store and CSRF header
- `fitdownload/src/activities.rs` — `ActivitySummary`, `list_activities()`, pagination
- `fitdownload/src/download.rs` — `download_fit()`, filename generation, retry, zip handling

**Modified:**

- `Cargo.toml` (workspace root) — add `fitdownload` to `members`; add `anyhow`, `reqwest`, `tokio`, `scraper`, `indicatif`, `zip`, `reqwest_cookie_store`, `cookie_store` to `[workspace.dependencies]`
- `justfile` — add `fitdownload` to release install targets

---

## Task 0: Stage plan and frontmatter

- [ ] **Step 0.1: Copy this plan to the project**

```bash
mkdir -p /Volumes/SSD/Source/Rust/fitutils/docs/plans
cp /Users/evensolberg/.claude/plans/i-would-like-to-imperative-stream.md \
   /Volumes/SSD/Source/Rust/fitutils/docs/plans/2026-07-01-fitdownload.md
```

- [ ] **Step 0.2: Populate docs/_Frontmatter.md**

Replace the contents of `/Volumes/SSD/Source/Rust/fitutils/docs/_Frontmatter.md` with:

```markdown
---
tags:
  - fitutils
  - garmin
  - download
  - planning
aliases:
  - fitdownload
  - garmin-download
doc_id: PLAN-2026-001
doc_name: fitdownload-plan
crumb_id:
document_title: "fitdownload — Garmin Connect FIT File Downloader"
created_date: 2026-07-01
synopsis: >
  Implementation plan for a new fitutils workspace member that
  authenticates with Garmin Connect using browser-exported session
  cookies and downloads FIT files for activities. Direct OAuth login is
  blocked by Cloudflare TLS fingerprinting (since March 2026); the
  cookie-import approach is the current reliable method.
status: draft
type: plan
revision: 1
review_date:
reviewed_by: []
completed_date:
comments: "See docs/plans/2026-07-01-fitdownload.md for the full implementation plan."
revision_history:
  - date: 2026-07-01
    author: even.solberg@gmail.com
    change: Initial creation
---
```

---

## Task 1: Workspace scaffolding

**Files:**

- Modify: `Cargo.toml` (workspace root)
- Create: `fitdownload/Cargo.toml`
- Create: `fitdownload/src/main.rs`

**Interfaces:**

- Produces: compilable skeleton that passes `cargo lcheck`

- [ ] **Step 1.1: Add new workspace dependencies**

In `/Volumes/SSD/Source/Rust/fitutils/Cargo.toml`, add `fitdownload` to `members` and extend `[workspace.dependencies]`:

```toml
[workspace]
members = [
    "fit2json",
    "fitexport",
    "fitrename",
    "fitview",
    "fitdownload",   # add this line
    "utilities",
]

resolver = "2"

[profile.release]
strip = true

[workspace.dependencies]
# existing (ensure serde includes derive feature)
clap = { version = "4", features = ["cargo", "env"] }
log = "0.4"
env_logger = "0.11"
chrono = "0.4"
csv = "1"
serde = { version = "1", features = ["derive"] }
fitparser = "0.11"
gpx = "0.10"
tcx = "0.9"
assay = "0.1"
convert_case = "0.11"
serde_json = "1"
uom = { version = "0.38", default-features = false }

# new — added for fitdownload
anyhow = "1"
reqwest = { version = "0.12", features = ["cookies", "json", "gzip", "rustls-tls", "stream"] }
reqwest_cookie_store = "0.8"
cookie_store = "0.21"
tokio = { version = "1", features = ["full"] }
scraper = "0.22"
indicatif = "0.17"
zip = "2"
```

> **Note on serde:** The existing entry `serde = "1"` has no `features`. Check each existing crate — if any uses `#[derive(Serialize, Deserialize)]` they must already pull in `derive` somehow. The safest fix is to add `features = ["derive"]` at the workspace level as shown above.

- [ ] **Step 1.2: Create fitdownload/Cargo.toml**

```toml
[package]
name = "fitdownload"
version = "0.1.0"
authors = ["evensolberg <even.solberg@gmail.com>"]
edition = "2024"
license = "Apache-2.0"
description = "Download FIT activity files from Garmin Connect"

[dependencies]
anyhow = { workspace = true }
clap = { workspace = true, features = ["derive"] }
log = { workspace = true }
env_logger = { workspace = true }
chrono = { workspace = true }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
reqwest = { workspace = true }
reqwest_cookie_store = { workspace = true }
cookie_store = { workspace = true }
tokio = { workspace = true }
scraper = { workspace = true }
indicatif = { workspace = true }
zip = { workspace = true }

[dev-dependencies]
mockito = "1"
tempfile = "3"
tokio = { workspace = true, features = ["full"] }
```

- [ ] **Step 1.3: Create fitdownload/src/main.rs skeleton**

```rust
mod activities;
mod cli;
mod client;
mod download;
mod session;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() {
    std::process::exit(match run().await {
        Ok(()) => 0,
        Err(err) => {
            log::error!("{err:#}");
            1
        }
    });
}

async fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::build_log(cli.verbose, cli.quiet);
    match cli.command {
        cli::Command::Auth(args) => cli::cmd_auth(args).await,
        cli::Command::Download(args) => cli::cmd_download(args).await,
    }
}
```

- [ ] **Step 1.4: Create placeholder module files**

Create the following files, each containing just `// placeholder` (will be replaced in later tasks):

- `fitdownload/src/cli.rs`
- `fitdownload/src/session.rs`
- `fitdownload/src/client.rs`
- `fitdownload/src/activities.rs`
- `fitdownload/src/download.rs`

- [ ] **Step 1.5: Verify it checks cleanly**

```bash
cd /Volumes/SSD/Source/Rust/fitutils
cargo lcheck --color always -p fitdownload
```

Expected: `Finished` with 0 errors. Fix any module-not-found or unused-import warnings.

- [ ] **Step 1.6: Commit**

```bash
git checkout -b feat/fitdownload
git add Cargo.toml fitdownload/
git commit -m "$(cat <<'EOF'
chore: scaffold fitdownload workspace member

Add fitdownload crate skeleton and new workspace dependencies (anyhow,
reqwest, tokio, etc.) needed for Garmin Connect HTTP access.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: CLI module

**Files:**

- Create: `fitdownload/src/cli.rs`

**Interfaces:**

- Produces:
  - `cli::Cli` (clap `Parser` struct)
  - `cli::Command::Auth(AuthArgs)` / `cli::Command::Download(DownloadArgs)`
  - `cli::build_log(verbose: u8, quiet: bool)`
  - `cli::cmd_auth(args: AuthArgs) -> Result<()>` (stub, replaced in Task 7)
  - `cli::cmd_download(args: DownloadArgs) -> Result<()>` (stub, replaced in Task 7)

- [ ] **Step 2.1: Write failing tests**

At the bottom of `fitdownload/src/cli.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn auth_long_flags_parse() {
        let cli = Cli::try_parse_from([
            "fitdownload", "auth",
            "--cookies", "/tmp/cookies.txt",
        ])
        .expect("should parse");
        let Command::Auth(args) = cli.command else {
            panic!("expected Auth");
        };
        assert_eq!(args.cookies, Some(std::path::PathBuf::from("/tmp/cookies.txt")));
        assert!(!args.status);
    }

    #[test]
    fn download_long_flags_parse() {
        let cli = Cli::try_parse_from([
            "fitdownload", "download",
            "--output-dir", "/tmp/fit",
            "--limit", "10",
            "--after", "2026-01-01",
        ])
        .expect("should parse");
        let Command::Download(args) = cli.command else {
            panic!("expected Download");
        };
        assert_eq!(args.output_dir, std::path::PathBuf::from("/tmp/fit"));
        assert_eq!(args.limit, Some(10));
        assert_eq!(args.after.as_deref(), Some("2026-01-01"));
    }
}
```

- [ ] **Step 2.2: Run tests to verify they fail**

```bash
cargo nextest run -p fitdownload
```

Expected: compilation error — `Cli`, `Command`, `AuthArgs`, `DownloadArgs` not yet defined.

- [ ] **Step 2.3: Implement cli.rs**

```rust
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Download FIT activity files from Garmin Connect.
#[derive(Debug, Parser)]
#[command(name = "fitdownload", version, author, about)]
pub struct Cli {
    /// Increase log verbosity (-v debug, -vv trace).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Suppress all non-error output.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Import browser cookies and save a Garmin Connect session.
    Auth(AuthArgs),
    /// Download FIT files for recent activities.
    Download(DownloadArgs),
}

#[derive(Debug, clap::Args)]
pub struct AuthArgs {
    /// Path to a Netscape-format cookies.txt file exported from your browser.
    #[arg(short, long, value_name = "FILE")]
    pub cookies: Option<PathBuf>,

    /// Show current session status (validity, expiry).
    #[arg(short, long)]
    pub status: bool,
}

#[derive(Debug, clap::Args)]
pub struct DownloadArgs {
    /// Directory to write downloaded FIT files into.
    #[arg(short, long, default_value = ".")]
    pub output_dir: PathBuf,

    /// Maximum number of activities to download (most recent first).
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// Only download activities on or after this date (YYYY-MM-DD).
    #[arg(long)]
    pub after: Option<String>,

    /// Only download activities on or before this date (YYYY-MM-DD).
    #[arg(long)]
    pub before: Option<String>,

    /// Comma-separated activity types to include, e.g. running,cycling.
    #[arg(long)]
    pub types: Option<String>,

    /// Overwrite files that already exist in output-dir.
    #[arg(long)]
    pub overwrite: bool,
}

/// Configure env_logger from CLI verbosity flags.
pub fn build_log(verbose: u8, quiet: bool) {
    use log::LevelFilter;
    let level = if quiet {
        LevelFilter::Error
    } else {
        match verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    env_logger::Builder::new().filter_level(level).init();
}

/// Stub — replaced in Task 7.
pub async fn cmd_auth(_args: AuthArgs) -> Result<()> {
    anyhow::bail!("not yet implemented")
}

/// Stub — replaced in Task 7.
pub async fn cmd_download(_args: DownloadArgs) -> Result<()> {
    anyhow::bail!("not yet implemented")
}

#[cfg(test)]
mod tests {
    // paste tests from Step 2.1 here
}
```

- [ ] **Step 2.4: Run tests to verify they pass**

```bash
cargo nextest run -p fitdownload
```

Expected: `auth_long_flags_parse` and `download_long_flags_parse` both pass.

- [ ] **Step 2.5: Clippy clean**

```bash
cargo lclippy -p fitdownload -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
```

Fix any warnings before continuing.

- [ ] **Step 2.6: Commit**

```bash
git add fitdownload/src/cli.rs fitdownload/src/main.rs
git commit -m "$(cat <<'EOF'
feat(fitdownload): add CLI subcommand definitions (auth, download)

Defines Cli, AuthArgs, and DownloadArgs with clap derive, and a
build_log() helper mirroring the pattern used in fitexport.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: Session module

**Files:**

- Create: `fitdownload/src/session.rs`

**Interfaces:**

- Produces:
  - `ParsedCookie { domain: String, path: String, secure: bool, expires_secs: u64, name: String, value: String }`
  - `Session { cookies: Vec<ParsedCookie>, csrf_header_value: Option<String>, imported_at: String }`
  - `parse_netscape(data: &str) -> Result<Vec<ParsedCookie>>`
  - `Session::from_cookies_file(path: &Path) -> Result<Session>`
  - `Session::load() -> Result<Session>` — reads `~/.config/fitutils/garmin-session.json`
  - `Session::load_from(path: &Path) -> Result<Session>`
  - `Session::save(&self) -> Result<()>`
  - `Session::save_to(&self, path: &Path) -> Result<()>`
  - `Session::is_valid(&self) -> bool`
  - `Session::csrf_from_cookies(&self) -> Option<String>` — looks for cookie named `connect-csrf-value` (the header value stored in a cookie)
  - `session_path() -> PathBuf`

> **Naming note:** The CSRF cookie Garmin sets is named `connect-csrf-token` but its *value* is what we send as the `connect-csrf-token` request header. The `Session` struct field is named `csrf_header_value` to avoid confusion.

- [ ] **Step 3.1: Write failing tests**

At the bottom of `fitdownload/src/session.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_COOKIES: &str = "# Netscape HTTP Cookie File\n\
        # https://curl.haxx.se/rfc/cookie_spec.html\n\
        .garmin.com\tTRUE\t/\tTRUE\t1893456000\tSESSIONID\tabc123xyz\n\
        connect.garmin.com\tFALSE\t/\tTRUE\t1893456000\tconnect-csrf-token\tcsrf-abc\n\
        .garmin.com\tTRUE\t/\tTRUE\t1893456000\tGARMIN_GUID\tguidvalue\n";

    #[test]
    fn parse_netscape_cookies() {
        let cookies = parse_netscape(SAMPLE_COOKIES).expect("parse ok");
        assert_eq!(cookies.len(), 3);
        let session_cookie = cookies.iter().find(|c| c.name == "SESSIONID").expect("found");
        assert_eq!(session_cookie.domain, ".garmin.com");
        assert_eq!(session_cookie.value, "abc123xyz");
        assert!(session_cookie.secure);
        assert_eq!(session_cookie.expires_secs, 1_893_456_000);
    }

    #[test]
    fn csrf_value_extracted_from_cookies() {
        let cookies = parse_netscape(SAMPLE_COOKIES).expect("parse ok");
        let session = Session {
            cookies,
            csrf_header_value: None,
            imported_at: String::new(),
        };
        assert_eq!(session.csrf_from_cookies().as_deref(), Some("csrf-abc"));
    }

    #[test]
    fn roundtrip_save_load() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("session.json");
        let session = Session {
            cookies: vec![ParsedCookie {
                domain: ".garmin.com".into(),
                path: "/".into(),
                secure: true,
                expires_secs: 1_893_456_000,
                name: "SESSIONID".into(),
                value: "abc123".into(),
            }],
            csrf_header_value: Some("csrf-xyz".into()),
            imported_at: "2026-07-01T00:00:00Z".into(),
        };
        session.save_to(&path).expect("save");
        let loaded = Session::load_from(&path).expect("load");
        assert_eq!(loaded.cookies[0].name, "SESSIONID");
        assert_eq!(loaded.csrf_header_value.as_deref(), Some("csrf-xyz"));
    }
}
```

- [ ] **Step 3.2: Run to verify tests fail**

```bash
cargo nextest run -p fitdownload
```

Expected: compile errors — `Session`, `ParsedCookie`, `parse_netscape` not yet defined.

- [ ] **Step 3.3: Implement session.rs**

```rust
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// A single cookie parsed from a Netscape-format cookies.txt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCookie {
    pub domain: String,
    pub path: String,
    pub secure: bool,
    /// Unix timestamp; 0 means session cookie (no expiry).
    pub expires_secs: u64,
    pub name: String,
    pub value: String,
}

/// Persisted Garmin Connect session (cookies + optional CSRF header value).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub cookies: Vec<ParsedCookie>,
    /// Value to send as the `connect-csrf-token` request header.
    /// May be extracted from the `connect-csrf-token` cookie or fetched
    /// from the Garmin Connect HTML page.
    pub csrf_header_value: Option<String>,
    /// ISO-8601 timestamp of when cookies were imported.
    pub imported_at: String,
}

/// Default path for the persisted session file.
pub fn session_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".config")
        .join("fitutils")
        .join("garmin-session.json")
}

impl Session {
    /// Load session from the default path (`~/.config/fitutils/garmin-session.json`).
    pub fn load() -> Result<Self> {
        Self::load_from(&session_path())
    }

    /// Load session from an explicit path.
    pub fn load_from(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)
            .with_context(|| format!("reading session file {}", path.display()))?;
        serde_json::from_str(&data)
            .with_context(|| format!("parsing session file {}", path.display()))
    }

    /// Save session to the default path.
    pub fn save(&self) -> Result<()> {
        self.save_to(&session_path())
    }

    /// Save session to an explicit path, creating parent directories as needed.
    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {}", parent.display()))?;
        }
        let json = serde_json::to_string_pretty(self).context("serialising session")?;
        let mut f = std::fs::File::create(path)
            .with_context(|| format!("creating session file {}", path.display()))?;
        f.write_all(json.as_bytes()).context("writing session file")?;
        Ok(())
    }

    /// Build a Session from a Netscape-format cookies.txt file.
    pub fn from_cookies_file(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)
            .with_context(|| format!("reading cookies file {}", path.display()))?;
        let cookies = parse_netscape(&data)
            .with_context(|| format!("parsing Netscape cookies from {}", path.display()))?;
        let csrf = cookies
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case("connect-csrf-token"))
            .map(|c| c.value.clone());
        Ok(Self {
            csrf_header_value: csrf,
            cookies,
            imported_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Returns `true` if any non-session cookie has not yet expired.
    pub fn is_valid(&self) -> bool {
        if self.cookies.is_empty() {
            return false;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.cookies
            .iter()
            .any(|c| c.expires_secs == 0 || c.expires_secs > now)
    }

    /// Extract the CSRF header value from the cookie named `connect-csrf-token`.
    pub fn csrf_from_cookies(&self) -> Option<String> {
        self.cookies
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case("connect-csrf-token"))
            .map(|c| c.value.clone())
    }
}

/// Parse a Netscape-format cookies.txt string into a list of `ParsedCookie`.
///
/// Each non-comment, non-empty line has 7 tab-separated fields:
/// `domain  http_only_flag  path  secure  expires_unix  name  value`
pub fn parse_netscape(data: &str) -> Result<Vec<ParsedCookie>> {
    let mut cookies = Vec::new();
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.splitn(7, '\t').collect();
        if fields.len() != 7 {
            log::debug!("skipping malformed cookie line (expected 7 tab-separated fields): {line}");
            continue;
        }
        let expires_secs = fields[4].parse::<u64>().unwrap_or(0);
        cookies.push(ParsedCookie {
            domain: fields[0].to_owned(),
            path: fields[2].to_owned(),
            secure: fields[3].eq_ignore_ascii_case("true"),
            expires_secs,
            name: fields[5].to_owned(),
            value: fields[6].to_owned(),
        });
    }
    Ok(cookies)
}

#[cfg(test)]
mod tests {
    // paste tests from Step 3.1 here
}
```

- [ ] **Step 3.4: Run tests to verify they pass**

```bash
cargo nextest run -p fitdownload
```

Expected: all 3 session tests pass.

- [ ] **Step 3.5: Commit**

```bash
git add fitdownload/src/session.rs fitdownload/Cargo.toml
git commit -m "$(cat <<'EOF'
feat(fitdownload): add session module for browser cookie import

Parses Netscape-format cookies.txt, extracts CSRF header value from
cookie store, and persists session to ~/.config/fitutils/garmin-session.json.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 4: HTTP client module

**Files:**

- Create: `fitdownload/src/client.rs`

**Interfaces:**

- Consumes: `session::Session`, `session::ParsedCookie`
- Produces:
  - `GarminClient` struct
  - `GarminClient::new(session: &Session) -> Result<GarminClient>`
  - `GarminClient::new_with_base(session: &Session, base_url: &str) -> Result<GarminClient>` (used in tests)
  - `GarminClient::refresh_csrf(&mut self) -> Result<()>` — fetches CSRF from page HTML
  - `GarminClient::get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T>`
  - `GarminClient::get_bytes(&self, path: &str) -> Result<Bytes>`
  - `extract_csrf_from_html(html: &str) -> Option<String>` (pub for testing)

Constants (defined in `client.rs`):

```rust
pub const BASE_URL: &str = "https://connect.garmin.com";
const CSRF_HEADER_NAME: &str = "connect-csrf-token";
const CONNECT_PAGE: &str = "/modern/";
const CSRF_META_SELECTOR: &str = "meta[name='csrf-token']";
```

- [ ] **Step 4.1: Write failing tests**

At the bottom of `fitdownload/src/client.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csrf_from_html_extracts_content_attr() {
        let html = r#"<html><head>
            <meta name="csrf-token" content="extracted-abc-def">
        </head><body></body></html>"#;
        let found = extract_csrf_from_html(html);
        assert_eq!(found.as_deref(), Some("extracted-abc-def"));
    }

    #[test]
    fn csrf_from_html_returns_none_when_absent() {
        let html = "<html><head></head><body></body></html>";
        assert!(extract_csrf_from_html(html).is_none());
    }

    #[tokio::test]
    async fn get_json_sends_csrf_header() {
        let mut server = mockito::Server::new_async().await;
        let expected_csrf = "header-value-xyz";
        let mock = server
            .mock("GET", "/test-endpoint")
            .match_header(CSRF_HEADER_NAME, expected_csrf)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ok": true}"#)
            .create_async()
            .await;

        let session = crate::session::Session {
            cookies: vec![],
            csrf_header_value: Some(expected_csrf.into()),
            imported_at: String::new(),
        };
        let client = GarminClient::new_with_base(&session, &server.url())
            .expect("build client");
        let val: serde_json::Value = client.get_json("/test-endpoint").await.expect("get");
        assert_eq!(val["ok"], true);
        mock.assert_async().await;
    }
}
```

- [ ] **Step 4.2: Run to verify tests fail**

```bash
cargo nextest run -p fitdownload
```

Expected: compile errors.

- [ ] **Step 4.3: Implement client.rs**

```rust
use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::session::Session;

pub const BASE_URL: &str = "https://connect.garmin.com";
const CSRF_HEADER_NAME: &str = "connect-csrf-token";
const CONNECT_PAGE: &str = "/modern/";
const CSRF_META_SELECTOR: &str = "meta[name='csrf-token']";

/// HTTP client pre-loaded with Garmin session cookies and CSRF header value.
pub struct GarminClient {
    inner: reqwest::Client,
    base_url: String,
    /// Value sent as the `connect-csrf-token` header on every request.
    csrf_header_value: String,
}

impl GarminClient {
    /// Build a client using the real Garmin Connect base URL.
    pub fn new(session: &Session) -> Result<Self> {
        Self::new_with_base(session, BASE_URL)
    }

    /// Build a client with a custom base URL (for unit tests with a mock server).
    pub fn new_with_base(session: &Session, base_url: &str) -> Result<Self> {
        let csrf_header_value = session
            .csrf_from_cookies()
            .or_else(|| session.csrf_header_value.clone())
            .unwrap_or_default();

        // Build a cookie store and populate it from the session.
        let store = Arc::new(reqwest_cookie_store::CookieStoreMutex::new(
            reqwest_cookie_store::CookieStore::new(None),
        ));
        {
            let mut jar = store.lock().map_err(|_| anyhow!("cookie store lock poisoned"))?;
            for cookie in &session.cookies {
                let url = build_cookie_url(&cookie.domain)?;
                // Format as a Set-Cookie header value for the store to parse.
                let raw = format!(
                    "{}={}; Domain={}; Path={}{}",
                    cookie.name,
                    cookie.value,
                    cookie.domain,
                    cookie.path,
                    if cookie.secure { "; Secure" } else { "" },
                );
                if let Ok(parsed) =
                    reqwest_cookie_store::RawCookie::parse(raw.as_str().into(), &url)
                {
                    jar.insert_raw(&parsed, &url).ok();
                }
            }
        }

        let client = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&store))
            .user_agent(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
                 AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/125.0.0.0 Safari/537.36",
            )
            .build()
            .context("building reqwest client")?;

        Ok(Self {
            inner: client,
            base_url: base_url.to_owned(),
            csrf_header_value,
        })
    }

    /// Fetch the Garmin Connect page and extract the CSRF header value from
    /// `<meta name="csrf-token" content="...">`. Updates the client in place.
    pub async fn refresh_csrf(&mut self) -> Result<()> {
        let url = format!("{}{}", self.base_url, CONNECT_PAGE);
        let html = self
            .inner
            .get(&url)
            .send()
            .await
            .context("fetching Garmin Connect page")?
            .text()
            .await
            .context("reading Garmin Connect page body")?;
        self.csrf_header_value = extract_csrf_from_html(&html).ok_or_else(|| {
            anyhow!(
                "CSRF value not found on Garmin Connect page; \
                 your session may be expired — re-import cookies"
            )
        })?;
        Ok(())
    }

    /// GET a JSON endpoint and deserialize the response body.
    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .get(&url)
            .header(CSRF_HEADER_NAME, &self.csrf_header_value)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET {url} returned {status}: {body}");
        }
        resp.json::<T>()
            .await
            .with_context(|| format!("parsing JSON from {url}"))
    }

    /// GET a binary endpoint and return the raw bytes.
    pub async fn get_bytes(&self, path: &str) -> Result<Bytes> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .get(&url)
            .header(CSRF_HEADER_NAME, &self.csrf_header_value)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        let status = resp.status();
        if !status.is_success() {
            anyhow::bail!("GET {url} returned {status}");
        }
        resp.bytes()
            .await
            .with_context(|| format!("reading bytes from {url}"))
    }
}

/// Extract CSRF header value from `<meta name="csrf-token" content="...">`.
pub fn extract_csrf_from_html(html: &str) -> Option<String> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse(CSRF_META_SELECTOR).ok()?;
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(ToOwned::to_owned)
}

fn build_cookie_url(domain: &str) -> Result<reqwest::Url> {
    let host = domain.trim_start_matches('.');
    reqwest::Url::parse(&format!("https://{host}/"))
        .with_context(|| format!("building URL for domain {domain}"))
}

#[cfg(test)]
mod tests {
    // paste tests from Step 4.1 here
}
```

> **Note on `reqwest_cookie_store`:** The exact API for `RawCookie::parse` changed between versions. Check the docs for `reqwest_cookie_store 0.8` — you may need `cookie::Cookie::parse` from the `cookie` crate instead. Adjust the import path accordingly.

- [ ] **Step 4.4: Run tests**

```bash
cargo nextest run -p fitdownload
```

Expected: `csrf_from_html_extracts_content_attr`, `csrf_from_html_returns_none_when_absent`, and `get_json_sends_csrf_header` all pass.

- [ ] **Step 4.5: Commit**

```bash
git add fitdownload/src/client.rs
git commit -m "$(cat <<'EOF'
feat(fitdownload): add GarminClient with cookie store and CSRF header

Builds a reqwest client pre-loaded with Netscape cookies; injects the
connect-csrf-token header on every request; falls back to HTML scraping
when the cookie value is absent.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 5: Activity list module

**Files:**

- Create: `fitdownload/src/activities.rs`

**Interfaces:**

- Consumes: `client::GarminClient`
- Produces:
  - `ActivitySummary { id: u64, name: String, date: String, activity_type: String, duration_secs: u64 }`
  - `ListOpts { limit: Option<usize>, after: Option<NaiveDate>, before: Option<NaiveDate>, types: Option<Vec<String>> }`
  - `list_activities(client: &GarminClient, opts: &ListOpts) -> Result<Vec<ActivitySummary>>`
  - `apply_type_filter(activities: Vec<ActivitySummary>, types: &Option<Vec<String>>) -> Vec<ActivitySummary>` (pub for testing)
  - `ActivitySummary::from_value(v: serde_json::Value) -> Result<ActivitySummary>` (pub for testing)

Key endpoint: `GET /activitylist-service/activities/search/activities?start=0&limit=100&sortField=startTimeInSeconds&sortOrder=DESC`

- [ ] **Step 5.1: Write failing tests**

At the bottom of `fitdownload/src/activities.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"[
        {
            "activityId": 12345678901,
            "activityName": "Morning Run",
            "startTimeLocal": "2026-06-30 07:00:00",
            "activityType": {"typeKey": "running"},
            "duration": 3600.0
        },
        {
            "activityId": 12345678902,
            "activityName": "Afternoon Ride",
            "startTimeLocal": "2026-06-29 15:00:00",
            "activityType": {"typeKey": "cycling"},
            "duration": 7200.0
        }
    ]"#;

    #[test]
    fn parse_activity_list() {
        let raw: Vec<serde_json::Value> = serde_json::from_str(SAMPLE).expect("parse");
        let activities: Vec<ActivitySummary> = raw
            .into_iter()
            .map(ActivitySummary::from_value)
            .collect::<Result<_>>()
            .expect("convert");
        assert_eq!(activities.len(), 2);
        assert_eq!(activities[0].id, 12_345_678_901);
        assert_eq!(activities[0].activity_type, "running");
        assert!((activities[0].duration_secs as f64 - 3600.0_f64).abs() < 1.0);
        assert_eq!(activities[0].date, "2026-06-30");
    }

    #[test]
    fn type_filter_applied() {
        let activities = vec![
            ActivitySummary {
                id: 1,
                name: "Run".into(),
                date: "2026-06-30".into(),
                activity_type: "running".into(),
                duration_secs: 3600,
            },
            ActivitySummary {
                id: 2,
                name: "Ride".into(),
                date: "2026-06-29".into(),
                activity_type: "cycling".into(),
                duration_secs: 7200,
            },
        ];
        let filtered = apply_type_filter(activities, &Some(vec!["running".to_owned()]));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].activity_type, "running");
    }
}
```

- [ ] **Step 5.2: Run to verify tests fail**

```bash
cargo nextest run -p fitdownload
```

Expected: compile errors.

- [ ] **Step 5.3: Implement activities.rs**

```rust
use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde_json::Value;

use crate::client::GarminClient;

const ACTIVITIES_PATH: &str =
    "/activitylist-service/activities/search/activities";
const PAGE_SIZE: usize = 100;

/// A summary of a single Garmin Connect activity.
#[derive(Debug, Clone)]
pub struct ActivitySummary {
    pub id: u64,
    pub name: String,
    /// Date in `YYYY-MM-DD` format.
    pub date: String,
    pub activity_type: String,
    pub duration_secs: u64,
}

/// Options for filtering the activity list.
pub struct ListOpts {
    pub limit: Option<usize>,
    pub after: Option<NaiveDate>,
    pub before: Option<NaiveDate>,
    pub types: Option<Vec<String>>,
}

impl ActivitySummary {
    /// Parse from a raw `Value` from the Garmin activities API.
    pub fn from_value(v: Value) -> Result<Self> {
        let id = v["activityId"]
            .as_u64()
            .context("missing or invalid activityId")?;
        let name = v["activityName"]
            .as_str()
            .unwrap_or("Unnamed")
            .to_owned();
        let start = v["startTimeLocal"].as_str().unwrap_or("");
        let date = start.split_whitespace().next().unwrap_or(start).to_owned();
        let activity_type = v["activityType"]["typeKey"]
            .as_str()
            .unwrap_or("unknown")
            .to_owned();
        let duration_secs = v["duration"].as_f64().map(|f| f as u64).unwrap_or(0);
        Ok(Self {
            id,
            name,
            date,
            activity_type,
            duration_secs,
        })
    }
}

/// Fetch all matching activities from Garmin Connect, with pagination.
///
/// Results are returned newest-first. Stops early when an `after` date boundary
/// is crossed (since the API returns results in descending date order).
pub async fn list_activities(
    client: &GarminClient,
    opts: &ListOpts,
) -> Result<Vec<ActivitySummary>> {
    let mut all = Vec::new();
    let mut start = 0_usize;
    let max = opts.limit.unwrap_or(usize::MAX);

    'pages: loop {
        let path = format!(
            "{ACTIVITIES_PATH}?start={start}&limit={PAGE_SIZE}\
             &sortField=startTimeInSeconds&sortOrder=DESC"
        );
        let page: Vec<Value> = client
            .get_json(&path)
            .await
            .with_context(|| format!("fetching activity page at offset {start}"))?;

        if page.is_empty() {
            break;
        }

        for raw in page {
            let activity = ActivitySummary::from_value(raw)?;

            // --after: results are newest-first, so once we pass this date we stop.
            if let Some(after) = opts.after {
                if let Ok(d) = NaiveDate::parse_from_str(&activity.date, "%Y-%m-%d") {
                    if d < after {
                        break 'pages;
                    }
                }
            }
            // --before: skip activities newer than the cutoff.
            if let Some(before) = opts.before {
                if let Ok(d) = NaiveDate::parse_from_str(&activity.date, "%Y-%m-%d") {
                    if d > before {
                        continue;
                    }
                }
            }

            all.push(activity);
            if all.len() >= max {
                break 'pages;
            }
        }

        start += PAGE_SIZE;
    }

    Ok(apply_type_filter(all, &opts.types))
}

/// Filter a list of activities to only the requested types (case-insensitive).
pub fn apply_type_filter(
    activities: Vec<ActivitySummary>,
    types: &Option<Vec<String>>,
) -> Vec<ActivitySummary> {
    match types {
        None => activities,
        Some(allowed) => {
            let lower: Vec<String> = allowed.iter().map(|t| t.to_lowercase()).collect();
            activities
                .into_iter()
                .filter(|a| lower.contains(&a.activity_type.to_lowercase()))
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    // paste tests from Step 5.1 here
}
```

- [ ] **Step 5.4: Run tests**

```bash
cargo nextest run -p fitdownload
```

Expected: both activity tests pass.

- [ ] **Step 5.5: Commit**

```bash
git add fitdownload/src/activities.rs
git commit -m "$(cat <<'EOF'
feat(fitdownload): add activity list module with pagination and filtering

Fetches activities in pages of 100, stopping early when --after date
is crossed. Supports type and date boundary filtering.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 6: FIT download module

**Files:**

- Create: `fitdownload/src/download.rs`

**Interfaces:**

- Consumes: `client::GarminClient`, `activities::ActivitySummary`
- Produces:
  - `activity_filename(activity: &ActivitySummary) -> String` — e.g. `"2026-06-30_running_12345678901.fit"`
  - `download_fit(client: &GarminClient, activity: &ActivitySummary, out_dir: &Path, overwrite: bool) -> Result<Option<PathBuf>>` — `None` = skipped (already exists)
  - `extract_fit_from_zip(data: &[u8]) -> Result<Vec<u8>>` (pub for testing)

Endpoint: `GET /download-service/export/fit/activity/{activityId}`

Response: binary FIT file, or a ZIP archive containing the FIT (detected by `PK` magic bytes at offset 0). Retry on HTTP 429 with exponential backoff (4 s, 4 s, 30 s, 30 s, 30 s; give up after 5 attempts).

- [ ] **Step 6.1: Write failing tests**

At the bottom of `fitdownload/src/download.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::activities::ActivitySummary;
    use std::io::Write;

    fn sample_activity() -> ActivitySummary {
        ActivitySummary {
            id: 12_345_678_901,
            name: "Morning Run".into(),
            date: "2026-06-30".into(),
            activity_type: "running".into(),
            duration_secs: 3600,
        }
    }

    #[test]
    fn filename_format() {
        assert_eq!(
            activity_filename(&sample_activity()),
            "2026-06-30_running_12345678901.fit"
        );
    }

    #[test]
    fn extract_fit_from_zip_finds_inner_file() {
        let mut zip_buf = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buf));
            w.start_file("12345678901.fit", zip::write::FileOptions::default())
                .expect("start file");
            w.write_all(b"FAKE_FIT_DATA").expect("write");
            w.finish().expect("finish");
        }
        let extracted = extract_fit_from_zip(&zip_buf).expect("extract");
        assert_eq!(extracted, b"FAKE_FIT_DATA");
    }

    #[tokio::test]
    async fn raw_fit_bytes_written_to_file() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/download-service/export/fit/activity/12345678901")
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(b"RAW_FIT_BYTES".as_ref())
            .create_async()
            .await;

        let session = crate::session::Session {
            cookies: vec![],
            csrf_header_value: Some("hdr".into()),
            imported_at: String::new(),
        };
        let client = crate::client::GarminClient::new_with_base(&session, &server.url())
            .expect("client");
        let dir = tempfile::tempdir().expect("tempdir");
        let path = download_fit(&client, &sample_activity(), dir.path(), false)
            .await
            .expect("download")
            .expect("should return a path");

        assert!(path.exists());
        assert_eq!(std::fs::read(&path).expect("read file"), b"RAW_FIT_BYTES");
        mock.assert_async().await;
    }
}
```

- [ ] **Step 6.2: Run to verify tests fail**

```bash
cargo nextest run -p fitdownload
```

Expected: compile errors.

- [ ] **Step 6.3: Implement download.rs**

```rust
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};

use crate::activities::ActivitySummary;
use crate::client::GarminClient;

const DOWNLOAD_PATH: &str = "/download-service/export/fit/activity";
const MAX_RETRIES: u32 = 5;
const RETRY_SHORT_SECS: u64 = 4;
const RETRY_LONG_SECS: u64 = 30;

/// Generate a filesystem-safe filename for an activity's FIT file.
///
/// Format: `YYYY-MM-DD_type_id.fit`
pub fn activity_filename(activity: &ActivitySummary) -> String {
    format!("{}_{}_{}.fit", activity.date, activity.activity_type, activity.id)
}

/// Download the FIT file for an activity to `out_dir`.
///
/// Returns `Ok(None)` if the file already exists and `overwrite` is `false`.
/// Returns `Ok(Some(path))` on a successful download.
pub async fn download_fit(
    client: &GarminClient,
    activity: &ActivitySummary,
    out_dir: &Path,
    overwrite: bool,
) -> Result<Option<PathBuf>> {
    let filename = activity_filename(activity);
    let dest = out_dir.join(&filename);

    if dest.exists() && !overwrite {
        log::debug!("skipping {} (already exists)", filename);
        return Ok(None);
    }

    let path = format!("{DOWNLOAD_PATH}/{}", activity.id);
    let bytes = retry_get_bytes(client, &path)
        .await
        .with_context(|| format!("downloading activity {}", activity.id))?;

    // Zip magic bytes: PK (0x50 0x4B)
    let fit_bytes = if bytes.starts_with(b"PK") {
        extract_fit_from_zip(&bytes)
            .with_context(|| format!("extracting FIT from zip for activity {}", activity.id))?
    } else {
        bytes.to_vec()
    };

    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("creating output directory {}", out_dir.display()))?;
    std::fs::write(&dest, &fit_bytes)
        .with_context(|| format!("writing {}", dest.display()))?;

    log::info!("saved {}", filename);
    Ok(Some(dest))
}

/// Extract the first `.fit` file from a zip archive.
pub fn extract_fit_from_zip(data: &[u8]) -> Result<Vec<u8>> {
    let cursor = std::io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(cursor).context("opening zip archive")?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).context("reading zip entry")?;
        if entry.name().to_lowercase().ends_with(".fit") {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf).context("reading .fit from zip")?;
            return Ok(buf);
        }
    }
    anyhow::bail!("no .fit file found inside zip archive")
}

/// GET bytes with exponential backoff on HTTP 429 (rate-limited).
async fn retry_get_bytes(client: &GarminClient, path: &str) -> Result<bytes::Bytes> {
    let mut consecutive_429s = 0_u32;
    for attempt in 0..MAX_RETRIES {
        match client.get_bytes(path).await {
            Ok(bytes) => return Ok(bytes),
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("429") {
                    consecutive_429s += 1;
                    let delay = if consecutive_429s >= 2 {
                        RETRY_LONG_SECS
                    } else {
                        RETRY_SHORT_SECS
                    };
                    log::warn!(
                        "rate limited on attempt {}/{MAX_RETRIES}; sleeping {delay}s",
                        attempt + 1,
                    );
                    tokio::time::sleep(Duration::from_secs(delay)).await;
                } else {
                    return Err(e);
                }
            }
        }
    }
    anyhow::bail!("gave up after {MAX_RETRIES} retries due to rate limiting")
}

#[cfg(test)]
mod tests {
    // paste tests from Step 6.1 here
}
```

- [ ] **Step 6.4: Run tests**

```bash
cargo nextest run -p fitdownload
```

Expected: all 3 download tests pass (`filename_format`, `extract_fit_from_zip_finds_inner_file`, `raw_fit_bytes_written_to_file`).

- [ ] **Step 6.5: Commit**

```bash
git add fitdownload/src/download.rs
git commit -m "$(cat <<'EOF'
feat(fitdownload): add FIT download module with zip handling and retry

Downloads activity FIT files; handles both raw FIT and zip-wrapped
responses; retries on 429 with exponential backoff.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 7: Wire up commands

**Files:**

- Modify: `fitdownload/src/cli.rs` — replace stub `cmd_auth` and `cmd_download` with real implementations

- [ ] **Step 7.1: Replace cmd_auth stub in cli.rs**

```rust
pub async fn cmd_auth(args: AuthArgs) -> anyhow::Result<()> {
    use crate::session::{session_path, Session};

    if args.status {
        let path = session_path();
        match Session::load_from(&path) {
            Ok(session) => {
                let valid = session.is_valid();
                println!("Session file : {}", path.display());
                println!("Cookies      : {}", session.cookies.len());
                println!(
                    "CSRF value   : {}",
                    session.csrf_header_value.as_deref().unwrap_or("(none — will fetch from page)")
                );
                println!("Imported at  : {}", session.imported_at);
                println!("Valid        : {valid}");
                if !valid {
                    anyhow::bail!(
                        "session is expired — re-import cookies with:\n  \
                         fitdownload auth --cookies /path/to/cookies.txt"
                    );
                }
            }
            Err(e) => {
                println!("No session found ({e})");
                println!();
                println!("To create one:");
                println!("  1. Log in to connect.garmin.com in Chrome or Firefox.");
                println!("  2. Export cookies with the 'Get cookies.txt LOCALLY' extension.");
                println!("  3. Run: fitdownload auth --cookies /path/to/cookies.txt");
            }
        }
        return Ok(());
    }

    let cookies_path = args.cookies.ok_or_else(|| {
        anyhow::anyhow!(
            "provide --cookies <FILE> with a Netscape cookies.txt exported from your browser.\n\
             Tip: use the 'Get cookies.txt LOCALLY' Chrome/Firefox extension on connect.garmin.com"
        )
    })?;

    let session = Session::from_cookies_file(&cookies_path)
        .with_context(|| format!("reading {}", cookies_path.display()))?;

    let count = session.cookies.len();
    session.save().context("saving session")?;

    println!("Imported {count} cookies");
    match &session.csrf_header_value {
        Some(v) => println!("CSRF value found in cookies: {}...", &v[..v.len().min(8)]),
        None => println!("No CSRF value in cookies — will be fetched from Garmin Connect on first use"),
    }
    println!("Session saved to {}", crate::session::session_path().display());
    Ok(())
}
```

- [ ] **Step 7.2: Replace cmd_download stub in cli.rs**

```rust
pub async fn cmd_download(args: DownloadArgs) -> anyhow::Result<()> {
    use crate::activities::{list_activities, ListOpts};
    use crate::client::GarminClient;
    use crate::download::download_fit;
    use crate::session::Session;
    use chrono::NaiveDate;
    use indicatif::{ProgressBar, ProgressStyle};

    let session = Session::load().context(
        "no session found — run: fitdownload auth --cookies /path/to/cookies.txt",
    )?;
    if !session.is_valid() {
        anyhow::bail!(
            "session is expired — re-import cookies with:\n  \
             fitdownload auth --cookies /path/to/cookies.txt"
        );
    }

    let mut client = GarminClient::new(&session).context("building HTTP client")?;

    // If no CSRF value was found in the session, fetch it from the page.
    if session.csrf_header_value.is_none() {
        log::info!("no CSRF value in session; fetching from Garmin Connect...");
        client.refresh_csrf().await?;
    }

    let after = args
        .after
        .as_deref()
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .context("invalid --after date; use YYYY-MM-DD")?;
    let before = args
        .before
        .as_deref()
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .context("invalid --before date; use YYYY-MM-DD")?;
    let types = args.types.as_deref().map(|t| {
        t.split(',')
            .map(str::trim)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
    });

    log::info!("fetching activity list...");
    let opts = ListOpts { limit: args.limit, after, before, types };
    let activities = list_activities(&client, &opts).await?;

    if activities.is_empty() {
        println!("No activities matched the given filters.");
        return Ok(());
    }

    println!("Downloading {} activities to {}", activities.len(), args.output_dir.display());
    let pb = ProgressBar::new(activities.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar()),
    );

    let mut downloaded = 0_usize;
    let mut skipped = 0_usize;
    for activity in &activities {
        pb.set_message(format!(
            "{} {} ({})",
            activity.date, activity.activity_type, activity.id
        ));
        match download_fit(&client, activity, &args.output_dir, args.overwrite).await? {
            Some(_) => downloaded += 1,
            None => skipped += 1,
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");
    println!("Downloaded: {downloaded}  Skipped (already exist): {skipped}");
    Ok(())
}
```

- [ ] **Step 7.3: Run full test suite**

```bash
cargo nextest run -p fitdownload
```

Expected: all tests pass.

- [ ] **Step 7.4: Lint**

```bash
cargo lclippy -p fitdownload -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
```

Fix all warnings.

- [ ] **Step 7.5: Build release binary**

```bash
cargo lbuild --release -p fitdownload --color always
```

- [ ] **Step 7.6: Smoke test `auth --status` with no session**

```bash
./target/release/fitdownload auth --status
```

Expected: prints "No session found" with setup instructions. Exit 0.

- [ ] **Step 7.7: Commit**

```bash
git add fitdownload/src/cli.rs
git commit -m "$(cat <<'EOF'
feat(fitdownload): wire up auth and download commands end-to-end

cmd_auth imports cookies and shows session status; cmd_download
fetches the activity list and downloads FIT files with a progress bar.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 8: Justfile and final polish

**Files:**

- Modify: `justfile` — add `fitdownload` to release install targets

- [ ] **Step 8.1: Update justfile release recipes**

Find the `release` and `releasea` recipes and add `fitdownload` to the `cargo install` invocations, following the same pattern as the other crates.

- [ ] **Step 8.2: Run full workspace check**

```bash
cargo lcheck --color always
cargo nextest run
```

Expected: all workspace crates compile and all tests pass.

- [ ] **Step 8.3: Commit and push**

```bash
git add justfile
git commit -m "$(cat <<'EOF'
chore: add fitdownload to justfile release targets

Ensures 'just release' and 'just releasea' install the fitdownload binary
alongside the other fitutils binaries.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
git push -u origin feat/fitdownload
```

---

## Verification

### Unit tests (no network required)

```bash
cargo nextest run -p fitdownload
```

All tests pass. No real Garmin servers are contacted.

### Manual end-to-end test (requires a real Garmin account)

1. In Chrome or Firefox, navigate to **connect.garmin.com** and log in (with MFA if enabled).
2. Install the **"Get cookies.txt LOCALLY"** extension and click it while on `connect.garmin.com`. Save the file.
3. Import the session:

   ```bash
   ./target/release/fitdownload auth --cookies ~/Downloads/cookies.txt
   ```

   Expected: `Imported N cookies` message.
4. Verify the session:

   ```bash
   ./target/release/fitdownload auth --status
   ```

   Expected: `Valid: true`.
5. Download the 5 most recent running activities:

   ```bash
   mkdir -p /tmp/fit-test
   ./target/release/fitdownload download \
       --output-dir /tmp/fit-test \
       --limit 5 \
       --types running
   ```

   Expected: 5 `.fit` files named `YYYY-MM-DD_running_<id>.fit`.
6. Validate each file with `fitview`:

   ```bash
   for f in /tmp/fit-test/*.fit; do
       ./target/release/fitview "$f" && echo "OK: $f"
   done
   ```

---

## Known Limitations and Future Work

| Limitation | Notes |
| --- | --- |
| **Session expiry** | Browser cookies last a few hours to days. When expired, `auth --status` reports `Valid: false`. Re-export and re-import. |
| **MFA** | Handled naturally — user completes MFA in the real browser before exporting cookies. |
| **Cloudflare changes** | Garmin's blocking rules evolve. If downloads start returning 403, check the `python-garminconnect` GitHub issues for updated endpoint paths. |
| **CSRF HTML fallback** | `refresh_csrf()` is only called when no CSRF value is in the session. If the cookie-extracted value stops working, call it unconditionally. |
| **Incremental sync** | Future `--since-last` flag: scan the output directory for the newest `*_<id>.fit`, derive the activity ID, and only fetch activities newer than it. |
| **garmin_client crate** | Not used — it relies on the pre-March-2026 SSO flow which is now blocked by Cloudflare TLS fingerprinting. |
| **Official Activity API** | If you gain access to the Garmin Connect Developer Program, the official Activity API at `developer.garmin.com/gc-developer-program/activity-api/` includes FIT/GPX/TCX downloads via OAuth2 PKCE. Migrate `client.rs` to use `GET /wellness-api/rest/activities` with a `Bearer` header. The session and cookie modules become unnecessary. |

---

## Appendix: Official API Alternative (if developer access is granted)

If you are accepted into the **Garmin Connect Developer Program** (`developer.garmin.com/gc-developer-program/`), you can replace the cookie-based auth with the official OAuth2 PKCE flow:

1. Register your app in the Garmin developer portal — you receive a `client_id` and `client_secret`.
2. Redirect the user to: `https://apis.garmin.com/tools/oauth2/authorizeUser?client_id=...&redirect_uri=...&response_type=code&code_challenge=...`
3. Exchange the authorization code at: `POST https://diauth.garmin.com/di-oauth2-service/oauth/token`
4. Access activities: `GET https://apis.garmin.com/wellness-api/rest/activities` with `Authorization: Bearer <access_token>`.
5. Download FIT files: `GET https://apis.garmin.com/wellness-api/rest/activities/{activityId}/fit`.

The `client.rs` module is the only file that needs to change — swap the cookie-store reqwest setup for a standard `Authorization: Bearer` header. All other modules (`activities.rs`, `download.rs`, `session.rs`, `cli.rs`) remain valid.

To apply for the developer program: visit `developer.garmin.com/gc-developer-program/` and use their contact/application form. As of July 2026 the program shows "stay tuned for more updates" so approval timelines are unknown.
