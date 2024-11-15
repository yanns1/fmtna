# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.3] - 2024-11-15

### Fixed

- Make sure to process input paths such that files have their names changed before
  their parent/ancestor directories. Otherwise, the renaming of a parent directory
  makes its descendant file paths outdated, causing an error when trying to rename
  them.

- Same idea for when reverting name changes via a history file. Because feedback lines
  are written to the history file in the order of processing, we need to process them
  in reverse order for the same reason as above.

  That wasn't the case, making reverts partially fail. Now it is.

### Added

- Most of the app's logic is now in a library. It's documentation is available on docs.rs.
- This changelog.

## [1.0.2] - 2024-07-12

### Fixed

Fixed a number of bugs that happened on Windows:

- Use a different date format for backup and history filenames, one that
  doesn't contain forbidden characters in Windows paths.
- Fixed buggy terminal prompting due to removing of newlines in a non
  cross-platform way (was removing LF instead of CRLF).
- Take into account the fact that paths are case-insensitive in Windows.
- Spawn the editor (edit subcommand) in cmd as it may not be in PATH.

## [1.0.1] - 2024-07-07

### Fixed

- Fix wrong/incomplete instructions in README.

## [1.0.0] - 2024-07-07

### Added

- Implementation of a program that formats filenames in a given naming convention.

[1.0.0]: https://github.com/yanns1/fmtna/releases/tag/v1.0.0
[1.0.1]: https://github.com/yanns1/fmtna/compare/v1.0.0...v1.0.1
[1.0.2]: https://github.com/yanns1/fmtna/compare/v1.0.1...v1.0.2
[1.0.3]: https://github.com/yanns1/fmtna/compare/v1.0.2...v1.0.3
[unreleased]: https://github.com/yanns1/fmtna/compare/v1.0.3...HEAD
