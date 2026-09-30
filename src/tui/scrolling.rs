pub const INITIAL_TOP_ROW_INDEX: usize = 0;

pub struct TuiScrollState {
    pub top_row_index: usize,
    pub available_space: usize
}

impl TuiScrollState {
    fn new() -> TuiScrollState {
        return TuiScrollState {
            top_row_index: 0,
            available_space: 0,
        };
    }
}
