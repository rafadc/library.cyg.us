use crate::book::Book;
use crate::stateful_list::StatefulList;

pub fn search_books(_query: &String) -> StatefulList<Book> {
    let sample_book = Book{
        title: "Good strategy/Bad Strategy".to_string(),
        author: "Rummelt".to_string(),
        openlibrary_id: "21q12f1".to_string(),
        openlibrary_author_ids: "ffqe12e1".to_string(),
        synopsis: "A nice book".to_string()
    };

    let other_sample_book = Book{
        title: "Range".to_string(),
        author: "Epstein".to_string(),
        openlibrary_id: "21q12f1".to_string(),
        openlibrary_author_ids: "ffqe12e1".to_string(),
        synopsis: "A ranged book".to_string()
    };
    StatefulList::with_items(vec![sample_book, other_sample_book])
}