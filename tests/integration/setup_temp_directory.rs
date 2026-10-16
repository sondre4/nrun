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
    pub fn root_path(&self) -> &Path {
        return &self.path;
    }
}

/// Copies a fixture project into a temporary directory.
///
/// The returned project path can be used as the working directory
fn create_temp_directory(fixture_path: &Path, project_directory_name: &Path) -> std::io::Result<TempProject> {
    let temp_dir = tempfile::tempdir()?;
    let project_path = temp_dir.path().join(project_directory_name);

    // println!("{fixture_path:#?}");

    let _ = fs_extra::dir::copy(fixture_path, &project_path, &fs_extra::dir::CopyOptions::new().content_only(true));

    return Ok(TempProject {
        _temp_dir: temp_dir,
        path: project_path,
    });
}

pub fn create_temp_project_from_base(base: &Path, project_directory_name: &Path) -> TempProject {
    let fixture_base_path = base.join("tests").join("fixtures").join(project_directory_name);

    let temp_fixture_path_result = create_temp_directory(&fixture_base_path, project_directory_name);
    assert!(temp_fixture_path_result.is_ok());
    let temp_fixture_project = temp_fixture_path_result.unwrap();
    let temp_fixture_monorepo_root = temp_fixture_project.root_path();

    // Check that the temporary directory was created properly
    assert!(temp_fixture_monorepo_root.join("package.json").exists(), "Could not find package.json at '{}'", temp_fixture_monorepo_root.join("package.json").to_string_lossy());
    return temp_fixture_project;
}

pub fn get_cargo_manifest_dir_path() -> PathBuf {
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"));
}
