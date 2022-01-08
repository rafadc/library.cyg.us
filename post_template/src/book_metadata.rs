use std::error::Error;
use std::fs;
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use yaml_front_matter::{Document, YamlFrontMatter};
use crate::download_images::{download_author, download_cover};
use crate::template_generator::generate_template;

#[derive(Serialize, Deserialize, Debug)]
pub struct BookMetadata {
    pub title: String,
    pub authors: Vec<String>,
    pub openlibrary_id: String,
    pub openlibrary_cover_edition_id: Option<String>,
    pub openlibrary_author_ids: Vec<String>,
}

impl BookMetadata {
    pub fn from_file(filename: String) -> BookMetadata{
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let document: Document<BookMetadata> = YamlFrontMatter::parse::<BookMetadata>(&contents).unwrap();
        document.metadata
    }

    pub fn crete_consolidated_files(&self) -> Result<(), Box<dyn Error>> {
        generate_template(self)?;
        self.update_consolidated_files()
    }

    pub fn update_consolidated_files(&self) -> Result<(), Box<dyn Error>> {
        match &self.openlibrary_cover_edition_id {
            Some(edition_id) => block_on(download_cover(edition_id))?,
            _ => ()
        }

        self.openlibrary_author_ids
            .clone()
            .into_iter()
            .for_each(|x| block_on(download_author(&x)).unwrap());

        Ok(())
    }
}