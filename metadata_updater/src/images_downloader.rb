# Downloads the missing images to the output folder
class ImagesDownloader
  def self.from_metadata(books_metadata)
    books_metadata.each do |book|
      id = book['openlibrary_id']

      FileUtils.mkdir_p("../site/assets/authors/")
      FileUtils.mkdir_p("../site/assets/covers/")

      download_cover_image(id)
      download_author_images(book['openlibrary_author_ids'])
    end
  end

  def self.download_author_images(author_ids)
    author_ids.each do |id|
      cover_file = "../site/assets/authors/#{id}.jpg"
      return if File.file?(cover_file)

      cover_url = "https://covers.openlibrary.org/a/olid/#{id}-L.jpg"
      download_file(cover_url, cover_file)
    end
  end

  def self.download_cover_image(id)
    author_file = "../site/assets/covers/#{id}.jpg"
    return if File.file?(author_file)

    author_url = "https://covers.openlibrary.org/b/olid/#{id}-L.jpg"
    download_file(author_url, author_file)
  end

  def self.download_file(url, file_location)
    http_conn = Faraday.new do |builder|
      builder.adapter Faraday.default_adapter
      builder.use FaradayMiddleware::FollowRedirects, {}
    end
    response = http_conn.get url
    File.open(file_location, 'wb') { |fp| fp.write(response.body) }
  end
end