use std::io::{self, Write};
use std::str::FromStr;
use std::fmt::{self, Debug, Display, Formatter};
use std::error::Error;

/// Custom error type for prompt function.
#[derive(Debug)]
pub enum PromptError<T: FromStr> 
where
    T::Err: Debug + Display,
{
    Io(io::Error),
    ParseError(<T as FromStr>::Err),
}

impl<T: FromStr> Display for PromptError<T>
where
    T::Err: Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PromptError::Io(e) => write!(f, "IO error: {}", e),
            PromptError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl<T: FromStr + Debug> Error for PromptError<T> 
where
    T::Err: Debug + Display,
{
}

impl<T: FromStr> From<io::Error> for PromptError<T>
where
    T::Err: Debug + Display,
{
    fn from(e: io::Error) -> Self {
        PromptError::Io(e)
    }
}

/// A function to prompt for prompt and read a line from stdin, parsing it into a specified type.
///
/// # Arguments
///
/// * `prompt` - A prompt to display before reading prompt.
///
/// # Errors
///
/// This function will return an error if there's an issue with reading from stdin
/// or parsing the prompt into the desired type.
pub fn prompt<T: FromStr>(prompt: &str) -> Result<T, PromptError<T>>
where
    T::Err: Debug + Display,
{
    io::stdout().write_all(prompt.as_bytes())?; // `print!` may `panic!`
    io::stdout().flush()?; // Always flush after the prompt

    let mut prompt = String::new();
    match io::stdin().read_line(&mut prompt) {
        Ok(0) => Err(PromptError::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"))),
        Ok(_) => prompt.trim().parse::<T>().map_err(PromptError::ParseError),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_int() {
        let i: i32 = prompt("").unwrap();
        println!("{}", i);
        let j: f32 = prompt("").unwrap();
        println!("{}", j);

    }

    #[test]
    fn get_int_2() {
        let i: i32 = prompt("Input: ").unwrap();
        print!("Output: {}", i);
    }

    #[test]
    fn hacker_rank() {
        let s: String = prompt("").unwrap();
        println!("hello, {}", s);
    }

}
