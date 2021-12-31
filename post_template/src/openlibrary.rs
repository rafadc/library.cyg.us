use crate::book::Book;
use crate::stateful_list::StatefulList;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct OpenLibraryDocument {
    title: String,
    author_name: Vec<String>,
    author_key: Vec<String>
}

#[derive(Deserialize, Debug)]
struct OpenLibraryResponse {
    docs: Vec<OpenLibraryDocument>
}

pub async fn search_books(query: &String) -> Result<StatefulList<Book>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let openlibrary_response = client.get("http://openlibrary.org/search.json")
        .query(&[("q", query)])
        .send()
        .await?
        .json::<OpenLibraryResponse>()
        .await?;

    let books = openlibrary_response
        .docs
        .into_iter()
        .map(|doc| Book {title: doc.title, author: doc.author_name[0].clone(), openlibrary_id: "".to_string(), openlibrary_author_ids: "".to_string(), synopsis: "".to_string() })
        .collect();

    Ok(StatefulList::with_items(books))
}