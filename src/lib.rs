use std::io::{self, Write};
use std::str::FromStr;

/// Custom error type for input parsing.
pub enum InputError {
    IoError(io::Error),
    ParseError(String),
}

/// Function to get input and parse it into the desired type.
pub fn parse_input<T: FromStr>(prompt: &str) -> Result<T, InputError>
where
    <T as FromStr>::Err: ToString,
{
    print!("{}", prompt);
    io::stdout().flush().map_err(InputError::IoError)?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(InputError::IoError)?;

    input.trim().parse::<T>().map_err(|e| InputError::ParseError(e.to_string()))
}

/// Macro to simplify calling the parse_input function.
#[macro_export]
macro_rules! parse_input {
    ($prompt:expr) => {
        $crate::parse_input::<_>($prompt)
    };
}

#[cfg(test)]
mod tests {
    // Example test for successful parsing
    #[test]
    fn test_parse_input_success() {
        let result = "42".parse::<f64>();
        assert_eq!(result, Ok(42.));
    }

    // Example test for failed parsing
    #[test]
    fn test_parse_input_failure() {
        let result = "abc".parse::<i32>();
        assert!(result.is_err());
    }

}