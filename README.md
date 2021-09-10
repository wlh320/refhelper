# refhelper

A CLI tool to manage paper references.

It is a toy project for me to learn rust.

work in process.

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
    open    Open a library
    list    List entries of current library
    add     Add an Entry to current library
    del     Delete an Entry in current library
    link    Create Link from entry and pdf file
    view    View chosen pdf file in pdfviewer
    gen     Generate bibtex file of current library
    quit    Quit from interactive CLI
    help    Prints this message or the help of the given subcommand(s)
```
