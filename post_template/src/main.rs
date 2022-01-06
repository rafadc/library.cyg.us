use std::error::Error;
use crate::ui::open_ui;

mod app_state;
mod book;
mod download_images;
mod openlibrary;
mod stateful_list;
mod template_generator;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    open_ui()
}
