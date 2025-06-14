// This file contains various helper utilities that support the functionality of the agent.

use std::fs;
use std::path::Path;

/// Reads the contents of a file and returns it as a String.
/// Returns an error message if the file cannot be read.
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

/// Writes a string to a file.
/// Returns an error message if the file cannot be written.
pub fn write_string_to_file<P: AsRef<Path>>(path: P, contents: &str) -> Result<(), String> {
    fs::write(path, contents).map_err(|e| e.to_string())
}

/// Checks if a file exists at the given path.
/// Returns true if the file exists, false otherwise.
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

/// Creates a directory if it does not already exist.
/// Returns an error message if the directory cannot be created.
pub fn create_dir_if_not_exists<P: AsRef<Path>>(path: P) -> Result<(), String> {
    if !file_exists(&path) {
        fs::create_dir_all(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}