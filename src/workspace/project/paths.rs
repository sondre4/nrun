pub fn get_start_directory() -> Result<std::path::PathBuf, std::io::Error> {
    let path = std::env::current_dir();
    return path;
}
