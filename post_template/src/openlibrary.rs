use chrono::Local;
use crate::book_metadata::BookMetadata;
use crate::ui::stateful_list::StatefulList;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct OpenLibraryDocument {
    key: String,
    title: String,
    cover_edition_key: Option<String>,
    author_name: Option<Vec<String>>,
    author_key: Option<Vec<String>>
}

#[derive(Deserialize, Debug)]
struct OpenLibraryResponse {
    docs: Vec<OpenLibraryDocument>
}

pub async fn search_books(title: &str) -> Result<StatefulList<BookMetadata>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let query = format!("title:{},language:eng", title);
    let query_params = [
        ("q", &query),
        ("fields", &String::from("key,title,author_name,author_key,cover_edition_key")),
        ("limit", &String::from("40"))
    ];

    let openlibrary_response = client.get("http://openlibrary.org/search.json")
        .query(&query_params)
        .send()
        .await?
        .json::<OpenLibraryResponse>()
        .await?;

    let books = openlibrary_response
        .docs
        .into_iter()
        .map(openlibrary_to_book)
        .collect();

    Ok(StatefulList::with_items(books))
}

fn openlibrary_to_book(doc: OpenLibraryDocument) -> BookMetadata {
    let openlibrary_id = doc.key.replace("/works/", "");

    let author_names = match doc.author_name {
        Some(names) => names,
        None => vec![]
    };

    let author_ids =  match doc.author_key {
        Some(keys) => keys,
        None => vec![]
    };

    BookMetadata {
        title: doc.title,
        authors: author_names,
        openlibrary_id,
        openlibrary_cover_edition_id: doc.cover_edition_key,
        openlibrary_author_ids: author_ids,
        finished_at: None,
        last_updated_at: Local::today().naive_local()
    }
}