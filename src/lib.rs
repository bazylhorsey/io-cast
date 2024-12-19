use std::io::{self, BufRead, Write};
use std::str::FromStr;

/// A custom error type that wraps I/O errors and parsing errors.
#[derive(Debug)]
pub enum InputError<E: std::fmt::Debug> {
    Io(io::Error),
    Parse(E),
}

impl<E: std::fmt::Display + std::fmt::Debug> std::fmt::Display for InputError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Io(e) => write!(f, "I/O error: {}", e),
            InputError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl<E: std::fmt::Debug + std::fmt::Display> std::error::Error for InputError<E> {}

/// A generic function to read a line from `reader`, trim it, and parse it into the desired type.
///
/// # Arguments
///
/// * `reader`: A buffered reader from which input is read.
/// * `prompt`: An optional prompt string to display before reading input.
///
/// # Returns
///
/// A `Result` containing the parsed value or an `InputError`.
fn read_and_parse_from<R, T>(reader: &mut R, prompt: Option<&str>) -> Result<T, InputError<T::Err>>
where
    R: BufRead,
    T: FromStr,
    T::Err: std::fmt::Debug + std::fmt::Display,
{
    if let Some(p) = prompt {
        // If you want to capture this in tests, you might inject a writer or mock stdout.
        print!("{}", p);
        io::stdout().flush().map_err(InputError::Io)?;
    }

    let mut input = String::new();
    reader.read_line(&mut input).map_err(InputError::Io)?;

    let trimmed_input = input.trim_end_matches(|c| c == '\r' || c == '\n');

    trimmed_input.parse::<T>().map_err(InputError::Parse)
}

/// A generic function to read a line from `reader`, parse it, and handle EOF by returning `Ok(None)`.
///
/// # Arguments
///
/// * `reader`: A buffered reader from which input is read.
/// * `prompt`: An optional prompt string.
///
/// # Returns
///
/// A `Result` containing an `Option<T>` or an `InputError`.
fn read_and_parse_with_eof_from<R, T>(
    reader: &mut R,
    prompt: Option<&str>,
) -> Result<Option<T>, InputError<T::Err>>
where
    R: BufRead,
    T: FromStr,
    T::Err: std::fmt::Debug + std::fmt::Display,
{
    if let Some(p) = prompt {
        print!("{}", p);
        io::stdout().flush().map_err(InputError::Io)?;
    }

    let mut input = String::new();
    match reader.read_line(&mut input) {
        Ok(0) => Ok(None), // EOF
        Ok(_) => {
            let trimmed_input = input.trim_end_matches('\r').trim_end_matches('\n');
            if trimmed_input.is_empty() && input.ends_with('\n') {
                Ok(None) // Consider an empty line as EOF
            } else {
                trimmed_input.parse::<T>().map(Some).map_err(InputError::Parse)
            }
        }
        Err(e) => Err(InputError::Io(e)),
    }
}

/// Production versions using `io::stdin()`:
pub fn read_and_parse<T: FromStr>() -> Result<T, InputError<T::Err>>
where
    T::Err: std::fmt::Debug + std::fmt::Display,
{
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    read_and_parse_from(&mut reader, None)
}

pub fn read_and_parse_with_prompt<T: FromStr>(prompt: &str) -> Result<T, InputError<T::Err>>
where
    T::Err: std::fmt::Debug + std::fmt::Display,
{
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    read_and_parse_from(&mut reader, Some(prompt))
}

pub fn read_and_parse_with_eof<T: FromStr>() -> Result<Option<T>, InputError<T::Err>>
where
    T::Err: std::fmt::Debug + std::fmt::Display,
{
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    read_and_parse_with_eof_from(&mut reader, None)
}

pub fn read_and_parse_with_prompt_eof<T: FromStr>(prompt: &str) -> Result<Option<T>, InputError<T::Err>>
where
    T::Err: std::fmt::Debug + std::fmt::Display,
{
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    read_and_parse_with_eof_from(&mut reader, Some(prompt))
}

/// Macros as before (now using the production versions)
#[macro_export]
macro_rules! input {
    () => {
        $crate::read_and_parse_with_eof::<String>()
    };
    ($prompt:expr) => {
        $crate::read_and_parse_with_eof_from(&mut ::std::io::stdin().lock(), Some($prompt))
    };
}

#[macro_export]
macro_rules! input_no_eof {
    () => {
        $crate::read_and_parse::<String>()
    };
    ($prompt:expr) => {
        $crate::read_and_parse_from(&mut ::std::io::stdin().lock(), Some($prompt))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_and_parse_float() {
        let input_data = b"3.14\n";
        let mut reader = Cursor::new(input_data);
        let result: f64 = read_and_parse_from(&mut reader, None).unwrap();
        assert_eq!(result, 3.14);
    }

    #[test]
    fn test_read_and_parse_integer() {
        let input_data = b"42\n";
        let mut reader = Cursor::new(input_data);
        let result: i32 = read_and_parse_from(&mut reader, None).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_read_and_parse_string() {
        let input_data = b"hello world\n";
        let mut reader = Cursor::new(input_data);
        let result: String = read_and_parse_from(&mut reader, None).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_read_and_parse_with_prompt() {
        // The prompt doesn't affect input parsing, but in a real scenario,
        // you'd check stdout if needed. Here, we just ensure it parses correctly.
        let input_data = b"100\n";
        let mut reader = Cursor::new(input_data);
        let result: i32 = read_and_parse_from(&mut reader, Some("Enter a number: ")).unwrap();
        assert_eq!(result, 100);
    }

    #[test]
    fn test_unix_line_ending() {
        let input_data = b"123\n";
        let mut reader = Cursor::new(input_data);
        let result: i32 = read_and_parse_from(&mut reader, None).unwrap();
        assert_eq!(result, 123);
    }

    #[test]
    fn test_windows_line_ending() {
        let input_data = b"456\r\n";
        let mut reader = Cursor::new(input_data);
        let result: i32 = read_and_parse_from(&mut reader, None).unwrap();
        assert_eq!(result, 456);
    }

    #[test]
    fn test_read_and_parse_with_eof() {
        let input_data = b"";
        let mut reader = Cursor::new(input_data);
        let result: Option<i32> = read_and_parse_with_eof_from(&mut reader, None).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_read_and_parse_with_eof_nonempty() {
        let input_data = b"789\n";
        let mut reader = Cursor::new(input_data);
        let result: Option<i32> = read_and_parse_with_eof_from(&mut reader, None).unwrap();
        assert_eq!(result, Some(789));
    }
}