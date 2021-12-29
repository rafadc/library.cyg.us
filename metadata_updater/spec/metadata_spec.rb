# frozen_string_literal: true

require_relative '../src/metadata_updater'

RSpec.describe 'Metadata Reader' do
  it 'returns error for a string with no metadata' do
    expect { Metadata.from('') }.to raise_error(ArgumentError)
  end

  it 'finds a single argument' do
    metadata = Metadata.from(''"
---
sample: test
---
"'')
    expect(metadata['sample']).to eq('test')
  end

  it 'finds multiple arguments' do
    metadata = Metadata.from(''"
---
sample: test
thing:  expected
---
"'')
    expect(metadata['thing']).to eq('expected')
  end

  it 'sets metadata correctly' do
    metadata = Metadata.new
    metadata['sample'] = 'thing'

    expect(metadata['sample']).to eq('thing')
  end
end
