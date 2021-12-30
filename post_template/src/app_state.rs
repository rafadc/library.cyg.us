use crate::book::Book;
use crate::stateful_list::StatefulList;

pub struct AppState {
    pub input: String,
    pub search_results: StatefulList<Book>
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            input: String::new(),
            search_results: StatefulList::with_items(vec![]),
        }
    }
}
