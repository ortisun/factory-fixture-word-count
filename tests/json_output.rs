//! The `--json` option, run against the real binary. The expected JSON is written inline, because
//! the exact bytes are themselves a requirement. The plain output is still compared with the
//! stored samples in `tests/golden`.

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

const SAMPLE_JSON: &str = "{\"lines\":3,\"words\":7,\"characters\":31}\n";
const EMPTY_JSON: &str = "{\"lines\":0,\"words\":0,\"characters\":0}\n";
const SAMPLE_TEXT: &[u8] = b"one two three\nfour five\nsix up\n";

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_word-count"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(Stdio::null())
        .output()
        .expect("the binary starts")
}

fn run_with_input(args: &[&str], input: &[u8]) -> std::process::Output {
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
        .expect("a pipe")
        .write_all(input)
        .expect("the write succeeds");
    child.wait_with_output().expect("the binary ends")
}

fn golden(name: &str) -> String {
    let path = format!("{}/tests/golden/{name}", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(path).expect("the stored sample exists")
}

fn stdout(output: &std::process::Output) -> &str {
    std::str::from_utf8(&output.stdout).expect("UTF-8")
}

#[test]
fn the_sample_file_gives_the_exact_compact_object() {
    let output = run(&["--json", "tests/golden/sample.txt"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), SAMPLE_JSON);
    assert!(output.stderr.is_empty());
}

#[test]
fn the_empty_file_gives_all_zeros() {
    let output = run(&["--json", "tests/golden/empty.txt"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), EMPTY_JSON);
    assert!(output.stderr.is_empty());
}

#[test]
fn standard_input_with_no_file_name_gives_the_object_without_a_name() {
    let output = run_with_input(&["--json"], SAMPLE_TEXT);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), SAMPLE_JSON);
    assert!(output.stderr.is_empty());
}

#[test]
fn standard_input_with_a_dash_gives_the_object_without_a_name() {
    let output = run_with_input(&["--json", "-"], SAMPLE_TEXT);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), SAMPLE_JSON);
    assert!(output.stderr.is_empty());
}

#[test]
fn a_missing_file_gives_no_output_and_an_error() {
    let output = run(&["--json", "tests/golden/missing.txt"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let message = String::from_utf8(output.stderr).expect("UTF-8");
    assert!(message.contains("cannot read"), "stderr was: {message}");
}

#[test]
fn the_flag_can_come_before_or_after_the_file_name() {
    let before = run(&["--json", "tests/golden/sample.txt"]);
    let after = run(&["tests/golden/sample.txt", "--json"]);
    assert_eq!(stdout(&before), SAMPLE_JSON);
    assert_eq!(stdout(&after), SAMPLE_JSON);
    assert!(after.status.success());
    assert!(after.stderr.is_empty());
}

#[test]
fn plain_output_for_the_sample_file_matches_the_stored_sample() {
    let output = run(&["tests/golden/sample.txt"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), golden("sample.plain.txt"));
    assert!(output.stderr.is_empty());
}

#[test]
fn plain_output_for_the_empty_file_matches_the_stored_sample() {
    let output = run(&["tests/golden/empty.txt"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), golden("empty.plain.txt"));
    assert!(output.stderr.is_empty());
}
