use std::path::{Path, PathBuf};

use crate::{config::{PNPM_CONFIG, Package, PackageJsonPartialSchema, Project, Script, Workspace}, parse_file::{parse_package_json, parse_pnpm_lockfile_yaml, parse_pnpm_workspace_yaml}, paths};

fn get_package_json_schema_from_path(path: &Path) -> Result<PackageJsonPartialSchema, String> {
    let parsed_package_json = parse_package_json(path)?;
    return Ok(parsed_package_json);
}

pub fn get_package_json_schema(project: &Project) -> Result<PackageJsonPartialSchema, String> {
    let package_json_path = project.root_path.join(project.config.manifest_name);
    return get_package_json_schema_from_path(&package_json_path);
}


fn get_scripts_from_package_json_schema(package_json_schema: &PackageJsonPartialSchema) -> Result<Vec<Script>, String> {
    let script_map_option = &package_json_schema.scripts;

    if script_map_option.is_none() {
        return Err("package.json schema contains no scripts".to_string());
    }

    let script_map = script_map_option.as_ref().unwrap();
    let mut scripts: Vec<Script> = Vec::new();

    for (script_name, script_command) in script_map {
        let script: Script = Script {
            name: script_name.to_string(),
            command: script_command.to_string(),
        };

        scripts.push(script);
    }

    return Ok(scripts);
}


pub fn get_project_info(root_path: &Path) -> Option<Project> {
    let called_from_path: std::borrow::Cow<'_, str> = root_path.to_string_lossy();
    println!("current path: {called_from_path}");

    let found_project = paths::find_project_root(root_path.to_path_buf());
    return found_project;
}

pub fn get_pnpm_workspace(project: &Project, root_package_json_schema: &PackageJsonPartialSchema) -> Result<Workspace, String> {

    // pnpm workspace file
    let workspace_path = project.root_path.join(project.config.workspace_config_name);
    let lockfile_path = project.root_path.join(project.config.lockfile_name);

    let printable_workspace_path = workspace_path.to_string_lossy();
    let printable_lockfile_path = workspace_path.to_string_lossy();

    let parsed_workspace_yaml = parse_pnpm_workspace_yaml(&workspace_path);

    if parsed_workspace_yaml.is_none() {
        println!("Error whilst parsing workspace file '{printable_workspace_path}'");
        // return;
    }


    // pnpm lockfile
    let parsed_lockfiles_yaml = parse_pnpm_lockfile_yaml(&lockfile_path);
    let second_schema = &parsed_lockfiles_yaml[1];

    // let keys = second_schema.importers.keys();
    // println!("{keys:#?}");

    let mut found_pnpm_packages: Vec<Package> = Vec::new();

    for package_directory in second_schema.importers.keys() {

        println!("\npackage: {package_directory}");

        // if package_directory == "." {
        //     break;
        // }

        // println!("package: {package_directory}");

        let package_path = project.root_path.join(PathBuf::from(package_directory));
        let package_json_path = package_path.join(PNPM_CONFIG.manifest_name);
        let package_json_schema  = get_package_json_schema_from_path(&package_json_path)?;
        let scripts: Vec<Script> = get_scripts_from_package_json_schema(&package_json_schema)?;

        let last = package_path.file_name().unwrap().to_string_lossy();
        let second_last = package_path.parent().unwrap().file_name().unwrap().to_string_lossy();
        let name = format!("{second_last}/{last}");

        let package: Package = Package {
            name: name,
            path: package_json_path,
            scripts: scripts,
        };

        found_pnpm_packages.push(package);
    }


    let workspace: Workspace = Workspace {
        root_path: project.root_path.clone(),
        package_manager: project.package_manager,
        packages: found_pnpm_packages,
    };

    return Ok(workspace);

}
