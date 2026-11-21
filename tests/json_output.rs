//! The `--json` option, end to end, against the real binary.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const SAMPLE: &str = "one two three\nfour five\nsix up\n";
const SAMPLE_JSON: &str = "{\"lines\":3,\"words\":7,\"characters\":31}\n";
const EMPTY_JSON: &str = "{\"lines\":0,\"words\":0,\"characters\":0}\n";

/// A temporary directory that is removed when the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("word-count-json-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("the directory is created");
        Scratch(dir)
    }

    fn file(&self, name: &str, bytes: &[u8]) -> String {
        let path = self.0.join(name);
        std::fs::write(&path, bytes).expect("the file is written");
        path.to_str().expect("UTF-8 path").to_string()
    }

    fn path(&self, name: &str) -> String {
        self.0.join(name).to_str().expect("UTF-8 path").to_string()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(Path::new(&self.0));
    }
}

fn run_with_stdin(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_word-count"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary starts");
    child
        .stdin
        .take()
        .expect("standard input is piped")
        .write_all(input)
        .expect("the input is written");
    child.wait_with_output().expect("the binary ends")
}

fn run(args: &[&str]) -> Output {
    run_with_stdin(args, b"")
}

fn text(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("UTF-8")
}

/// Runs `args` with and without `--json` and asserts that nothing differs.
fn assert_same_without_json(args: &[&str], expected_code: i32) {
    let with: Vec<&str> = std::iter::once("--json")
        .chain(args.iter().copied())
        .collect();
    let a = run(&with);
    let b = run(args);
    assert_eq!(a.status.code(), Some(expected_code));
    assert_eq!(a.status.code(), b.status.code());
    assert_eq!(a.stdout, b.stdout);
    assert_eq!(a.stderr, b.stderr);
}

#[test]
fn fr_002_fr_010_the_sample_file_gives_the_exact_bytes() {
    let dir = Scratch::new("sample");
    let file = dir.file("notes.txt", SAMPLE.as_bytes());
    let output = run(&["--json", &file]);
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
}

#[test]
fn fr_003_an_empty_file_gives_zeros() {
    let dir = Scratch::new("empty");
    let file = dir.file("empty.txt", b"");
    let output = run(&["--json", &file]);
    assert_eq!(text(&output.stdout), EMPTY_JSON);
}

#[test]
fn fr_014_a_successful_run_exits_with_zero_and_a_clean_standard_error() {
    let dir = Scratch::new("clean");
    let file = dir.file("notes.txt", SAMPLE.as_bytes());
    let output = run(&["--json", &file]);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}

#[test]
fn fr_009_standard_input_without_a_file_gives_the_same_object() {
    let output = run_with_stdin(&["--json"], SAMPLE.as_bytes());
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}

#[test]
fn fr_009_standard_input_with_a_dash_gives_the_same_object() {
    let output = run_with_stdin(&["--json", "-"], SAMPLE.as_bytes());
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
    let output = run_with_stdin(&["-", "--json"], SAMPLE.as_bytes());
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
}

#[test]
fn fr_009_empty_standard_input_gives_zeros() {
    let output = run_with_stdin(&["--json"], b"");
    assert_eq!(text(&output.stdout), EMPTY_JSON);
}

#[test]
fn fr_001_fr_012_the_flag_works_before_after_and_repeated() {
    let dir = Scratch::new("position");
    let file = dir.file("notes.txt", SAMPLE.as_bytes());
    for args in [
        vec!["--json", file.as_str()],
        vec![file.as_str(), "--json"],
        vec!["--json", "--json", file.as_str()],
        vec!["--json", file.as_str(), "--json"],
    ] {
        let output = run(&args);
        assert_eq!(text(&output.stdout), SAMPLE_JSON, "arguments {args:?}");
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
    }
}

