use std::error::Error;
use crate::ui::open_ui;
use crate::update_file_metadata::update_file_metadata;
use clap::Parser;

mod book_metadata;
mod download_images;
mod openlibrary;
mod template_generator;
mod ui;
mod update_file_metadata;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the markdown file to update
    #[clap(short, long)]
    update_file: Option<String>,

    /// Refresh all metadata
    #[clap(long)]
    refresh_all: bool,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.update_file {
        Some(filename) => update_file_metadata(filename).await,
        _ => open_ui()
    }
}
