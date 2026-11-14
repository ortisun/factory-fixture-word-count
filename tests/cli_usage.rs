//! The command line: help, version and the refusals.

use std::process::{Command, Stdio};

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_word-count"))
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("the binary starts")
}

#[test]
fn help_lists_the_options_and_exits_with_zero() {
    let output = run(&["--help"]);
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).expect("UTF-8");
    assert!(text.contains("Usage: word-count [FILE]"));
    assert!(text.contains("--help"));
    assert!(text.contains("--version"));
}

#[test]
fn version_prints_the_package_version() {
    let output = run(&["--version"]);
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).expect("UTF-8");
    assert_eq!(
        text.trim(),
        format!("word-count {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn an_unknown_option_is_a_usage_error() {
    let output = run(&["--no-such-option"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let text = String::from_utf8(output.stderr).expect("UTF-8");
    assert!(text.contains("unknown option '--no-such-option'"));
}

#[test]
fn two_files_in_one_run_are_a_usage_error() {
    let output = run(&["a.txt", "b.txt"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}

#[test]
fn a_file_that_does_not_exist_is_a_read_error() {
    let output = run(&["tests/golden/no-such-file.txt"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let text = String::from_utf8(output.stderr).expect("UTF-8");
    assert!(text.contains("cannot read tests/golden/no-such-file.txt"));
}
