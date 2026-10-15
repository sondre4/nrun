use std::path::{Path, PathBuf};

use tempfile::TempDir;

// pub fn setup_temp_directory() -> () {
//     tempdir
// }

/// A temporary copy of a fixture project.
///
/// The directory is deleted automatically when this value is dropped.
#[derive(Debug)]
pub struct TempProject {
    _temp_dir: TempDir,
    path: PathBuf,
}

impl TempProject {
    pub fn path(&self) -> &Path {
        return &self.path;
    }
}

/// Copies a fixture project into a temporary directory.
///
/// The returned project path can be used as the working directory
pub fn create_temp_directory(fixture_path: &Path) -> std::io::Result<TempProject> {
    let temp_dir = tempfile::tempdir()?;
    let project_path = temp_dir.path().join("test-project");

    // println!("{fixture_path:#?}");

    let _ = fs_extra::dir::copy(fixture_path, &project_path, &fs_extra::dir::CopyOptions::new().content_only(true));

    return Ok(TempProject {
        _temp_dir: temp_dir,
        path: project_path,
    });
}
