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
fn fr_001_fr_002_fr_003_fr_011_the_sample_file_gives_the_exact_json_line() {
    let output = run(&["--json", "tests/golden/sample.txt"]);
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn fr_004_the_empty_file_gives_all_zero_counts() {
    let output = run(&["--json", "tests/golden/empty.txt"]);
    assert_eq!(text(&output.stdout), EMPTY_JSON);
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn fr_010_standard_input_gives_the_same_object_as_a_file() {
    let output = run_with_input(&["--json"], b"one two three\nfour five\nsix up\n");
    assert_eq!(text(&output.stdout), SAMPLE_JSON);
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn fr_001_the_option_may_come_before_after_or_repeat() {
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
fn fr_006_a_missing_file_fails_with_and_without_the_option() {
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
fn fr_012_other_paths_behave_the_same_with_and_without_the_option() {
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
fn fr_012_two_file_names_print_no_json_and_exit_with_two() {
    let output = run(&[
        "--json",
        "tests/golden/sample.txt",
        "tests/golden/empty.txt",
    ]);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn fr_013_the_help_text_does_not_mention_the_option() {
    let output = run(&["--help"]);
    assert!(output.status.success());
    assert!(!text(&output.stdout).contains("--json"));
}

#[test]
fn fr_005_plain_output_equals_the_stored_samples() {
    assert_eq!(
        text(&run(&["tests/golden/sample.txt"]).stdout),
        golden("sample.plain.txt")
    );
    assert_eq!(
        text(&run(&["tests/golden/empty.txt"]).stdout),
        golden("empty.plain.txt")
    );
}

#[test]
fn fr_007_the_readme_documents_the_option() {
    let path = format!("{}/README.md", env!("CARGO_MANIFEST_DIR"));
    assert!(fs::read_to_string(path).expect("README").contains("--json"));
}

#[test]
fn fr_008_the_changelog_lists_the_option() {
    let path = format!("{}/CHANGELOG.md", env!("CARGO_MANIFEST_DIR"));
    assert!(
        fs::read_to_string(path)
            .expect("changelog")
            .contains("--json")
    );
}

#[test]
fn fr_009_the_manifest_has_no_dependencies() {
    let path = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
    let manifest = fs::read_to_string(path).expect("manifest");
    let section = manifest
        .split("[dependencies]")
        .nth(1)
        .map(|rest| rest.split("\n[").next().unwrap_or(""))
        .unwrap_or("");
    assert!(section.trim().is_empty());
}
