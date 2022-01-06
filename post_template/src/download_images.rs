use std::fs::File;
use std::io::Write;

pub async fn download_cover(book_id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let image_url = format!("https://covers.openlibrary.org/b/olid/{}-L.jpg", book_id);
    let file_name = format!("./site/assets/covers/{}.jpg", book_id);
    download(image_url, file_name).await?;

    Ok(())
}

pub async fn download_author(author_id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let image_url = format!("https://covers.openlibrary.org/a/olid/{}-L.jpg", author_id);
    let file_name = format!("./site/assets/authors/{}.jpg", author_id);
    download(image_url, file_name).await?;

    Ok(())
}

async fn download(url: String, filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let image = client
        .get(url)
        .send()
        .await?
        .bytes()
        .await?;

    let mut file = File::create(&filepath)
        .expect(format!("Cannot create file at {}", filepath.clone()).as_str());

    file.write_all(&image)
        .expect(format!("Cannot write file at {}", filepath.clone()).as_str());

    Ok(())
}