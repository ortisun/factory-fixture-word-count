//! The plain-text output, compared with stored samples. The samples live in `tests/golden`, which
//! the project manifest protects, so a change that makes a test pass by editing a sample is
//! stopped before it reaches review.

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_word-count"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(Stdio::null())
        .output()
        .expect("the binary starts")
}

fn golden(name: &str) -> String {
    let path = format!("{}/tests/golden/{name}", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(path).expect("the stored sample exists")
}

#[test]
fn a_file_of_three_lines_and_seven_words_is_counted() {
    let output = run(&["tests/golden/sample.txt"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8"),
        golden("sample.plain.txt")
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn an_empty_file_has_three_zero_counts() {
    let output = run(&["tests/golden/empty.txt"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8"),
        golden("empty.plain.txt")
    );
}

#[test]
fn standard_input_is_counted_when_there_is_no_file() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_word-count"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("the binary starts");
    child
        .stdin
        .take()
        .expect("a pipe")
        .write_all(b"one two three\nfour five\nsix up\n")
        .expect("the write succeeds");
    let output = child.wait_with_output().expect("the binary ends");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8"),
        "      3       7      31\n"
    );
}
