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
refhelper 0.1.1
A CLI tool to manage paper references

USAGE:
    refhelper [SUBCOMMAND]

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
refhelper 0.1.1

USAGE:
    Type command in an interactive shell

SUBCOMMANDS:
    open         Open a library
    list         List entries of current library
    search       Search entries with some pattern
    add          Add an entry to current library using DOI or arXiv id
    add_batch    Add a batch of entries to current library (from a txt file)
    load         Load a batch of entries to current library (from a bibtex file)
    del          Delete an entry in current library
    link         Create link from entry to a pdf file
    view         View chosen pdf file in pdfviewer
    gen          Generate bibtex file of one entry or entire library
    quit         Quit from interactive CLI
    help         Prints this message or the help of the given subcommand(s)

```

example:

```
Welcome to refhelper 0.1.1!
>> open test.json
>> ls
Current library: test.json
+----+------+-------+-----+------+
| id | name | title | doi | path |
+================================+
+----+------+-------+-----+------+
>> add swan 10.1145/2486001.2486012
>> add test1 1904.12901
>> ls
Current library: test.json
+----+-------+----------------------------+-------------------------+------+
| id | name  | title                      | doi                     | path |
+==========================================================================+
| 0  | swan  | Achieving high utilization | 10.1145/2486001.2486012 | n    |
|    |       | with software-driven WAN   |                         |      |
|----+-------+----------------------------+-------------------------+------|
| 1  | test1 | Challenges of real-world   | 1904.12901              | n    |
|    |       | reinforcement learning     |                         |      |
+----+-------+----------------------------+-------------------------+------+
>> add_batch doi.txt
███████████████████████████████████████████████████████████████████████████ 7/7
add 'xxx' error: Failed to get bibtex from DOI/arXiv
Add 6 entries from file
>> search SIGCOMM
Current library: test.json
+----+------+---------------------------------+-------------------------+------+
| id | name | title                           | doi                     | path |
+==============================================================================+
| 0  | swan | Achieving high utilization with | 10.1145/2486001.2486012 | n    |
|    |      | software-driven WAN             |                         |      |
|----+------+---------------------------------+-------------------------+------|
| 4  | defo | A declarative and expressive    | 10.1145/2829988.2787495 | y    |
|    |      | approach to control forwarding  |                         |      |
|    |      | paths in carrier-grade networks |                         |      |
+----+------+---------------------------------+-------------------------+------+

```

## TODO

- [x] rustyline history and file completion
- [x] add a batch of entries from file
- [x] run doi2bib concurrently
- [x] fuzzy search
- [x] open pdf cross-platform support (linux, windows)
- [x] add support for arXiv id
- [ ] more tests
- [ ] download and link pdf file automatically when adding entries
- [ ] proxy argument
