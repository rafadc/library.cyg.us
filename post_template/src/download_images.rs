use std::error::Error;
use std::fs::File;
use std::io::Write;

pub async fn download_cover(book_id: &str) -> Result<(), Box<dyn Error>> {
    let image_url = format!("https://covers.openlibrary.org/b/olid/{}-L.jpg", book_id);
    let file_name = format!("./site/assets/covers/{}.jpg", book_id);
    download(image_url, file_name).await?;

    Ok(())
}

pub async fn download_author(author_id: &str) -> Result<(), Box<dyn Error>> {
    let image_url = format!("https://covers.openlibrary.org/a/olid/{}-L.jpg", author_id);
    let file_name = format!("./site/assets/authors/{}.jpg", author_id);
    download(image_url, file_name).await?;

    Ok(())
}

async fn download(url: String, filepath: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let image = client
        .get(url)
        .send()
        .await?
        .bytes()
        .await?;

    let mut file = File::create(&filepath)
        .unwrap_or_else(|_| panic!("Cannot create file at {}", filepath.clone()));

    file.write_all(&image)
        .unwrap_or_else(|_| panic!("Cannot write file at {}", filepath.clone()));

    Ok(())
}