use crate::tui::app::App;

pub fn toggle_expansion(app: &mut App) -> () {
    let selected_package_id_wrapped = app.app_state.selection.row.get_package_id();

    if selected_package_id_wrapped.is_none() {
        return;
    }

    let selected_package_id = selected_package_id_wrapped.unwrap();

    if app.app_state.expanded_packages.contains(&selected_package_id) {
        app.app_state.expanded_packages.remove(&selected_package_id);
    } else {
        app.app_state.expanded_packages.insert(selected_package_id);
    }
}

// not implemented yet
pub fn scripts_not_visible_after_expanding() -> bool {
    return false;
}

pub fn new_scroll_offset_after_expanding() -> usize {
    return 0;
}
