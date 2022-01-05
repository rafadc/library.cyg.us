use std::fs;
use tinytemplate::TinyTemplate;
use crate::book::Book;
use serde::Serialize;

static TEMPLATE : &'static str = "---\n\
title: {book.title}\n\
authors: [{book.authors | as_comma_separated_list}]\n\
openlibrary_id: {book.openlibrary_id}\n\
openlibrary_author_ids: [{book.openlibrary_author_ids | as_comma_separated_list}]\n\
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
    tt.add_formatter("as_comma_separated_list", as_comma_separated_list);
    tt.add_template("book", TEMPLATE).unwrap();

    let context = TemplateContext {
        book,
        printable_authors: book.authors.join(","),
        printable_author_ids: book.openlibrary_author_ids.join(",")
    };

    let rendered = tt.render("book", &context).unwrap();
    fs::write("foo.txt", rendered).unwrap();

    Ok(())
}

fn as_comma_separated_list(value: &serde_json::value::Value, output: &mut String) -> tinytemplate::error::Result<()> {
    let values_as_strings: Vec<String> = value
        .as_array()
        .unwrap()
        .into_iter()
        .map(|x| format!("\"{}\"",x.as_str().unwrap().to_string()))
        .collect();
    output.push_str(values_as_strings.join(",").as_str());
    Ok(())
}