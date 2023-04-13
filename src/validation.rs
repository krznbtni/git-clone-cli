use std::fs;
use std::path::Path;

use regex::Regex;

pub fn is_valid_username(username: &str) -> bool {
    // r: a raw string.
    // A raw string is just like a regular string,
    // except it does not process any escape sequences.
    // For example, "\\d" is the same expression as r"\d".
    let re = Regex::new(r"^[0-9A-Za-z_.-]+$").unwrap();
    re.is_match(username)
}

#[test]
fn t_is_valid_username() {
    assert!(is_valid_username("0valid_.-"));
    assert!(!is_valid_username(" invalid_.-/¤"));
}

pub fn is_valid_directory(path: &Path) -> bool {
    match fs::metadata(path) {
        Err(_) => false,
        Ok(res) => res.is_dir(),
    }
}

#[test]
fn t_is_valid_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new()?;
    assert!(is_valid_directory(dir.path()));

    let file = assert_fs::NamedTempFile::new("temp-file.txt")?;
    assert!(!is_valid_directory(file.path()));

    Ok(())
}
