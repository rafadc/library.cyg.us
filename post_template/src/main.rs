use std::error::Error;
use crate::ui::open_ui;
use clap::Parser;
use crate::book_metadata::BookMetadata;

mod book_metadata;
mod download_images;
mod openlibrary;
mod template_generator;
mod ui;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// An existing book so we download again its images
    #[clap()]
    md_file_to_consolidate: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.md_file_to_consolidate {
        Some(filename) => BookMetadata::from_file(filename).update_consolidated_files(),
        _ => open_ui()
    }
}
