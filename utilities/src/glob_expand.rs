use glob::glob;
use std::collections::BTreeSet;

/// Expands a list of file-path strings and glob patterns into a sorted,
/// deduplicated list of existing file paths.
///
/// Each element of `patterns` may be a literal path or a shell-style glob
/// (`*`, `?`, `[a-z]`). If an element matches no files — whether because it
/// is a glob that matches nothing or a literal path that does not exist — a
/// `warn!` log is emitted and that element contributes nothing to the output.
///
/// The result is sorted lexicographically and deduplicated so that processing
/// order is deterministic regardless of filesystem iteration order.
///
/// # Examples
///
/// ```no_run
/// # use utilities::expand_globs;
/// let files = expand_globs(&["*.fit".to_string()]);
/// // returns sorted Vec<String> of matched paths
/// ```
#[must_use]
pub fn expand_globs(patterns: &[String]) -> Vec<String> {
    let mut result: BTreeSet<String> = BTreeSet::new();

    for pattern in patterns {
        match glob(pattern) {
            Err(e) => log::warn!("Invalid glob pattern '{pattern}': {e}"),
            Ok(entries) => {
                let mut matched = false;
                for entry in entries {
                    match entry {
                        Ok(path) => {
                            matched = true;
                            result.insert(path.to_string_lossy().into_owned());
                        }
                        Err(e) => log::warn!("Glob error in '{pattern}': {e}"),
                    }
                }
                if !matched {
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
