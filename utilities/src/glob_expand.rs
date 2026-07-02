use glob::glob;
use std::{collections::BTreeSet, path::Path};

/// Expands a list of file-path strings and glob patterns into a sorted,
/// deduplicated list of existing file paths.
///
/// Each element of `patterns` is resolved as follows:
/// 1. If the string names an existing file or directory on disk, it is included
///    directly without glob interpretation. This preserves filenames that
///    contain glob metacharacters (e.g. `"file[1].fit"` on Unix).
/// 2. Otherwise the string is treated as a shell-style glob (`*`, `?`,
///    `[a-z]`). If it matches nothing, a `warn!` is emitted and the element
///    contributes nothing to the output.
///
/// The result is sorted lexicographically and deduplicated so that processing
/// order is deterministic regardless of filesystem iteration order.
///
/// # Caveats
///
/// Paths returned by glob matches are converted with [`std::path::Path::to_string_lossy`],
/// which replaces non-UTF-8 bytes with `U+FFFD`. If a glob pattern matches
/// a file whose path is not valid UTF-8, the returned string will be lossy and
/// may not round-trip back to the original path. Callers that need to open
/// matched files should be aware of this limitation. Literal paths supplied
/// directly (i.e. those that already exist on disk) bypass this conversion and
/// are returned as-is.
///
/// # Examples
///
/// ```no_run
/// # use utilities::expand_globs;
/// let files = expand_globs(&["*.fit"]);
/// // returns sorted Vec<String> of matched paths
/// ```
#[must_use]
pub fn expand_globs<S: AsRef<str>>(patterns: &[S]) -> Vec<String> {
    let mut result: BTreeSet<String> = BTreeSet::new();

    for pattern in patterns {
        let pattern = pattern.as_ref();
        // If the path already exists on disk, include it as-is without glob
        // interpretation so that filenames containing metacharacters are safe.
        if Path::new(pattern).exists() {
            result.insert(pattern.to_owned());
            continue;
        }

        match glob(pattern) {
            Err(e) => log::warn!("Invalid glob pattern '{pattern}': {e}"),
            Ok(entries) => {
                // `any_entry` tracks whether the iterator produced at least one
                // candidate (success or permission error).  The "no match"
                // warning must only fire when the glob found zero candidates;
                // when entries exist but all error (e.g. permission denied) we
                // have already warned per-entry and should not also claim the
                // pattern matched nothing.
                let mut any_entry = false;
                for entry in entries {
                    any_entry = true;
                    match entry {
                        Ok(path) => {
                            result.insert(path.to_string_lossy().into_owned());
                        }
                        Err(e) => log::warn!("Glob error in '{pattern}': {e}"),
                    }
                }
                if !any_entry {
                    log::warn!("No files matched pattern: {pattern}");
                }
            }
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    fn make_files(dir: &Path, names: &[&str]) {
        for name in names {
            fs::write(dir.join(name), b"").expect("create temp file");
        }
    }

    fn temp_dir(suffix: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("fitutils_glob_{}_{suffix}", std::process::id()));
        // Remove any stale directory from a previous crashed run before creating
        // a fresh one, so leftover files never influence test results.
        if d.exists() {
            fs::remove_dir_all(&d).expect("remove stale temp dir");
        }
        fs::create_dir_all(&d).expect("create temp dir");
        d
    }

    #[test]
    fn test_literal_path_returns_itself() {
        let tmp = temp_dir("literal");
        let file = tmp.join("test.fit");
        fs::write(&file, b"").unwrap();

        let path_str = file.to_string_lossy().into_owned();
        let result = expand_globs(&[path_str.clone()]);
        assert_eq!(result, vec![path_str]);

        fs::remove_file(file).unwrap();
        fs::remove_dir(tmp).unwrap();
    }

    #[test]
    fn test_glob_results_are_sorted() {
        let tmp = temp_dir("sorted");
        make_files(&tmp, &["c.fit", "a.fit", "b.fit"]);

        let pattern = tmp.join("*.fit").to_string_lossy().into_owned();
        let result = expand_globs(&[pattern]);

        let mut expected: Vec<String> = ["a.fit", "b.fit", "c.fit"]
            .iter()
            .map(|n| tmp.join(n).to_string_lossy().into_owned())
            .collect();
        expected.sort();
        assert_eq!(result, expected);

        for n in &["a.fit", "b.fit", "c.fit"] {
            fs::remove_file(tmp.join(n)).unwrap();
        }
        fs::remove_dir(tmp).unwrap();
    }

    #[test]
    fn test_overlapping_patterns_deduplicated() {
        let tmp = temp_dir("dedup");
        let file = tmp.join("2024_run.fit");
        fs::write(&file, b"").unwrap();

        let exact = file.to_string_lossy().into_owned();
        let glob_pat = tmp.join("*.fit").to_string_lossy().into_owned();
        // Same file matched by both the literal path and the glob
        let result = expand_globs(&[exact.clone(), glob_pat]);
        assert_eq!(result, vec![exact]);

        fs::remove_file(file).unwrap();
        fs::remove_dir(tmp).unwrap();
    }

    #[test]
    fn test_no_match_returns_empty() {
        let tmp = temp_dir("nomatch");
        let pattern = tmp.join("*.fit").to_string_lossy().into_owned();
        let result = expand_globs(&[pattern]);
        assert!(result.is_empty());
        fs::remove_dir(tmp).unwrap();
    }

    #[test]
    fn test_literal_with_metacharacters_not_interpreted_as_glob() {
        // A filename that contains glob metacharacters must be treated as a
        // literal path when the file exists, not expanded as a pattern.
        let tmp = temp_dir("metachar");
        let file = tmp.join("run[1].fit");
        fs::write(&file, b"").unwrap();

        let path_str = file.to_string_lossy().into_owned();
        let result = expand_globs(&[path_str.clone()]);
        assert_eq!(result, vec![path_str], "metacharacter filename must not be glob-expanded");

        fs::remove_file(file).unwrap();
        fs::remove_dir(tmp).unwrap();
    }

    #[test]
    fn test_mixed_literal_and_glob() {
        let tmp = temp_dir("mixed");
        make_files(&tmp, &["run.gpx", "ride.fit", "walk.fit"]);

        let literal = tmp.join("run.gpx").to_string_lossy().into_owned();
        let glob_pat = tmp.join("*.fit").to_string_lossy().into_owned();
        let result = expand_globs(&[literal.clone(), glob_pat]);

        let mut expected: Vec<String> = vec![
            tmp.join("ride.fit").to_string_lossy().into_owned(),
            tmp.join("run.gpx").to_string_lossy().into_owned(),
            tmp.join("walk.fit").to_string_lossy().into_owned(),
        ];
        expected.sort();
        assert_eq!(result, expected);

        for n in &["run.gpx", "ride.fit", "walk.fit"] {
            fs::remove_file(tmp.join(n)).unwrap();
        }
        fs::remove_dir(tmp).unwrap();
    }
}
