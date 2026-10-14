// Monorepos
#[path = "integration/discovers_monorepo_package_manager.rs"]
mod discovers_monorepo_package_manager;

#[path = "integration/discovers_monorepo_workspace.rs"]
mod discovers_monorepo_workspace;

// Normal projects
#[path = "integration/discovers_project_package_manager.rs"]
mod discovers_project_package_manager;

#[path = "integration/discovers_project_workspace.rs"]
mod discovers_project_workspace;
