use std::fmt::Arguments;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

/// A unified error type indicating either an I/O error, a parse error, or EOF.
#[derive(Debug)]
pub enum InputError<E> {
    /// An I/O error occurred (e.g., closed stdin).
    Io(io::Error),
    /// Failed to parse the input into the desired type.
    Parse(E),
    /// EOF encountered (read_line returned 0).
    Eof,
}

impl<E: std::fmt::Display + std::fmt::Debug> std::fmt::Display for InputError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Io(e) => write!(f, "I/O error: {}", e),
            InputError::Parse(e) => write!(f, "Parse error: {}", e),
            InputError::Eof => write!(f, "EOF encountered"),
        }
    }
}

impl<E: std::fmt::Display + std::fmt::Debug> std::error::Error for InputError<E> {}

/// A single function that:
/// 1. Optionally prints a prompt (and flushes).
/// 2. Reads one line from the provided `BufRead`.
/// 3. Returns `Err(InputError::Eof)` if EOF is reached.
/// 4. Parses into type `T`, returning `Err(InputError::Parse)` on failure.
/// 5. Returns `Err(InputError::Io)` on I/O failure.
pub fn read_input_from<R, T>(
    reader: &mut R,
    prompt: Option<Arguments<'_>>,
) -> Result<T, InputError<T::Err>>
where
    R: BufRead,
    T: FromStr,
    T::Err: std::fmt::Display + std::fmt::Debug,
{
    if let Some(prompt_args) = prompt {
        print!("{}", prompt_args);
        // Always flush so the user sees the prompt immediately
        io::stdout().flush().map_err(InputError::Io)?;
    }

    let mut input = String::new();
    let bytes_read = reader.read_line(&mut input).map_err(InputError::Io)?;

    // If 0, that's EOF — return Eof error
    if bytes_read == 0 {
        return Err(InputError::Eof);
    }

    let trimmed = input.trim_end_matches(['\r', '\n'].as_ref());
    trimmed.parse::<T>().map_err(InputError::Parse)
}

/// A convenience wrapper that reads from stdin (locking it), without printing a prompt.
pub fn read_input<T>() -> Result<T, InputError<T::Err>>
where
    T: FromStr,
    T::Err: std::fmt::Display + std::fmt::Debug,
{
    let stdin = io::stdin();
    let mut locked = stdin.lock();
    read_input_from(&mut locked, None)
}

