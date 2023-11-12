use std::io::{self, Write};
use std::str::FromStr;
use std::fmt::{self, Debug, Display, Formatter};
use std::error::Error;

/// Custom error type for input function.
#[derive(Debug)]
pub enum InputError {
    IoError(io::Error),
    ParseError(String),
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InputError::IoError(e) => write!(f, "IO error: {}", e),
            InputError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl Error for InputError {}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> Self {
        InputError::IoError(e)
    }
}

/// A function similar to Python's `input`, allowing for a prompt and reading a line from stdin.
/// 
/// # Arguments
///
/// * `prompt` - An optional prompt to display before reading input.
/// * `flush` - Whether to flush stdout after displaying the prompt.
/// 
/// # Errors
///
/// This function will return an error if there's an issue with reading from stdin
/// or parsing the input into the desired type.
pub fn input<T: FromStr>(prompt: Option<&str>, flush: bool) -> Result<T, InputError>
where
    T::Err: Display {
    if let Some(p) = prompt {
        print!("{}", p);
        if flush {
            io::stdout().flush()?;
        }
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => Err(InputError::IoError(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"))),
        Ok(_) => input.trim().parse::<T>().map_err(|e| InputError::ParseError(e.to_string())),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_int() {
        let i: i32 = input(None, false).unwrap();
        println!("{}", i);
        let j: i32 = input(None, true).unwrap();
        println!("{}", j);

    }

    #[test]
    fn get_int_2() {
        let i: i32 = input(Some("Input: "), false).unwrap();
        print!("Output: {}", i);
    }

    #[test]
    fn hacker_rank() {
        let s: String = input(None, true).unwrap();
        println!("hello, {}", s);
    }

}
