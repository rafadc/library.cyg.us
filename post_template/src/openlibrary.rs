use crate::book::Book;
use crate::stateful_list::StatefulList;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct OpenLibraryDocument {
    title: String,
    author_name: Option<Vec<String>>,
    author_key: Option<Vec<String>>
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
        .map(|doc| open_library_to_book(doc))
        .collect();

    Ok(StatefulList::with_items(books))
}

fn open_library_to_book(doc: OpenLibraryDocument) -> Book {
    let author_name = match doc.author_name {
        Some(names) => names[0].clone(),
        None => "".to_string()
    };

    Book {title: doc.title, author: author_name, openlibrary_id: "".to_string(), openlibrary_author_ids: "".to_string(), synopsis: "".to_string() }
}