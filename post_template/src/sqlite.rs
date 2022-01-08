use std::error::Error;
use rusqlite::Connection;

pub fn migrate_db() -> Result<(), Box<dyn Error>>{
   let conn = Connection::open("./site/metadata/books.sqlite")?;

    conn.execute(
        "create table if not exists books (
             openlibrary_id text primary key,
             title text not null,
             finished_at text,
             last_updated_at text not null
         )",
        [],
    )?;

    Ok(())
}