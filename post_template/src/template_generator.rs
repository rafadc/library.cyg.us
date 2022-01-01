use std::fs;
use tinytemplate::TinyTemplate;
use crate::book::Book;
use serde::Serialize;

static TEMPLATE : &'static str = "---\n\
title: {book.title}\n\
authors: {printable_authors}\n\
openlibrary_id: {book.openlibrary_id}\n\
openlibrary_author_ids: {printable_author_ids}\n\
---\
";

#[derive(Serialize)]
pub struct TemplateContext<'a> {
    pub book: &'a Book,
    pub printable_authors: String,
    pub printable_author_ids: String,
}


pub fn generate_template(book: &Book) -> Result<(), std::io::Error> {
    let mut tt = TinyTemplate::new();
    tt.add_template("book", TEMPLATE).unwrap();

    let context = TemplateContext {
        book,
        printable_authors: "".to_string(),
        printable_author_ids: "".to_string()
    };

    let rendered = tt.render("book", &context).unwrap();
    fs::write("foo.txt", rendered).unwrap();

    Ok(())
}