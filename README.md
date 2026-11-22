# word-count

A small command line tool that counts the lines, words and characters of a text file, or of
standard input. It has no dependencies and builds without a network.

This repository is the fixture that the Universal Agentic Software Factory works on in its
end-to-end proof. It is a real tool with real checks, kept small on purpose.

## Install

You need Rust 1.98 or later.

```sh
cargo build --release
```

The program is then at `target/release/word-count`.

## Use

Count one file:

```sh
word-count notes.txt
```

The tool prints three right-aligned columns, then the file name: the number of lines, the number
of words and the number of characters.

```text
      3       7      31 notes.txt
```

Without a file name, or with a single dash, the tool reads standard input and prints no name:

```sh
printf 'one two three\n' | word-count
```

```text
      1       3      14
```

### What is counted

| Count | Meaning |
|---|---|
| Lines | The number of line breaks. A last line with no break at its end is not counted, as in `wc`. |
| Words | The number of runs of characters with no white space in them. |
| Characters | The number of Unicode scalar values. For plain ASCII text this equals the number of bytes. |

### Options

| Option | Effect |
|---|---|
| `-h`, `--help` | Show the help text and exit. |
| `-V`, `--version` | Show the version and exit. |

### Exit status

| Status | Meaning |
|---|---|
| 0 | The counts were printed. |
| 1 | The input could not be read, or it is not valid UTF-8. Nothing is printed on standard output. |
| 2 | The command line is wrong: an unknown option, or more than one file. |

## Develop

These are the commands the project checks run. They work offline.

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build
cargo test
```

The stored samples that the tests compare with are in `tests/golden`. Change them only on purpose:
the project manifest in `.factory/project.yaml` lists that directory as protected.

## Where things are

| Path | What it holds |
|---|---|
| `src/lib.rs` | The counting and the plain-text line. |
| `src/main.rs` | The command line: the arguments, the reading and the exit status. |
| `tests/` | The tests that run the real binary, and the stored samples. |
| `notes.txt` | The sample text the examples under Use count. |
| `.factory/project.yaml` | The project manifest the factory reads: gate commands, protected paths and required documents. |
| `CODEOWNERS` | Who reviews which paths. |
| `CHANGELOG.md` | What changed in each release. |
