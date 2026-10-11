//
//
// Cleaner function should be implemented
//
//

// use std::path::{Path, PathBuf};

// use crate::workspace::{config::{PNPM_CONFIG, Project}, parsers::{package_json::parse_package_json, pnpm_lockfile::parse_pnpm_lockfile_yaml, pnpm_workspace::parse_pnpm_workspace}, types::{Package, PackageJson, Script, Workspace}};

// pub fn get_pnpm_workspace(project: &Project, root_package_json_schema: &PackageJson) -> Result<Workspace, String> {

//     // pnpm workspace file
//     let workspace_path = project.root_path.join(project.config.workspace_config_name);
//     let lockfile_path = project.root_path.join(project.config.lockfile_name);

//     let printable_workspace_path = workspace_path.to_string_lossy();
//     let printable_lockfile_path = workspace_path.to_string_lossy();

//     let parsed_workspace_yaml = parse_pnpm_workspace(&workspace_path);

//     if parsed_workspace_yaml.is_none() {
//         println!("Error whilst parsing workspace file '{printable_workspace_path}'");
//         // return;
//     }

//     // pnpm lockfile
//     let parsed_lockfiles_yaml = parse_pnpm_lockfile_yaml(&lockfile_path);
//     let second_schema = &parsed_lockfiles_yaml[1];

//     // let keys = second_schema.importers.keys();
//     // println!("{keys:#?}");

//     let mut found_pnpm_packages: Vec<Package> = Vec::new();

//     for package_directory in second_schema.importers.keys() {

//         println!("\npackage: {package_directory}");

//         // if package_directory == "." {
//         //     break;
//         // }

//         // println!("package: {package_directory}");

//         let package_path = project.root_path.join(PathBuf::from(package_directory));
//         let package_json_path = package_path.join(PNPM_CONFIG.manifest_name);
//         let package_json_schema  = get_package_json_schema_from_path(&package_json_path)?;
//         let scripts: Vec<Script> = get_scripts_from_package_json_schema(&package_json_schema)?;

//         let last = package_path.file_name().unwrap().to_string_lossy();
//         let second_last = package_path.parent().unwrap().file_name().unwrap().to_string_lossy();
//         let name = format!("{second_last}/{last}");

//         let package: Package = Package {
//             name: name,
//             path: package_json_path,
//             scripts: scripts,
//         };

//         found_pnpm_packages.push(package);
//     }

//     let workspace: Workspace = Workspace {
//         root_path: project.root_path.clone(),
//         package_manager: project.package_manager,
//         packages: found_pnpm_packages,
//     };

//     return Ok(workspace);

// }
