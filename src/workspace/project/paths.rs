use std::path::PathBuf;

pub fn get_start_directory() -> Result<PathBuf, String> {
    let path = std::env::current_dir();
    match path {
        Ok(result) => {
            return Ok(result);
        }
        Err(error) => {
            return Err(format!("Error whilst getting start directory: {error}"));
        }
    }
}
