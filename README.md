# fmtna

A CLI app to format filenames according to a chosen naming convention.

This is for people that want to make their filenames as consistent as possible.

Supported naming conventions are:

- snake_case
- kebab-case
- camelCase
- PascalCase
- lowercase
- UPPERCASE

It should be cross-platform, but has only been tested on Linux (more specifically Linux Mint 21.2) and Windows 11.

## Contents

- [Installation](#installation)
- [Usage](#usage)
- [TODO](#todo)

## Installation

### Download the executable from the Release page

#### Prerequisites

- None

See https://github.com/yanns1/fmtna/releases.

### Download fmtna from crates.io

#### Prerequisites

- You need Rust installed (more specifically cargo).

Run `cargo install fmtna` in your terminal.

### Build from source

#### Prerequisites

- You need Rust installed (more specifically cargo).
- You need Git installed.

Clone this repo and run `./install.sh` from the root of the repo (you might need to give the script executable permission).
`install.sh` builds the project using cargo then make a symlink at `~/.local/bin/fmtna` targeting the executable produced.

## Usage

Everything is explained in `fmtna --help`.

```
Format filenames according to a chosen naming convention.

For each file/path (of any kind) given as argument, change the filename
(i.e. the base of the path) according to the naming convention selected.

WARNING! This program is dangerous.
Changing filenames is error prone and may cause undesired overwrites
or consequences (some files are expected to have the name they have
and not something else!).
fmtna's solves these problems by:
    1. Asking you what to do when conflicts happen (the program
       wants to change a path to an already existing path).
    2. Backing up the filename changes and allowing you to revert
       the changes partially or completely.
    3. Giving you ways to exclude some filenames from formatting.
Still, fmtna can't stop you from shooting yourself in the foot.
It can go as far as corrupting your system.

Usage: fmtna [OPTIONS] [FILES]...
       fmtna <COMMAND>

Commands:
  exclude  Exclude filenames matching the given patterns when formatting.
  revert   Revert filename changes.
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [FILES]...
          A list of files (of any kind) for which to format the name.
          
          If no file is given, nothing will happen and the program will exit gracefully.

Options:
  -n, --naming-convention <NAMING_CONVENTION>
          The naming convention to use.
          
          The default is "snake_case".
          If one is specified in the config file, it will be used instead.
          
          [possible values: camelCase, kebab-case, snake_case, PascalCase, lower, UPPER]

  -r, --recursive
          Recursively format filenames within directories.
          
          For arguments that are directories, the default is to treat them like
          any other file, that is format their names.
          By using this flag, every file (directories included) within each of
          the directories will be formatted as well.

      --keep-dots
          Don't treat dots as separators, let them as is.
          
          A separator is a character indicating a break between words.
          The characters "_", "-", "." and spaces are considered separators
          and may change according to the chosen naming convention, unless
          this flag is used.

      --keep-special-chars
          Keep special characters.
          
          By special characters we mean characters that are neither alphanumeric
          nor separators ("_", "-", "." and spaces).
          If not set, special characters are removed with the exception of some
          accented letters that are replaced by their non-accented variants.

      --keep-unicode
          Keep Unicode (more precisely, non-ASCII) characters.
          
          When not set, unicode characters to their closest ASCII counterparts
          using https://crates.io/crates/unidecode.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## TODO

- More tests
- User-defined naming convention?
  Have the user write its own Rust module, include it in fmtna (build script is probably the way to go) then recompile.
