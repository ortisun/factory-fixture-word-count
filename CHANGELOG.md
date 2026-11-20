# Changelog

All notable changes to this project are listed here, newest first. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Unreleased

### Added

- The `--json` option, which prints the three counts as one compact JSON object with the keys
  `lines`, `words` and `characters`. The plain output and the exit statuses are unchanged.

## 0.1.0

### Added

- The `word-count` command, which prints the number of lines, words and characters of a file or
  of standard input.
- The `--help` and `--version` options.
- Exit status 1 when the input cannot be read and 2 when the command line is wrong.
