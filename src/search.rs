pub mod searching;

// pub fn search_packages()

// TODO: to implement package expansion with search:
// add field in app.tui_state for search_packages_expanded: HashSet<usize>
// this field should be initialized to all packages when typing search query (in operation handler: Operation::SearchOperation(input_request))
// this field should be cleared on search cancel
// when rendering: check if search is active
// 