# frozen_string_literal: true

require_relative './metadata_updater'

book_files = Dir.glob("../site/books/**/*.md")

books_metadata = book_files.map { |file| Metadata.from_file(file) }

FileUtils.mkdir_p('../output/metadata')
File.delete('../output/metadata/metadata.db') if File.file?('../output/metadata/metadata.db')
# DBCreate.from_metadata(books_metadata)

ImagesDownloader.from_metadata(books_metadata)
