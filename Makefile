SHELL := /bin/bash
MDS=$(wildcard site/*.md) $(wildcard site/*/*.md)
HTMLS=$(patsubst site/%.md,output/%.html, $(MDS))

.PHONY: all

all: $(HTMLS) css assets
	echo $(HTMLS)

output:
	mkdir output

%.html: output
	mkdir -p $(dir $@)
	pandoc $(patsubst output/%.html,site/%.md,$@) -o "$@" --template site/templates/template.html

css:
	cp site/style.css output/

assets:
	cp -r site/assets output/

watch:
	ag -l | entr sh -c 'make'

serve:
	browser-sync start --server './output' --files "site/**/*"

clean:
	rm -rf output

update_binaries:
	cd post_template; cargo build

