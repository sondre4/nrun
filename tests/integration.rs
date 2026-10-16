#[path = "integration/setup_temp_directory.rs"]
mod setup_temp_directory;

// Monorepos
#[path = "integration/can_call_monorepo_script.rs"]
mod can_call_monorepo_script;


#[path = "integration/discovers_monorepo_info.rs"]
mod discovers_monorepo_info;

#[path = "integration/discovers_monorepo_workspace.rs"]
mod discovers_monorepo_workspace;

// Normal projects
#[path = "integration/can_call_project_script.rs"]
mod can_call_project_script;

#[path = "integration/discovers_project_info.rs"]
mod discovers_project_info;

#[path = "integration/discovers_project_workspace.rs"]
mod discovers_project_workspace;