fn golden(name: &str) -> String {
    let path = format!("{}/tests/golden/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path).expect("the stored sample exists")
}

#[test]
fn fr_004_the_run_without_the_flag_matches_the_stored_plain_samples() {
    for (input, expected) in [
        ("tests/golden/sample.txt", "sample.plain.txt"),
        ("tests/golden/empty.txt", "empty.plain.txt"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_word-count"))
            .arg(input)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .stdin(Stdio::null())
            .output()
            .expect("the binary starts");
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(text(&output.stdout), golden(expected));
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn fr_012_help_is_the_same_with_and_without_the_flag() {
    assert_same_without_json(&["--help"], 0);
    assert_same_without_json(&["-h"], 0);
}

#[test]
fn fr_012_version_is_the_same_with_and_without_the_flag() {
    assert_same_without_json(&["--version"], 0);
}

#[test]
fn fr_012_an_unknown_option_is_the_same_with_and_without_the_flag() {
    assert_same_without_json(&["--no-such-option"], 2);
    let output = run(&["--json", "--no-such-option"]);
    assert!(output.stdout.is_empty());
}

#[test]
fn fr_011_two_files_are_refused_as_without_the_flag() {
    assert_same_without_json(&["a.txt", "b.txt"], 2);
    let output = run(&["--json", "a.txt", "b.txt"]);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn fr_011_fr_012_two_files_are_refused_with_the_flag_in_any_position() {
    let a = run(&["a.txt", "--json", "b.txt"]);
    let b = run(&["a.txt", "b.txt"]);
    assert_eq!(a.status.code(), Some(2));
    assert_eq!(a.stdout, b.stdout);
    assert_eq!(a.stderr, b.stderr);
}

#[test]
fn fr_005_a_missing_file_is_the_same_with_and_without_the_flag() {
    let dir = Scratch::new("missing");
    let file = dir.path("missing.txt");
    assert_same_without_json(&[&file], 1);
    let output = run(&["--json", &file]);
    assert!(output.stdout.is_empty());
    assert!(text(&output.stderr).contains("cannot read"));
}

#[test]
fn fr_013_a_directory_is_the_same_with_and_without_the_flag() {
    let dir = Scratch::new("directory");
    let path = dir.path("");
    assert_same_without_json(&[&path], 1);
    let output = run(&["--json", &path]);
    assert!(output.stdout.is_empty());
}

#[test]
fn fr_013_invalid_utf8_in_a_file_is_the_same_with_and_without_the_flag() {
    let dir = Scratch::new("invalid");
    let file = dir.file("bad.txt", &[b'a', b' ', 0xff, 0xfe, b'\n']);
    assert_same_without_json(&[&file], 1);
    let output = run(&["--json", &file]);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn fr_013_invalid_utf8_on_standard_input_is_the_same_with_and_without_the_flag() {
    let bad = [b'a', 0xff, b'\n'];
    let a = run_with_stdin(&["--json"], &bad);
    let b = run_with_stdin(&[], &bad);
    assert_eq!(a.status.code(), Some(1));
    assert!(a.stdout.is_empty());
    assert_eq!(a.stdout, b.stdout);
    assert_eq!(a.stderr, b.stderr);
    assert_eq!(a.status.code(), b.status.code());
}

#[test]
fn fr_004_a_missing_file_without_the_flag_prints_exactly_the_error() {
    let dir = Scratch::new("missing-plain");
    let file = dir.path("missing.txt");
    let reason = std::fs::read(&file).expect_err("the file is missing");
    let output = run(&[&file]);
    assert!(output.stdout.is_empty());
    assert_eq!(
        text(&output.stderr),
        format!("word-count: cannot read {file}: {reason}\n")
    );
    assert_eq!(output.status.code(), Some(1));
}

/// A read error on standard input: reading from a directory fails on Unix platforms.
#[cfg(unix)]
#[test]
fn fr_013_a_read_error_on_standard_input_is_the_same_with_and_without_the_flag() {
    let dir = Scratch::new("stdin-directory");
    let run_on_directory = |args: &[&str]| {
        let directory = std::fs::File::open(&dir.0).expect("the directory opens");
        Command::new(env!("CARGO_BIN_EXE_word-count"))
            .args(args)
            .stdin(Stdio::from(directory))
            .output()
            .expect("the binary starts")
    };
    let a = run_on_directory(&["--json"]);
    let b = run_on_directory(&[]);
    assert_eq!(a.status.code(), Some(1));
    assert!(a.stdout.is_empty());
    assert!(text(&a.stderr).starts_with("word-count: cannot read standard input: "));
    assert_eq!(a.stdout, b.stdout);
    assert_eq!(a.stderr, b.stderr);
    assert_eq!(a.status.code(), b.status.code());
}

#[test]
fn fr_006_the_readme_documents_the_json_option() {
    let readme = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
        .expect("the README is readable");
    assert!(readme.contains("--json"));
}

#[test]
fn fr_007_the_changelog_lists_the_json_option_under_unreleased() {
    let changelog = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/CHANGELOG.md"))
        .expect("the changelog is readable");
    let unreleased = changelog
        .split("\n## ")
        .find(|part| part.starts_with("[Unreleased]") || part.starts_with("Unreleased"))
        .or_else(|| changelog.split("## ").find(|p| p.contains("Unreleased")))
        .expect("an Unreleased section exists");
    assert!(unreleased.contains("--json"));
}

#[test]
fn fr_008_the_manifest_has_no_third_party_dependencies() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("the manifest is readable");
    let dependencies = manifest
        .split("[dependencies]")
        .nth(1)
        .map(|rest| rest.split("\n[").next().unwrap_or(""))
        .unwrap_or("");
    assert!(dependencies.trim().is_empty());
}
