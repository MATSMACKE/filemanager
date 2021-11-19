use std::fs;
use std::path::Path;

/// A function to return the lines in a document as a `Vec<String>` (a lovely datatype to work with)
///
/// # Errors
/// - File does not exist
/// - No sudo permissions
/// - Corrupt file
/// - Binary file (potentially)
pub fn read_file_to_vec(path: &Path) -> std::io::Result<Vec<String>> {
    let filestring: String = fs::read_to_string(path)?;
    let vec: Vec<String> = filestring
        .split('\n')
        .map(std::string::ToString::to_string)
        .collect();
    Ok(vec)
}

/// A function to read the files and folders in a given directory and returns them as a `Vec<String>`
///
/// # Errors
/// - Path does not exist
/// - No sudo permissions could cause issues
pub fn get_items_to_vec(path: &Path) -> std::io::Result<(Vec<String>, Vec<String>)> {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for item in fs::read_dir(path)? {
        let item = item.unwrap();
        if fs::metadata(item.path()).unwrap().is_dir() {
            dirs.push(item.file_name().into_string().unwrap());
        }
        if fs::metadata(item.path()).unwrap().is_file() {
            files.push(item.file_name().into_string().unwrap());
        }
    }

    dirs.sort_by_key(|s| s.to_lowercase());
    files.sort_by_key(|s| s.to_lowercase());

    Ok((dirs, files))
}
