//! The JSON output, checked through the real binary. The golden files are only read, never
//! written.

use std::fs;
use std::io::Write;
use std::process::{Command, Output, Stdio};

const SAMPLE_JSON: &str = "{\"lines\":3,\"words\":7,\"characters\":31}\n";
const EMPTY_JSON: &str = "{\"lines\":0,\"words\":0,\"characters\":0}\n";

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_word-count"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(Stdio::null())
        .output()
        .expect("the binary starts")
}

fn run_with_input(args: &[&str], input: &[u8]) -> Output {
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

fn text(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("UTF-8")
}

fn assert_same(with: &Output, without: &Output) {
    assert_eq!(with.stdout, without.stdout);
    assert_eq!(with.stderr, without.stderr);
    assert_eq!(with.status.code(), without.status.code());
}

#[test]
fn the_sample_file_gives_the_exact_json_line() {
    let output = run(&["--json", "tests/golden/sample.txt"]);
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_empty_file_gives_all_zero_counts() {
    let output = run(&["--json", "tests/golden/empty.txt"]);
    assert_eq!(text(&output.stdout), EMPTY_JSON);
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn standard_input_gives_the_same_object_as_a_file() {
    let output = run_with_input(&["--json"], b"one two three\nfour five\nsix up\n");
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_option_may_come_before_after_or_repeat() {
    for args in [
        &["--json", "tests/golden/sample.txt"][..],
        &["tests/golden/sample.txt", "--json"][..],
        &["--json", "--json", "tests/golden/sample.txt"][..],
        &["--json", "tests/golden/sample.txt", "--json"][..],
    ] {
        let output = run(args);
        assert_eq!(text(&output.stdout), SAMPLE_JSON, "{args:?}");
        assert!(output.stderr.is_empty(), "{args:?}");
        assert_eq!(output.status.code(), Some(0), "{args:?}");
    }
}

#[test]
fn a_missing_file_fails_with_and_without_the_option() {
    for args in [
        &["--json", "tests/golden/missing.txt"][..],
        &["tests/golden/missing.txt"][..],
    ] {
        let output = run(args);
        assert!(output.stdout.is_empty(), "{args:?}");
        assert!(text(&output.stderr).contains("cannot read"), "{args:?}");
        assert_eq!(output.status.code(), Some(1), "{args:?}");
    }
}

#[test]
fn other_paths_behave_the_same_with_and_without_the_option() {
    let cases: [&[&str]; 4] = [
        &["tests/golden/sample.txt", "tests/golden/empty.txt"],
        &["--bogus"],
        &["--help"],
        &["--version"],
    ];
    for case in cases {
        let mut with = vec!["--json"];
        with.extend_from_slice(case);
        assert_same(&run(&with), &run(case));
    }
}

#[test]
fn two_file_names_print_no_json_and_exit_with_two() {
    let output = run(&["--json", "tests/golden/sample.txt", "tests/golden/empty.txt"]);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn the_help_text_does_not_mention_the_option() {
    let output = run(&["--help"]);
    assert!(output.status.success());
    assert!(!text(&output.stdout).contains("--json"));
}

#[test]
fn plain_output_equals_the_stored_samples() {
    assert_eq!(
        text(&run(&["tests/golden/sample.txt"]).stdout),
        golden("sample.plain.txt")
    );
    assert_eq!(
        text(&run(&["tests/golden/empty.txt"]).stdout),
        golden("empty.plain.txt")
    );
}
