use std::{collections::HashMap, error::Error, fs, path::Path};

pub fn move_file<S: ::std::hash::BuildHasher>(
    filename: &str,
    target_pattern: &str,
    values: &HashMap<String, String, S>,
    unique_val: usize,
    dry_run: bool,
) -> Result<String, Box<dyn Error>> {
    log::debug!("filename: {filename}");
    log::debug!("target_pattern: {target_pattern}");
    log::debug!("values: {values:?}");

    let mut target = target_pattern.to_string();

    // Perform substitutions on the target path
    for (key, value) in values {
        let fixed_value = value.clone().trim().to_string();
        log::debug!("key: {key}, fixed_value: {fixed_value}");

        // Do the actual target path replacement
        let nt = target.replace(key, &fixed_value);
        target = nt;
        log::debug!("target: {target}");
    }

    log::debug!("final target: {target}");

    // Check if the target exists and if it is a directory
    let target_path = Path::new(&target);
    if !target_path.exists() {
        // Make the target directory
        if dry_run {
            log::debug!("mkdir -p {target}");
        } else {
            let md_res = std::fs::create_dir_all(&target_path);
            match md_res {
                Ok(()) => log::debug!("mkdir -p {target}"),
                Err(err) => {
                    return Err(format!("Unable to create {target}. Error message: {err}").into());
                }
            }
        }
    }

    // Verify that the target is a directory
    if !target_path.is_dir() {
        return Err(format!("Target path {target} is not a directory.").into());
    }

    // Move the file to the target direcctory
    let target_filename = Path::new(&filename).file_name().unwrap_or_default();
    log::debug!("target_filename = {target_filename:?}");

    let mut target_file = target_path.join(target_filename);
    log::debug!("target_file = {target_file:?}");

    // Check if a file with the new filename already exists - make the filename unique if it does.
    if Path::new(&target_file).exists() {
        log::warn!(
            "{} already exists. Appending unique identifier.",
            target_file.to_string_lossy()
        );
        let target_filename = format!(
            "{} ({unique_val})",
            target_filename.to_string_lossy().to_string()
        );
        target_file = target_path.join(target_filename);
    }

    // Perform the actual move
    if dry_run {
        log::debug!("dr: mv {filename} {}", target_file.to_string_lossy());
    } else {
        log::debug!("mv {filename} {}", target_file.to_string_lossy());
        let _res =
            fs::rename(filename, &target_file).map_err(|e| format!("Unable to move file: {e}",));
    }

    Ok(target_file.to_string_lossy().to_string())
}
