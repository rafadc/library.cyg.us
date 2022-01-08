use std::error::Error;
use std::fs;
use yaml_front_matter::{Document, YamlFrontMatter};
use crate::book_metadata::BookMetadata;

pub async fn update_file_metadata(filename: String) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let document: Document<BookMetadata> = YamlFrontMatter::parse::<BookMetadata>(&contents).unwrap();
    println!("{:?}", document.metadata);

    Ok(())
}