use std::{collections::HashMap, error::Error, path::Path};
use utilities::get_extension;

/// Performs template substitution on a pattern string using the provided values.
/// Replaces `{%key}` placeholders with metadata values and sanitizes path separators.
fn substitute_pattern<S: ::std::hash::BuildHasher>(
    pattern: &str,
    values: &HashMap<String, String, S>,
) -> String {
    // Sort keys longest-first so that e.g. "%month" is replaced before "%mo"
    let mut keys: Vec<&String> = values.keys().collect();
    keys.sort_by_key(|k| std::cmp::Reverse(k.len()));

    let mut result = pattern.to_string();
    for key in keys {
        let fixed_value = values[key].trim().to_string();
        // Support both {%key} (braced) and %key (bare) syntax
        let braced_key = format!("{{{key}}}");
        log::debug!("substitute_pattern() -- key: {key}, fixed_value: {fixed_value}");
        result = result
            .replace(&braced_key, &fixed_value)
            .replace(key.as_str(), &fixed_value)
            .replace(['/', '\\'], "-");
    }
    result
}

/// Renames (and optionally moves) the target file based on the provided patterns.
///
/// # Arguments
///
/// - `filename` - the file to be renamed
/// - `rename_pattern` - the pattern for the new file name
/// - `move_pattern` - optional pattern for the target directory (None = rename in place)
/// - `values` - a `HashMap` with key/value pairs of the replacement values
/// - `unique_val` - starting counter for collision avoidance
/// - `dry_run` - whether this is a dry run
///
/// # Returns
///
/// `Result<String>` containing the new file path.
///
/// # Errors
///
/// Returns an error if renaming/moving fails or target directory can't be created.
pub fn rename_and_move<S: ::std::hash::BuildHasher>(
    filename: &str,
    rename_pattern: &str,
    move_pattern: Option<&str>,
    values: &HashMap<String, String, S>,
    unique_val: usize,
    dry_run: bool,
) -> Result<String, Box<dyn Error>> {
    log::debug!("rename_and_move() -- rename_pattern: {rename_pattern}");
    log::debug!("rename_and_move() -- move_pattern: {move_pattern:?}");

    // 1. Substitute the rename pattern to get the new filename stem
    let new_stem = substitute_pattern(rename_pattern, values);
    log::debug!("rename_and_move() -- new_stem: {new_stem}");

    // 2. Determine target directory
    let target_dir = if let Some(mp) = move_pattern {
        let mut target = substitute_pattern(mp, values);
        // Extra sanitization for directory paths
        target = target.replace("..", "_");
        log::debug!("rename_and_move() -- target dir: {target}");

        let target_path = Path::new(&target);
        if !target_path.exists() {
            if dry_run {
                log::debug!("dr: mkdir -p {target}");
            } else {
                std::fs::create_dir_all(target_path)
                    .map_err(|e| format!("Unable to create directory '{target}': {e}"))?;
                log::debug!("mkdir -p {target}");
            }
        } else if !dry_run && !target_path.is_dir() {
            return Err(format!("Target path '{target}' is not a directory.").into());
        }

        target_path.to_path_buf()
    } else {
        // No move — stay in the original file's directory
        Path::new(filename)
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    };

    // 3. Assemble final path: target_dir / new_stem.original_ext
    let ext = get_extension(filename);
    let mut final_path = target_dir.join(Path::new(&new_stem).with_extension(&ext));
    log::debug!("rename_and_move() -- final_path: {}", final_path.display());

    // 4. Collision avoidance
    let mut counter = unique_val;
    while final_path.exists() {
        log::warn!(
            "{} already exists. Appending unique identifier.",
            final_path.display()
        );
        let new_name = format!("{new_stem} ({counter}).{ext}");
        final_path = target_dir.join(new_name);
        counter += 1;
    }

    // 5. Perform the rename/move as a single operation
    let final_str = final_path.to_string_lossy();
    if dry_run {
        log::debug!("dr: {filename} --> {final_str}");
    } else {
        std::fs::rename(filename, &final_path)
            .map_err(|e| format!("Unable to rename '{filename}' to '{final_str}': {e}"))?;
        log::debug!("{filename} --> {final_str}");
    }

    Ok(final_path.to_string_lossy().to_string())
}
