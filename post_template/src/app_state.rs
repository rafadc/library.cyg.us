use crate::book::Book;

pub struct AppState {
    pub input: String,
    pub cursor_position: u8,
    pub search_results: Vec<Book>
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            input: String::new(),
            cursor_position: 0,
            search_results: vec!(),
        }
    }
}
