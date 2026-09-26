use crate::workspace::types::{PackageJson, Script};

pub fn get_scripts_from_package_json(package_json_schema: &PackageJson) -> Result<Vec<Script>, String> {
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

