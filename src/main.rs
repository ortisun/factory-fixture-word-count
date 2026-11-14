//! The word-count command line tool.

use std::io::{self, Read, Write};
use std::process::ExitCode;

use word_count::{count, format_plain};

const USAGE: &str = "\
Usage: word-count [FILE]

Counts the lines, words and characters of FILE, or of standard input when FILE is
missing or is a single dash.

Options:
  -h, --help     Show this help and exit.
  -V, --version  Show the version and exit.

Exit status: 0 on success, 1 when the input cannot be read, 2 when the command
line is wrong.
";

/// What the command line asked for.
enum Request {
    Help,
    Version,
    Count { path: Option<String> },
}

/// Why the command line was refused.
enum UsageError {
    UnknownOption(String),
    TooManyArguments,
}

fn parse(args: impl IntoIterator<Item = String>) -> Result<Request, UsageError> {
    let mut path: Option<String> = None;
    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Request::Help),
            "-V" | "--version" => return Ok(Request::Version),
            "-" => {}
            option if option.starts_with('-') => {
                return Err(UsageError::UnknownOption(arg));
            }
            _ => {
                if path.is_some() {
                    return Err(UsageError::TooManyArguments);
                }
                path = Some(arg);
            }
        }
    }
    Ok(Request::Count { path })
}

fn read_input(path: Option<&str>) -> io::Result<String> {
    let mut bytes = Vec::new();
    match path {
        Some(path) => bytes = std::fs::read(path)?,
        None => {
            io::stdin().lock().read_to_end(&mut bytes)?;
        }
    }
    String::from_utf8(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "the input is not valid UTF-8"))
}

fn main() -> ExitCode {
    let request = match parse(std::env::args().skip(1)) {
        Ok(request) => request,
        Err(UsageError::UnknownOption(option)) => {
            eprintln!("word-count: unknown option '{option}'");
            eprint!("{USAGE}");
            return ExitCode::from(2);
        }
        Err(UsageError::TooManyArguments) => {
            eprintln!("word-count: only one file can be counted in a run");
            eprint!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match request {
        Request::Help => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        Request::Version => {
            println!("word-count {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Request::Count { path } => match read_input(path.as_deref()) {
            Ok(text) => {
                let line = format_plain(&count(&text), path.as_deref());
                let mut out = io::stdout().lock();
                if writeln!(out, "{line}").is_err() {
                    return ExitCode::from(1);
                }
                ExitCode::SUCCESS
            }
            Err(error) => {
                let name = path.as_deref().unwrap_or("standard input");
                eprintln!("word-count: cannot read {name}: {error}");
                ExitCode::from(1)
            }
        },
    }
}
