use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/// A function to return the lines in a document as a `Vec<String>` (a lovely datatype to work with)
/// 
/// # Errors
/// - File does not exist
/// - No sudo permissions
/// - Corrupt file
/// - Binary file (potentially)
pub fn read_file_to_vec(path: &str) -> std::io::Result<Vec<String>> {
    let filestring: String = fs::read_to_string(path)?;
    let vec: Vec<String> = filestring.split('\n').map(|x| x.to_string()).collect();
    Ok(vec)
}

/// A function to read the files and folders in a given directory and returns them as a `Vec<String>`
/// 
/// # Panics
/// I'm starting to regret setting Clippy to pedantic
/// 
/// # Errors
/// - Path does not exist
/// - No sudo permissions could cause issues
pub fn get_items_to_vec(path: &PathBuf) -> std::io::Result<Vec<String>> {
    let mut vec: Vec<String> = Vec::new();
    for item in fs::read_dir(path)? {
        vec.push(item.unwrap().file_name().into_string().unwrap());
    }
    Ok(vec)
}