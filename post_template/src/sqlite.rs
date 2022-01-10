use std::error::Error;
use rusqlite::{Connection, params};
use slug::slugify;
use crate::BookMetadata;

pub async fn update_db(book: &BookMetadata) -> Result<(), Box<dyn Error>>{
   let conn = Connection::open("./site/metadata/books.sqlite")?;

    conn.execute(
        "create table if not exists books (
             openlibrary_id text primary key,
             title text not null,
             slug text not null,
             finished_at text,
             last_updated_at text not null
         )",
        [],
    )?;

    conn.execute("delete from books where openlibrary_id = ?", &[&book.openlibrary_id])?;

    let slug = slugify(&book.title);

    conn.execute(
        "insert into books(openlibrary_id, title, slug, finished_at, last_updated_at) values (?, ?, ?, ?, ?)",
        params![&book.openlibrary_id, &book.title, slug, &book.finished_at, &book.last_updated_at]
    )?;

    Ok(())
}