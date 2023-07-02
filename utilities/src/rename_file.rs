use std::{collections::HashMap, error::Error, path::Path};

/// Renames the target file based on the provided patterntar
///
/// # Arguments
///
/// - `filename: &str` - the file to be renamed
/// - `pattern: &str` - the pattern upon which the new file name will be based
/// - `values: &HashMap<String, String>` - a `HashMap` with key/value pairs of the replacement values for the pattern
/// - `unique_val: usize` - If necessary, we can append a unique value to ensure file name uniqueness.
/// - `dry_run: bool` - An indicator of whether this is a dry run or not.
///
/// # Returns
///
/// Result<String> containing the new file name.
///
/// # Errors
///
/// An error message if we're unable to rename the file.
///
/// # Panics
///
/// None.
pub fn rename_file<S: ::std::hash::BuildHasher>(
    filename: &str,
    pattern: &str,
    values: &HashMap<String, String, S>,
    unique_val: usize,
    dry_run: bool,
) -> Result<String, Box<dyn Error>> {
    let mut new_filename = pattern.to_string();

    for (key, value) in values {
        // Make sure to pad disc and track numbers with leading zeros
        let fixed_value = value.clone().trim().to_string();

        // Do the actual filename replacement
        new_filename
            .replace(key, &fixed_value)
            .replace('/', "-")
            .trim()
            .to_string();
    }

    // Get the path before the filename (eg. "music/01.flac" returns "music/")
    let parent = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Create the new filename
    let mut new_path =
        parent.join(Path::new(&new_filename).with_extension(crate::get_extension(filename)));
    log::debug!("new_path = {new_path:?}");

    // Check if a file with the new filename already exists - make the filename unique if it does.
    if Path::new(&new_path).exists() {
        log::warn!("{new_filename} already exists. Appending unique identifier.");
        new_filename = format!("{new_filename} ({unique_val})");
        new_path =
            parent.join(Path::new(&new_filename).with_extension(crate::get_extension(filename)));
    }

    // Perform the actual rename
    let npl = new_path.to_string_lossy();
    if dry_run {
        log::debug!("dr: {filename} --> {npl}");
    } else {
        // Perform rename
        let rn_res = std::fs::rename(filename, &new_path);
        match rn_res {
            Ok(_) => log::debug!("{filename} --> {npl}"),
            Err(err) => {
                return Err(
                    format!("Unable to rename {filename} to {npl}. Error message: {err}").into(),
                );
            }
        }
    }

    // return safely
    Ok(new_path.to_string_lossy().to_string())
}
