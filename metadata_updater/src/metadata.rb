# frozen_string_literal: true

# Class to extract the metadata from a Markdown file
class Metadata
  EXTRACT_METADATA_FROM_MD = /---\n(.*)\n---\n/m

  def initialize(values = {})
    @values = values
  end

  def self.from_file(filename)
    metadata = from(IO.read(filename))
    metadata['input_file'] = filename
    metadata
  end

  def self.from(input)
    m = input.match(EXTRACT_METADATA_FROM_MD)

    raise ArgumentError, "No metadata found in #{input}" unless m

    values = YAML.safe_load("---\n#{m[1]}").to_h

    Metadata.new(values)
  end

  def [](key)
    @values[key]
  end

  def []=(key, value)
    @values[key] = value
  end
end