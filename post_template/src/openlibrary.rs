use crate::book::Book;

pub fn search_books(query: &String) -> Vec<Book> {
    let sample_book = Book{
        title: "Good strategy/Bad Strategy".to_string(),
        author: "Rummelt".to_string(),
        openlibrary_id: "21q12f1".to_string(),
        openlibrary_author_ids: "ffqe12e1".to_string(),
        synopsis: "A nice book".to_string()
    };
    vec![sample_book]
}