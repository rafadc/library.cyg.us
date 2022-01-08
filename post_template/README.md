# Creator of templates for new books

A simple CLI to create a new template to retrieve metadata from a book. It downloads images for the cover and the author.

## Usage

```shell
post_template 0.1.0

USAGE:
    post_template [MD_FILE_TO_CONSOLIDATE]

ARGS:
    <MD_FILE_TO_CONSOLIDATE>    An existing book so we download again its images

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

We can start the command with no parameters and a UI to create a new template will be shown.

We can also choose an already existing markdown file so we can reprocess again all metadata we need for the site.

## Roadmap

* [ ] Exit with CTRL-C
* [ ] Download images for a book in parallel
* [ ] Show the synopsis of a book
* [ ] Show a "searching" dialog when blocked by search or even better.
* [ ] Async search
* [ ] Parameter for searching in another language
* [ ] Parameter option to update images
* [ ] Parameter option to update metadata on an existing md
* [ ] Warn not to overwrite file
* [ ] Tags on books
