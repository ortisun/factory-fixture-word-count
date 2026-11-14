# word-count constitution

## Principles

### I. The tool only reads

The tool reads a file or standard input and prints counts. It never writes a file, never changes
one, and never opens a network connection. A change that adds any of these is out of scope.

### II. Plain output stays stable

The plain-text output is a contract with the people and scripts that use it today. A change to it
needs a new major version, so a new option must leave it byte for byte as it is.

### III. Tests run the real program

Behaviour is proved by tests that run the real binary and by unit tests of the counting code. A
sample file under `tests/golden` changes only on purpose and only with a person's review.

### IV. No dependencies

The tool has no dependencies and builds without a network. A new dependency needs a person's
approval before it is added.

## Governance

A person with the maintainer role approves any change to this constitution, apart from any feature
request. Feature requests never rewrite it.

**Version**: 1.0.0 | **Ratified**: 2026-10-28
