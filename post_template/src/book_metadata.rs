use futures::executor::block_on;
use serde::{Deserialize, Serialize};
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
    pub fn crete_metadata(&self) -> Result<(), Box<dyn std::error::Error>> {
        generate_template(self)?;

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