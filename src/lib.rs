//! Counting for the word-count tool.
//!
//! The functions here read no file and print nothing. The binary in `main.rs` does the reading and
//! the printing, so the counting can be tested on plain strings.

/// The counts of one text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Counts {
    /// The number of line breaks. A last line with no break at its end is not counted, as in `wc`.
    pub lines: usize,
    /// The number of words. A word is a run of characters with no white space in it.
    pub words: usize,
    /// The number of characters, counted as Unicode scalar values.
    pub characters: usize,
}

/// Counts the lines, words and characters of `text`.
#[must_use]
pub fn count(text: &str) -> Counts {
    Counts {
        lines: text.bytes().filter(|byte| *byte == b'\n').count(),
        words: text.split_whitespace().count(),
        characters: text.chars().count(),
    }
}

/// The plain-text line the tool prints: three right-aligned columns (lines, words, characters),
/// then the name of the file when there is one.
#[must_use]
pub fn format_plain(counts: &Counts, name: Option<&str>) -> String {
    let columns = format!(
        "{:>7} {:>7} {:>7}",
        counts.lines, counts.words, counts.characters
    );
    match name {
        Some(name) => format!("{columns} {name}"),
        None => columns,
    }
}

/// The JSON object the tool prints with `--json`: the keys `lines`, `words` and `characters` in
/// that order, whole-number values and no white space. It has no trailing newline; the caller adds
/// it.
#[must_use]
pub fn format_json(counts: &Counts) -> String {
    format!(
        "{{\"lines\":{},\"words\":{},\"characters\":{}}}",
        counts.lines, counts.words, counts.characters
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_text_has_no_lines_words_or_characters() {
        assert_eq!(count(""), Counts::default());
    }

    #[test]
    fn each_line_break_is_one_line() {
        assert_eq!(count("a\nb\nc\n").lines, 3);
    }

    #[test]
    fn a_last_line_without_a_break_is_not_a_line() {
        assert_eq!(count("a\nb").lines, 1);
    }

    #[test]
    fn words_are_split_on_any_white_space() {
        assert_eq!(count("one  two\tthree\nfour").words, 4);
    }

    #[test]
    fn a_text_of_only_white_space_has_no_words() {
        let counts = count(" \t\n  \n");
        assert_eq!(counts.words, 0);
        assert_eq!(counts.lines, 2);
    }

    #[test]
    fn characters_are_scalar_values_and_not_bytes() {
        let counts = count("caf\u{e9}\n");
        assert_eq!(counts.characters, 5);
    }

    #[test]
    fn the_json_line_has_the_three_keys_in_order() {
        let counts = Counts {
            lines: 3,
            words: 7,
            characters: 31,
        };
        assert_eq!(
            format_json(&counts),
            r#"{"lines":3,"words":7,"characters":31}"#
        );
    }

    #[test]
    fn the_json_line_of_zero_counts_has_zero_values() {
        assert_eq!(
            format_json(&Counts::default()),
            r#"{"lines":0,"words":0,"characters":0}"#
        );
    }

    #[test]
    fn the_json_line_has_no_white_space_and_no_newline() {
        let json = format_json(&count("a b\nc\n"));
        assert!(!json.contains(char::is_whitespace));
    }

    #[test]
    fn the_plain_line_is_three_columns_and_an_optional_name() {
        let counts = Counts {
            lines: 3,
            words: 7,
            characters: 31,
        };
        assert_eq!(format_plain(&counts, None), "      3       7      31");
        assert_eq!(
            format_plain(&counts, Some("notes.txt")),
            "      3       7      31 notes.txt"
        );
    }
}
