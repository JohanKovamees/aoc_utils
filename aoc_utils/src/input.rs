use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("parse error: {0}")]
    Parse(String),
}

/// Read entire file into a String.
pub fn read_to_string(path: impl AsRef<Path>) -> Result<String, InputError> {
    Ok(fs::read_to_string(path)?)
}

/// Read all of stdin into a String.
pub fn read_stdin() -> Result<String, InputError> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

/// Split string into lines and parse each line into T.
pub fn parse_lines<T>(s: &str) -> Result<Vec<T>, InputError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            line.trim().parse::<T>().map_err(|e| {
                InputError::Parse(format!("failed to parse '{line}': {e}"))
            })
        })
        .collect()
}

/// Split input into "groups" separated by blank lines.
pub fn groups(s: &str) -> Vec<&str> {
    s.split("\n\n").collect()
}

/// Parse a single comma- (or custom) separated line into Vec<T>.
pub fn parse_separated<T>(s: &str, sep: char) -> Result<Vec<T>, InputError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    s.trim()
        .split(sep)
        .filter(|p| !p.is_empty())
        .map(|p| {
            p.trim().parse::<T>().map_err(|e| {
                InputError::Parse(format!("failed to parse '{p}': {e}"))
            })
        })
        .collect()
}

/// Parse a grid of characters into Vec<Vec<char>>.
pub fn char_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().filter(|l| !l.is_empty()).map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

    // ---- InputError tests ----

    #[test]
    fn input_error_display_for_parse() {
        let err = InputError::Parse("something went wrong".into());
        assert_eq!(format!("{err}"), "parse error: something went wrong");
    }

    // ---- read_to_string tests ----

    #[test]
    fn read_to_string_reads_entire_file() -> Result<(), InputError> {
        // create a temp file
        let mut path = env::temp_dir();
        path.push("aoc_utils_test_read_to_string.txt");

        let mut file = File::create(&path).expect("failed to create temp file");
        write!(file, "hello\nworld").expect("failed to write temp file");

        // call the function under test
        let contents = read_to_string(&path)?;
        assert_eq!(contents, "hello\nworld");

        // cleanup
        fs::remove_file(&path).ok();

        Ok(())
    }

    #[test]
    fn read_to_string_missing_file_returns_io_error() {
        let path = "this_file_should_not_exist_12345.txt";
        let err = read_to_string(path).unwrap_err();

        match err {
            InputError::Io(_) => {} // expected
            other => panic!("expected InputError::Io, got {other:?}"),
        }
    }

    // read_stdin is a very thin wrapper around io::stdin().read_to_string().
    // It's usually better covered by an integration test that runs the binary
    // and pipes data on stdin, so we don't unit-test it here.

    // ---- parse_lines tests ----

    #[test]
    fn parse_lines_parses_non_empty_trimmed_lines() {
        let input = "1\n  2  \n\n3\n";
        let nums: Vec<i32> = parse_lines(input).expect("parse_lines failed");
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn parse_lines_returns_parse_error_on_bad_line() {
        let input = "10\nfoo\n20\n";
        let err = parse_lines::<i32>(input).unwrap_err();

        match err {
            InputError::Parse(msg) => {
                assert!(msg.contains("foo"), "error message did not mention bad line: {msg}");
            }
            other => panic!("expected InputError::Parse, got {other:?}"),
        }
    }

    // ---- groups tests ----

    #[test]
    fn groups_splits_on_double_newline() {
        let input = "a\nb\n\nc\nd\n";
        let g = groups(input);
        assert_eq!(g, vec!["a\nb", "c\nd\n"]);
    }

    #[test]
    fn groups_keeps_trailing_empty_segment() {
        let input = "a\n\n";
        let g = groups(input);
        // "a" + "\n\n" + "" => ["a", ""]
        assert_eq!(g, vec!["a", ""]);
    }

    // ---- parse_separated tests ----

    #[test]
    fn parse_separated_parses_and_trims_and_skips_empty() {
        let input = "1, 2 ,3,";
        let nums: Vec<i32> = parse_separated(input, ',').expect("parse_separated failed");
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn parse_separated_returns_parse_error_on_bad_piece() {
        let input = "1,foo,2";
        let err = parse_separated::<i32>(input, ',').unwrap_err();

        match err {
            InputError::Parse(msg) => {
                assert!(msg.contains("foo"), "error message did not mention bad piece: {msg}");
            }
            other => panic!("expected InputError::Parse, got {other:?}"),
        }
    }

    // ---- char_grid tests ----

    #[test]
    fn char_grid_ignores_empty_lines_and_splits_chars() {
        let input = "abc\n\ndef\n";
        let grid = char_grid(input);

        assert_eq!(
            grid,
            vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
            ]
        );
    }
}