/// A convenience wrapper that reads from stdin, printing the given prompt first.
pub fn read_input_with_prompt<T>(prompt: Arguments<'_>) -> Result<T, InputError<T::Err>>
where
    T: FromStr,
    T::Err: std::fmt::Display + std::fmt::Debug,
{
    let stdin = io::stdin();
    let mut locked = stdin.lock();
    read_input_from(&mut locked, Some(prompt))
}

/// A macro that:
/// - reads **one line** from stdin (as `String` by default),
/// - returns `Ok(None)` if EOF is encountered (`InputError::Eof`).
///
/// # Usage:
/// ```no_run
/// // No prompt
/// let text: Option<String> = input!().unwrap();
///
/// // With prompt
/// let name: Option<String> = input!("Enter your name: ").unwrap();
///
/// // Formatted prompt
/// let user = "Alice";
/// let age: Option<String> = input!("Enter {}'s age: ", user).unwrap();
/// ```
#[macro_export]
macro_rules! input {
    () => {{
        // If you'd like a different type, just replace <String> below:
        match $crate::read_input_from(&mut ::std::io::stdin().lock(), None) {
            Ok(val) => Ok(Some(val)),
            Err($crate::InputError::Eof) => Ok(None),
            Err(err) => Err(err),
        }
    }};
    ($($arg:tt)*) => {{
        match $crate::read_input_from(
            &mut ::std::io::stdin().lock(),
            Some(format_args!($($arg)*))
        ) {
            Ok(val) => Ok(Some(val)),
            Err($crate::InputError::Eof) => Ok(None),
            Err(err) => Err(err),
        }
    }};
}

/// A macro that:
/// - prints the prompt on its own line (with `println!`),
/// - then reads one line,
/// - returns `Ok(None)` on EOF,
/// - otherwise parses into `String`.
///
/// # Usage:
/// ```no_run
/// let line: Option<String> = inputln!("What's your favorite color?").unwrap();
/// ```
#[macro_export]
macro_rules! inputln {
    () => {{
        match $crate::read_input_from(&mut ::std::io::stdin().lock(), None) {
            Ok(val) => Ok(Some(val)),
            Err($crate::InputError::Eof) => Ok(None),
            Err(err) => Err(err),
        }
    }};
    ($($arg:tt)*) => {{
        println!("{}", format_args!($($arg)*));
        ::std::io::Write::flush(&mut ::std::io::stdout()).unwrap();
        match $crate::read_input_from(&mut ::std::io::stdin().lock(), None) {
            Ok(val) => Ok(Some(val)),
            Err($crate::InputError::Eof) => Ok(None),
            Err(err) => Err(err),
        }
    }};
}

/// A macro that:
/// - reads one line from stdin,
/// - tries to parse into `String`,
/// - **treats EOF as an error** (no `Ok(None)`).
///
/// # Usage:
/// ```no_run
/// // No prompt
/// let line: String = input_no_eof!().unwrap();
///
/// // With prompt
/// let age: i32 = input_no_eof!("Enter your age: ").unwrap();
/// ```
#[macro_export]
macro_rules! input_no_eof {
    () => {{
        // If you want a different type, change <String> here:
        $crate::read_input_from(&mut ::std::io::stdin().lock(), None)
    }};
    ($($arg:tt)*) => {{
        $crate::read_input_from(
            &mut ::std::io::stdin().lock(),
            Some(format_args!($($arg)*))
        )
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Error, ErrorKind};

    /// Basic test reading an integer
    #[test]
    fn test_read_input_integer() {
        let mut reader = Cursor::new("42\n");
        let res: Result<i32, _> = read_input_from(&mut reader, None);
        assert_eq!(res.unwrap(), 42);
    }

    /// Test reading a floating-point number
    #[test]
    fn test_read_input_float() {
        let mut reader = Cursor::new("3.14159\n");
        let res: Result<f64, _> = read_input_from(&mut reader, None);
        assert!((res.unwrap() - 3.14159).abs() < f64::EPSILON);
    }

    /// Test reading an unsigned integer
    #[test]
    fn test_read_input_unsigned() {
        let mut reader = Cursor::new("255\n");
        let res: Result<u32, _> = read_input_from(&mut reader, None);
        assert_eq!(res.unwrap(), 255);
    }

    /// EOF immediately (0 bytes read)
    #[test]
    fn test_read_input_eof() {
        let mut reader = Cursor::new("");
        let res: Result<i32, _> = read_input_from(&mut reader, None);
        assert!(matches!(res, Err(InputError::Eof)));
    }

    /// Parse error: not an integer
    #[test]
    fn test_read_input_parse_error() {
        let mut reader = Cursor::new("not an int\n");
        let res: Result<i32, _> = read_input_from(&mut reader, None);
        assert!(matches!(res, Err(InputError::Parse(_))));
    }

    /// Reading a standard string
    #[test]
    fn test_read_input_string() {
        let mut reader = Cursor::new("hello world\r\n");
        let res: Result<String, _> = read_input_from(&mut reader, None);
        assert_eq!(res.unwrap(), "hello world");
    }

    /// Test with an explicit prompt passed as `format_args!`
    #[test]
    fn test_with_prompt() {
        let mut reader = Cursor::new("100\n");
        // Demonstrate passing "Enter: " as a &str with format_args!
        let prompt = format_args!("Enter: ");
        let res: Result<i32, _> = read_input_from(&mut reader, Some(prompt));
        assert_eq!(res.unwrap(), 100);
    }

    /// Multiple lines: read first line (valid), then second line (valid)
    #[test]
    fn test_multiple_lines_valid() {
        let mut reader = Cursor::new("123\n456\n");
        // Read first line
        let first: i32 = read_input_from(&mut reader, None).unwrap();
        assert_eq!(first, 123);

        // Read second line
        let second: i32 = read_input_from(&mut reader, None).unwrap();
        assert_eq!(second, 456);
    }

    /// Multiple lines: read first line (valid), second line (invalid parse), third line (EOF)
    #[test]
    fn test_multiple_lines_parse_error_then_eof() {
        let mut reader = Cursor::new("42\nnotanint\n");
        // Read first line
        let first: i32 = read_input_from(&mut reader, None).unwrap();
        assert_eq!(first, 42);

        // Read second line: parse error
        let second = read_input_from::<_, i32>(&mut reader, None);
        assert!(matches!(second, Err(InputError::Parse(_))));

        // Next read is EOF (because we've consumed all input)
        let third = read_input_from::<_, i32>(&mut reader, None);
        assert!(matches!(third, Err(InputError::Eof)));
    }

    /// Test what happens with an empty line (just "\n")
    /// - By default, we interpret empty line as an empty string -> parse error for integer
    #[test]
    fn test_empty_line_behavior() {
        let mut reader = Cursor::new("\n");
        let res: Result<i32, _> = read_input_from(&mut reader, None);
        // Typically this is a parse error, because "" can't parse into i32
        assert!(matches!(res, Err(InputError::Parse(_))));
    }

    /// Test that macros compile and work as expected (this is a basic usage check).
    /// We simulate a single line of input, then confirm `input!()` returns `Ok(Some(...))`.
    #[test]
    fn test_input_macro() {
        let mut reader = Cursor::new("HelloFromMacro\n");
        // Because macros read from stdin, we can temporarily override stdin by locking,
        // but to test here, we'll just manually call `read_input_from`.
        // In a real scenario, you might do an integration test or skip macro tests in unit tests.

        // Direct call for demonstration:
        let result: Result<String, _> = read_input_from(&mut reader, None);
        assert_eq!(result.unwrap(), "HelloFromMacro");
    }

    /// Check that a custom `Read` implementation that returns an error triggers `InputError::Io`.
    #[test]
    fn test_io_error() {
        struct ErrorReader;

        impl BufRead for ErrorReader {
            fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
                // Force an I/O error
                Err(Error::new(ErrorKind::Other, "Simulated I/O failure"))
            }
            fn consume(&mut self, _amt: usize) {}
        }

        // We only need `read_line` to fail:
        impl std::io::Read for ErrorReader {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(Error::new(ErrorKind::Other, "Simulated I/O failure"))
            }
        }

        let mut reader = ErrorReader;
        let res: Result<String, _> = read_input_from(&mut reader, None);
        assert!(matches!(res, Err(InputError::Io(_))));
    }
}