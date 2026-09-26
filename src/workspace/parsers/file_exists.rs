use std::path::Path;

pub fn file_exists(path: &Path) -> Result<bool, String> {
    if path.exists() {
        return Ok(true);
    }
    return Err(format!("File '{}' does not exist", path.to_string_lossy()));
}
