---
tags:
  - note
  - deps
  - maintenance
  - investigation
aliases: []
doc_id: FIT-001
doc_name: chrono-jiff-migration-investigation
crumb_id: fit-ce6
document_title: "Investigation: chrono → time/jiff migration and fitparser switch"
created_date: 2026-07-04
synopsis: >
  Investigates whether fitutils should migrate from chrono to time or jiff, and
  whether fitparser should be replaced with rustyfit. Conclusion: migration is
  blocked by fitparser's own chrono dependency; switching parsers is not warranted.
  Track via crumb fit-ce6.
status: complete
type: Note
revision: "1.1"
review_date:
reviewed_by: []
completed_date: 2026-07-04
comments:
---

# Investigation: chrono → time/jiff migration and fitparser switch

## Context

The premise that chrono is "soft-deprecated" is partially true but overstated:
- chrono 0.4 is in **maintenance mode** (bugfixes and security patches, no new architecture)
- The old RUSTSEC-2020-0159 advisory (`localtime_r` unsoundness) was fixed in 0.4.20 and is long irrelevant at 0.4.45
- There is no formal deprecation notice; the library continues to receive releases
- The concern is more about chrono's dated API design than about abandonment

## Key Finding: Migration is Blocked by fitparser

**fitparser 0.11.0** (Cargo.lock, line 356) still depends on chrono:

```
[[package]]
name = "fitparser"
version = "0.11.0"
dependencies = [
 "chrono",
 "nom",
 "serde",
]
```

`fitparser::Value::Timestamp` wraps a `chrono::DateTime<Local>`. As long as fitparser uses
chrono, **chrono cannot be removed from the dependency tree**. Migrating `utilities/` to
`jiff` or `time` while fitparser stays on chrono would:

1. Keep chrono in `Cargo.lock` anyway (via fitparser)
2. Require an explicit chrono → jiff/time conversion at the fitparser boundary
3. Add a second time-handling library alongside chrono — net increase in complexity

The two other time crates already in the tree (`jiff 0.2.31` via **env_logger 0.11.11**,
`time 0.3.53` via the **gpx** crate) are incidental — they do not represent migration
opportunities.

## Recommendation: Don't migrate now

There is no benefit-to-cost case for migrating `utilities/` while fitparser keeps pulling
in chrono. The existing chrono 0.4.45 usage is sound, serde works (the custom `Duration(f64)`
newtype solved the one real pain point), and there are no open RUSTSEC advisories against
the current version.

**Action taken:** Crumb **fit-ce6** created to track fitparser's chrono dependency.

## Future migration path (when fitparser drops chrono)

Target: **jiff** (not `time`). Reasons:
- Richer IANA timezone handling; `Zoned` is a better model than `DateTime<Local>`
- Authored by Andrew Gallant (BurntSushi) — consistent track record
- `jiff::SignedDuration` has serde support, potentially eliminating the `Duration(f64)` workaround
- More active development trajectory than `time 0.3`

### Scope (all changes confined to `utilities/`)

| File | Changes needed |
|------|----------------|
| `Cargo.toml` (workspace) | `chrono = "0.4"` → `jiff = "0.2"` |
| `utilities/Cargo.toml` | `chrono = { workspace = true }` → `jiff = { workspace = true }` |
| `utilities/src/datetime_keys.rs` | Drop `Datelike`/`Timelike` traits; call `.date().year()`, `.time().hour()` etc. on `jiff::Zoned` |
| `utilities/src/duration.rs` | Replace `DateTime<Local>`, `chrono::Duration` with `jiff::Zoned`, `jiff::SignedDuration` |
| `utilities/src/fit/activity.rs` | Replace `Local`, `TimeZone` |
| `utilities/src/fit/session.rs` | ⚠ Most complex: replace `DateTime<Local>` fields, `chrono::Duration::seconds()` arithmetic, serde attrs |
| `utilities/src/fit/record.rs` | Replace `DateTime<Local>` field + serde attr |
| `utilities/src/fit/lap.rs` | Minor import cleanup |
| `utilities/src/gpx/activity.rs` | Replace `Local`, `TimeZone` |
| `utilities/src/gpx/gpxmetadata.rs` | ⚠ Most complex: `Local::now()`, RFC3339 parsing, `Datelike::year()`, `.with_timezone()` |
| `utilities/src/gpx/track.rs` | Replace `DateTime<Local>` field |
| `utilities/src/tcx/activity.rs` | Replace `DateTime<Local>` field |
| `utilities/src/tcx/trackpoints.rs` | ⚠ Most complex: RFC3339 parsing, timezone conversion, epoch fallback |

### Key API mapping: chrono → jiff

