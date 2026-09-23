use std::{fs, path::Path, thread::panicking};

use crate::config::PackageJsonPartialSchema;

pub fn parse_package_json(path: &Path) -> Option<PackageJsonPartialSchema> {
    let debug_out_path = path.to_string_lossy();

    println!("trying to parse {debug_out_path}...");

    let json_file = fs::read_to_string(path).unwrap();
    let parsedJsonResult: Result<PackageJsonPartialSchema, serde_json::Error> = serde_json::from_str(&json_file);

    if parsedJsonResult.is_err() {
        return None;
    }

    let parsedJson = parsedJsonResult.unwrap();
    // return parsedJsonResult;
    return Some(parsedJson);
}
