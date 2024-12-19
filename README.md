# input_macro

A simple and safe input macro for Rust, inspired by Python's `input()`.

## Motivation

Rust's standard library currently lacks a simple way to read user input and parse it into different types, similar to Python's `input()`. This can make simple programs and examples more complex than necessary, especially for beginners. This library aims to fill that gap by providing a user-friendly macro.

## Features

-   **Easy to use:** The `input!` macro provides a simple interface for reading user input.
-   **Type inference:** The macro automatically infers the desired type based on the variable it's assigned to.
-   **Error handling:** It handles both I/O errors and parsing errors gracefully, returning a `Result`.
-   **Optional prompt:** You can provide an optional prompt string that will be printed before reading input.
-   **Flushing:** The prompt is automatically flushed to ensure it's visible to the user before input is read.
-   **Safety:** The macro is designed to be safe and prevent common errors like buffer overflows.
-   **Performance:** It's built on top of Rust's efficient I/O and string handling.
-   **EOF Handling:** Returns `Ok(None)` on EOF, allowing for graceful handling of end-of-file conditions.

## Usage Examples

```rust
use input_macro::input;

fn main() {
    // Read a line of text into a String
    let name: String = input!("Enter your name: ").unwrap();
    println!("Hello, {}!", name);

    // Read an integer
    let age: i32 = input!("Enter your age: ").unwrap();
    println!("You are {} years old.", age);

    // Read a floating-point number
    let price: f64 = input!().unwrap(); // No prompt
    println!("The price is {}.", price);

    // Handle potential errors
    let num: Result<i32, _> = input!();
    match num {
        Ok(n) => println!("You entered: {}", n),
        Err(e) => eprintln!("Invalid input: {}", e),
    }

    // Example of handling EOF
    loop {
        let line: Option<String> = input!().unwrap();
        match line {
            Some(value) => println!("You entered: {}", value),
            None => {
                println!("End of input reached.");
                break;
            }
        }
    }
}
```