| chrono | jiff |
|--------|------|
| `DateTime<Local>` | `jiff::Zoned` |
| `DateTime<FixedOffset>` | `jiff::Zoned` (offset zone) |
| `Local::now()` | `jiff::Zoned::now()` |
| `Local.timestamp_opt(0, 0).unwrap()` | `jiff::Timestamp::UNIX_EPOCH.to_zoned(jiff::tz::TimeZone::system())` |
| `DateTime::parse_from_rfc3339(s)` | `s.parse::<jiff::Timestamp>()?.to_zoned(jiff::tz::TimeZone::system())` |
| `.with_timezone(&Local)` | `.with_time_zone(jiff::tz::TimeZone::system())` |
| `Datelike::year()` | `.date().year()` (returns `i16`) |
| `Datelike::month()` (u32) | `.date().month() as u32` (returns `i8`) |
| `Datelike::day()` (u32) | `.date().day() as u32` (returns `i8`) |
| `Datelike::weekday()` | `.date().weekday()` |
| `Timelike::hour()` | `.time().hour() as u32` |
| `Timelike::hour12()` | `let h = z.time().hour(); (h >= 12, if h % 12 == 0 { 12 } else { h % 12 })` (chrono returns `(is_pm, 1..=12)`) |
| `Timelike::minute()` | `.time().minute() as u32` |
| `Timelike::second()` | `.time().second() as u32` |
| `chrono::Duration::seconds(n)` | `jiff::Span::new().seconds(n)` |
| `dt.signed_duration_since(other)` | `other.duration_until(&dt)` → `jiff::SignedDuration` |
| `dt + chrono::Duration` | `dt.checked_add(jiff::Span::...)` |
| Serde on `DateTime<Local>` field | Enable jiff's `serde` feature; `jiff::Zoned` implements `Serialize`/`Deserialize` directly, so no attribute is needed |

JSON output changes unless handled: chrono's `DateTime<Local>` serialises as RFC 3339
(`2024-01-01T10:00:00+01:00`), while jiff's `Zoned` uses RFC 9557 with an IANA zone
annotation (`2024-01-01T10:00:00+01:00[Europe/Oslo]`). To keep the output identical,
serialise a `jiff::Timestamp` or an offset-only representation instead. At the fitparser boundary, convert once:

```rust
// fitparser::Value::Timestamp(chrono_dt) → jiff::Zoned
let zoned = jiff::Timestamp::from_second(chrono_dt.timestamp())
    .expect("valid timestamp")
    .to_zoned(jiff::tz::TimeZone::system());
```

### Verification (when the time comes)

```bash
cargo lcheck --color always          # incremental as you go
cargo nextest run                     # full suite
cargo lclippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
# Smoke-test: run fit2json on a sample .fit file, diff JSON output before/after
```

---

## Addendum: Should we switch from fitparser to rustyfit?

**Recommendation: No.**

### What rustyfit is

`rustyfit` (v0.5.0, ~2,900 all-time downloads) is a Rust rewrite of the author's Go FIT
implementation. It is actively developed but young. Key characteristics:

| | fitparser 0.11 | rustyfit 0.5 |
|---|---|---|
| All-time downloads | ~38,000 | ~2,900 |
| Write support | No | **Yes** (Encoder + EncoderBuilder) |
| API model | Eager: `from_reader() → Vec<FitDataRecord>` | Streaming: `StreamDecoder` + `StreamingIterator` |
| Message types | `MesgNum` enum (profile-generated) | `profile` module (generated from Profile.xlsx) |
| Timestamp dep | `chrono::DateTime<Local>` (confirmed) | **None** — raw numeric values, consumer converts |

### Why the API difference matters

rustyfit uses an **event-streaming model** (`DecoderEvent::FileHeader`, `MessageDefinition`,
`Message`, `CRC`) rather than fitparser's eager `Vec<FitDataRecord>`. Every FIT parsing
site in `utilities/src/fit/` would need to be restructured — an architectural rewrite of
the parsing layer, not a mechanical substitution.

### Reasons not to switch

1. **High migration cost, unclear gain.** Streaming API is a paradigm shift; all of
   `utilities/src/fit/` needs restructuring.
2. **rustyfit has zero time library dependencies** (confirmed 2026-07-04 via Cargo.lock).
   It returns raw numeric timestamp values; the consumer is responsible for conversion.
   Switching to rustyfit + jiff would remove chrono from the tree entirely *and* give
   full freedom to use jiff natively — a stronger position than a utilities-only migration.
   However, the streaming API rewrite cost and lower maturity still outweigh this benefit
   for now.
3. **~13× lower adoption.** fitparser has far more real-world FIT file coverage.
4. **Write support is irrelevant.** fitutils is read-only.
5. **fitparser is actively maintained** (last publish ~1 month before this investigation).

### Other crates noted (for future reference)

- **`fit-rust`** — supports read + write + merge
- **`fit_file`** — callback-based streaming; memory-efficient for very large files

### Conclusion

Stay on fitparser. Track the chrono dependency via crumb **fit-ce6**. The migration
trigger is fitparser dropping chrono — not a parser switch.

## Revisions

| Revision | Date | Notes |
| --- | --- | --- |
| 1.0 | 2026-07-04 | Initial investigation |
| 1.1 | 2026-10-09 | Corrected the hour12 and serde rows and the JSON output claim; H1 matches title |
