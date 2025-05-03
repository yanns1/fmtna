# fmtna

[![Build status](https://img.shields.io/github/actions/workflow/status/yanns1/fmtna/build_and_test.yml?style=flat-square)](https://github.com/yanns1/fmtna/actions/workflows/build_and_test.yml?query=branch%3Amain)
[![Crates.io version](https://img.shields.io/crates/v/fmtna?style=flat-square&color=orange)](https://crates.io/crates/fmtna)
[![Crates.io downloads](https://img.shields.io/crates/d/fmtna?style=flat-square)](https://crates.io/crates/fmtna)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue?style=flat-square)](https://docs.rs/fmtna)
[![License](https://img.shields.io/badge/license-GPL%203.0-blue?style=flat-square)](https://github.com/yanns1/fmtna/blob/main/LICENSE)

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

See <https://github.com/yanns1/fmtna/releases>.

### Download fmtna from crates.io

#### Prerequisites

- You need Rust installed (more specifically cargo).

Run `cargo install fmtna` in your terminal.

### Build from source

#### Prerequisites

- You need Rust installed (more specifically cargo).
- You need Git installed.

Clone this repo and run `./install.sh` from the root of the repo (you might need to give the script executable permission).
`install.sh` builds the project using cargo then makes a symlink at `~/.local/bin/fmtna` targeting the executable produced.

## Usage

Everything is explained in `fmtna --help`.

```text
Format filenames according to a chosen naming convention.

For each file/path (of any kind) given as argument, change the filename
(i.e. the base of the path) according to the selected naming convention.

WARNING! This program is dangerous.
Changing filenames is error prone and may cause undesired consequences
(some files are expected to have the name they have and not something else!).
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

          Possible values:
          - camelCase:  The camelCase naming convention
          - kebab-case: The kebab-case naming convention
          - snake_case: The snake_case naming convention
          - PascalCase: The PascalCase naming convention
          - lower:      The lowercase naming convention
          - UPPER:      The UPPERCASE naming convention

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

          When not set, convert unicode characters to their closest ASCII
          counterparts using <https://crates.io/crates/unidecode>.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## TODO

- User-defined naming convention?
  Have the user write its own Rust module, include it in fmtna (build script is probably the way to go) then recompile.
- *Integration tests*: Cumbersome given that many uses of the app are interactive.
  Furthermore, the codebase is relatively small, so I am rather confident in it.
  But in case I want to do integration tests, here are useful ideas:

  - Before starting to write tests, think about what is worth testing, partition
    the input space wisely.
  - Making the application's code more generic can help for testing (for example
    being generic over `Read` and `Write` traits).
    See <https://rust-cli.github.io/book/tutorial/testing.html>.
  - [assert_fs](https://docs.rs/assert_fs/latest/assert_fs/): Filesystem fixtures
    and assertions for testing.
  - [assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/): Easy command
    initialization and assertions.
  - [predicates](https://docs.rs/predicates/latest/predicates/): Composable
    first-order predicate functions.
  - [lit](https://docs.rs/lit/latest/lit/): A reusable testing tool, inspired by
    LLVM’s lit tool. It defines a DSL for matching text outputs easily.
    See <https://www.neilhenning.dev/posts/rust-lit/> also.
  - [rexpect](https://docs.rs/rexpect/latest/rexpect/): Assert against the output
    of an interactive CLI, and send input back to it. See
    <https://www.rustadventure.dev/building-a-digital-garden-cli/clap-v4/testing-interactive-clis-with-rexpect>
    for a simple tutorial.

