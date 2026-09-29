use crate::tui::app::App;


pub fn toggle_expansion(app: &mut App) -> () {
    // let visible_rows = build_visible_rows(&app);
    // let index_option = find_index_of_selection_in_visible_rows(&app.selection, &visible_rows);

    let selected_package_id = app.selection.package_id;
    if app.expanded_packages.contains(&selected_package_id) {
        app.expanded_packages.remove(&selected_package_id);
    } else {
        app.expanded_packages.insert(selected_package_id);
    }
}