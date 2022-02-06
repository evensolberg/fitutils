use std::{collections::HashMap, error::Error, path::Path};

/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    std::path::Path::new(&filename)
        .extension()
        .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Renames the target file based on the provided patterntar
pub fn rename_file(
    filename: &str,
    pattern: &str,
    values: &HashMap<String, String>,
    dry_run: bool,
) -> Result<String, Box<dyn Error>> {
    let mut new_filename = pattern.to_string();

    for (key, value) in values {
        // Make sure to pad disc and track numbers with leading zeros
        let mut fixed_value = value.clone();
        fixed_value = fixed_value.trim().to_string();

        // Do the actual filename replacement
        new_filename = new_filename.replace(key, &fixed_value);

        // Fix a few things we know will give us trouble later.
        new_filename = new_filename.replace('/', "-");
        // new_filename = new_filename.replace(':', " -");
        // new_filename = new_filename.replace('.', "");

        // Remove leading or trailing spaces
        new_filename = new_filename.trim().to_string();
    }

    // Get the path before the filename (eg. "music/01.flac" returns "music/")
    let parent = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Create the new filename
    let new_path = parent.join(Path::new(&new_filename).with_extension(get_extension(filename)));
    log::debug!("new_path = {:?}", new_path);

    if dry_run {
        log::debug!("dr: {} --> {}", filename, new_path.display());
    } else {
        // Get parent dir
        let rn_res = std::fs::rename(&filename, &new_path);
        match rn_res {
            Ok(_) => log::debug!("{} --> {}", filename, new_path.to_string_lossy()),
            Err(err) => {
                return Err(format!(
                    "Unable to rename {} to {}. Error message: {}",
                    filename,
                    new_path.to_string_lossy(),
                    err
                )
                .into());
            }
        }
    }

    // return safely
    Ok(new_path.to_string_lossy().to_string())
}
