class DBCreate
  def self.from_metadata(metadatas)
    db = SQLite3::Database.new "../output/metadata/metadata.db"

    rows = db.execute <<-SQL
  create table books (
    openlibrary_id varchar(30),
    title varchar(30),
    author varchar(30),
    cover varchar(255)
  );
    SQL
  end
end