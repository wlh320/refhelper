# refhelper

A CLI tool to manage paper references.

It is a toy project for me to learn rust.

work in process.

## Build

just like other rust project

```
cargo build --release
```

## Usage

`Entry` - A bibtex entry, can be auto generated from DOI

`Library` - A collection of entries, can be exported to `.bibtex` file

```
refhelper 0.1.0
A CLI tool to manage paper references

USAGE:
    refhelper <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cli     Start interactive CLI
    gen     Generate bibtex file from library
    help    Prints this message or the help of the given subcommand(s)
```

interactive CLI:

```
refhelper 0.1.0

USAGE:
    Type command in an interactive shell

SUBCOMMANDS:
    open         Open a library
    list         List entries of current library
    add          Add an entry to current library
    add_batch    Add a batch of entries to current library (from a txt file)
    load         Load a batch of entries to current library (from a bibtex file)
    del          Delete an entry in current library
    link         Create link from entry to a pdf file
    view         View chosen pdf file in pdfviewer
    gen          Generate bibtex file of current library
    quit         Quit from interactive CLI
    help         Prints this message or the help of the given subcommand(s)
```

## TODO

- [x] rustyline history and file completion
- [x] add a batch of entries from file
- [x] run doi2bib concurrently
- [ ] fuzzy search
- [ ] open pdf cross-platform support
