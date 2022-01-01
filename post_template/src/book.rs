use serde::Serialize;

#[derive(Serialize)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub openlibrary_id: String,
    pub openlibrary_author_ids: Vec<String>,
    pub description: String
}
