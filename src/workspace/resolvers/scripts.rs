use crate::workspace::{parsers::config_types::PackageJson, types::Script};

pub fn resolve_scripts(package_json: &PackageJson) -> Result<Vec<Script>, String> {
    let script_map_option = &package_json.scripts;

    if script_map_option.is_none() {
        return Err("package.json schema contains no scripts".to_string());
    }

    let script_map = script_map_option.as_ref().unwrap();
    let mut scripts: Vec<Script> = Vec::new();

    let mut current_script_id: usize = 0;

    for (script_name, script_command) in script_map {
        let script: Script = Script {
            id: current_script_id,
            name: script_name.to_string(),
            command: script_command.to_string(),
        };

        scripts.push(script);
        current_script_id += 1;
    }

    scripts.sort_by(|script_a, script_b| {
        return script_a.name.cmp(&script_b.name);
    });

    return Ok(scripts);
}
