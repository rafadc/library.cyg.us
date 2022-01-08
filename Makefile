SHELL := /bin/bash
MDS=$(wildcard site/*/*.md)
HTMLS=$(patsubst site/%.md,output/%.html, $(MDS))

.PHONY: all

all: $(HTMLS) output/index.html css js metadata assets
	echo $(HTMLS)

output:
	mkdir output

output/index.html: site/index.md
	pandoc site/index.md -o output/index.html --template site/templates/index.html

%.html: output
	mkdir -p $(dir $@)
	pandoc $(patsubst output/%.html,site/%.md,$@) -o "$@" --template site/templates/template.html

css:
	cp site/style.css output/

js:
	cp site/index.js output/

metadata:
	cp -r site/metadata output/

assets:
	cp -r site/assets output/

watch:
	ag -l | entr sh -c 'make clean && make'

serve:
	browser-sync start --server './output' --files "site/**/*"

clean:
	rm -rf output/*

update_binaries:
	cd post_template; cargo build
