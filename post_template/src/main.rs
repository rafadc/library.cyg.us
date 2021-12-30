use std::error::Error;
use crate::ui::open_ui;

mod app_state;
mod book;
mod openlibrary;
mod stateful_list;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    open_ui()
}
