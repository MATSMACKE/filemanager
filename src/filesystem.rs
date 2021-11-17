use std::fs;
use std::path::Path;
use std::ffi::OsStr;

pub fn read_file_to_vec(path: &str) -> std::io::Result<Vec<String>> {
    let filestring: String = fs::read_to_string(path)?;
    let vec: Vec<String> = filestring.split("\n").map(|x| x.to_string()).collect();
    return Ok(vec)
}

pub fn get_items_to_vec(path: &str) -> std::io::Result<Vec<String>> {
    let mut vec: Vec<String> = Vec::new();
    for item in fs::read_dir(path)? {
        if let Ok(item) = item {
            vec.push(item.file_name().into_string().unwrap());
        }
    }
    Ok(vec)
}