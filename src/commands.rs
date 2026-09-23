use std::path::PathBuf;

use crate::paths::get_caller_directory;

pub fn check(path: PathBuf, verbose: bool) -> () {

}

pub fn find_package_json() -> () {
    let called_from: PathBuf = get_caller_directory().unwrap();
    let called_from_path: String = called_from.into_string().unwrap();
    println!("current path: {called_from_path}");

}
