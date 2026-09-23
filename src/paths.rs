// use std::path::PathBuf;

pub fn get_caller_directory() -> Result<std::path::PathBuf, std::io::Error> {
    return std::env::current_dir();
}