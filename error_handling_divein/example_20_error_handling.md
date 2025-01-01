To provide a comprehensive set of examples on
error handling in Rust, I will cover various
real-world scenarios ranging from basic error
handling to more advanced use cases involving
custom error types, conversions, and integrating
with popular Rust libraries.

### `lib.rs`

Below is the content of `lib.rs`, which includes
20 examples of error handling in Rust. These
examples cover common patterns such as handling
errors with `Result`, using `Option`, creating
custom errors, converting errors, and working with
third-party libraries.

#### `lib.rs`

```rust
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{error::Error, fmt};

// Basic error handling with Result
pub fn open_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?; // unable to find the file or open it
    let mut content = String::new();
    file.read_to_string(&mut content)?; // unable to read the content
    Ok(content)
}

// Handling Option with Result
pub fn parse_optional_number(input: Option<&str>) -> Result<i32, ParseIntError> {
    input.unwrap_or("0").parse::<i32>()
}

// Chaining multiple errors
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Custom error types with Display and Error traits
#[derive(Debug)]
pub struct InvalidInputError {
    details: String,
}

impl InvalidInputError {
    fn new(msg: &str) -> InvalidInputError {
        InvalidInputError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Input: {}", self.details)
    }
}

impl Error for InvalidInputError {}
// above line enables the source(), which is
// already implemented, so you wont require to wrte
// this supports error chaining
fn source(&self) -> Option<&(dyn Error + 'static)> {
        // In this case, there is no underlying error, so we return None
        None
    }
}

pub fn validate_number(input: &str) -> Result<i32, InvalidInputError> {
    match input.parse::<i32>() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err(InvalidInputError::new("Only positive numbers are allowed")),
    }
}

// Using the `?` operator with custom errors
pub fn double_positive_number(input: &str) -> Result<i32, InvalidInputError> {
    let number = validate_number(input)?;
    Ok(number * 2)
}

// Combining different error types using `Box<dyn Error>`
pub fn parse_and_double(input: &str) -> Result<i32, Box<dyn Error>> {
    let number = input.parse::<i32>()?;
    if number < 0 {
        return Err(Box::new(InvalidInputError::new("Negative number")));
    }
    Ok(number * 2)
}

// Using Option for a possible None value
pub fn find_word(input: &str, word: &str) -> Option<usize> {
    input.find(word)
}

// Error handling with external libraries (reqwest example)
// Below cfg(feature="reqwest") tell to compiler it an optional dep
// reqwest = { version = "0.11", optional = true }: This makes the reqwest crate an optional dependency.
// The [features] section defines a feature named reqwest that enables the reqwest dependency.
#[cfg(feature = "reqwest")]
pub async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

// Handling JSON parsing errors (serde_json example)
#[cfg(feature = "serde_json")]
pub fn parse_json(input: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(input)
}

// Using Result with Option to avoid nested Result<Option<T>>
pub fn parse_number_and_check(input: &str) -> Result<Option<i32>, ParseIntError> {
    match input.parse::<i32>() {
        Ok(n) => Ok(Some(n)), // why do this? where will it be used
        Err(e) => Err(e),
    }
}
// Where it could be used:
//This pattern is useful in cases where you
// might want to:
// Distinguish between three possible states:
// Successful parsing (Ok(Some(n)))
// Empty/optional input (which could return Ok(None) if implemented)
// Failed parsing (Err(e))

// Wrapping errors for more context
pub fn open_file_with_context(file_path: &str) -> Result<String, io::Error> {
    File::open(file_path).map_err(|e| io::Error::new(e.kind(), format!("Failed to open {}: {}", file_path, e)))
}

// Ignoring errors
pub fn log_error_to_file(error: &str) {
    let _ = File::create("error.log").and_then(|mut f| f.write_all(error.as_bytes()));
}

// Error handling in iterators
pub fn parse_numbers(inputs: Vec<&str>) -> Vec<Result<i32, ParseIntError>> {
    inputs.into_iter().map(|s| s.parse::<i32>()).collect()
}

// Using unwrap_or_else for custom error handling
pub fn safe_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero is not allowed!");
    }
    a / b
}

// Working with Result and ? operator in functions that return Option
pub fn maybe_parse_number(input: &str) -> Option<Result<i32, ParseIntError>> {
    Some(input.parse::<i32>())
}

// Handling errors with an iterator that returns Option

// Returning custom error types from functions
pub fn parse_and_validate(input: &str) -> Result<i32, InvalidInputError> {
    let number = input.parse::<i32>().map_err(|_| InvalidInputError::new("Parse error"))?;
    if number < 0 {
        return Err(InvalidInputError::new("Negative number"));
    }
    Ok(number)
}

// Using unwrap_or for Option types
pub fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}

// Propagating errors from lower-level functions
pub fn read_file_and_double_numbers(file_path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let content = open_file(file_path)?;
    content
        .lines()
        .map(|line| line.parse::<i32>().map_err(|e| e.into()))
        .collect()
}

// Example of combining different error types using `thiserror` (optional)
#[cfg(feature = "thiserror")]
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}
```

The `MyError` enum is designed to combine
different types of errors (`io::Error` and
`std::num::ParseIntError`) with custom error
messages using the `thiserror` crate. This allows
you to propagate and handle different types of
errors in a unified way while adding custom error
messages.

Let’s provide an example where you encounter and
handle all three error types (`InvalidInput`,
`Io`, and `ParseIntError`). This example will
showcase how these errors can be raised and caught
using the `MyError` type.

### Example Code Using `MyError`:

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use thiserror::Error;  // Make sure to include thiserror in your dependencies.

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

// Function to simulate an invalid input error
fn check_input(input: &str) -> Result<(), MyError> {
    if input.is_empty() {
        Err(MyError::InvalidInput("Input cannot be empty".to_string()))
    } else {
        Ok(())
    }
}

// Function to open and read a file
fn read_file(file_path: &str) -> Result<String, MyError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Function to parse a string to an integer
fn parse_number(input: &str) -> Result<i32, MyError> {
    let num: i32 = input.parse()?;
    Ok(num)
}

fn main() -> Result<(), MyError> {
    // Example 1: Invalid input
    match check_input("") {
        Ok(_) => println!("Input is valid."),
        Err(e) => eprintln!("Error: {}", e), // Expecting InvalidInput error here
    }

    // Example 2: IO error (file not found)
    match read_file("non_existent_file.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error: {}", e), // Expecting Io error here
    }

    // Example 3: Parsing error (invalid integer)
    match parse_number("abc") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => eprintln!("Error: {}", e), // Expecting ParseInt error here
    }

    Ok(())
}
```

### Explanation:

1. **`MyError` Enum:**

   - Combines three error types: `InvalidInput`,
     `Io` (from `std::io`), and `ParseIntError`
     (from parsing strings into integers).
   - The `thiserror::Error` derive macro
     automatically implements the `Display` and
     `Error` traits for the `MyError` enum.

2. **Error Variants:**

   - **`InvalidInput(String)`**: A custom error
     for invalid input, in this case, when an
     empty string is provided.
   - **`Io(#[from] io::Error)`**: Automatically
     converts `io::Error` to `MyError` when it
     occurs (e.g., when trying to open a
     non-existent file).
   - **`ParseInt(#[from] ParseIntError)`**:
     Automatically converts `ParseIntError` into
     `MyError` when parsing a string to an integer
     fails.

3. **`check_input` Function**:

   - This function simulates an "invalid input"
     error by returning `MyError::InvalidInput` if
     the input is an empty string.

4. **`read_file` Function**:

   - This function opens a file and reads its
     contents, potentially causing an `io::Error`
     if the file doesn’t exist or if there’s a
     reading issue.

5. **`parse_number` Function**:

   - This function attempts to parse a string into
     an integer, which may result in a
     `ParseIntError`.

6. **Main Program:**

   - The `main` function demonstrates how each of
     the three error types is handled.
   - Each error variant is raised in different
     scenarios:
     - **`InvalidInput`** when an empty string is
       provided.
     - **`Io`** when trying to open a non-existent
       file.
     - **`ParseIntError`** when attempting to
       parse an invalid integer string.

### Output Example:

```plaintext
Error: Invalid input: Input cannot be empty
Error: Failed to open non_existent_file.txt: No such file or directory (os error 2)
Error: invalid digit found in string
```

### Usage Notes:

- **Automatic Conversion with `#[from]`**: The
  `#[from]` attribute allows Rust to automatically
  convert errors from the underlying types
  (`io::Error` and `ParseIntError`) into your
  custom `MyError` type.
- **Unified Error Handling**: Using a unified
  error type like `MyError` makes it easier to
  handle different error cases in a consistent
  way, and provides meaningful error messages with
  additional context when needed.

### `tests.rs`

Below is the `tests.rs` file containing tests for
each function in `lib.rs`. This will ensure that
the error handling behaves as expected.

#### `tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file() {
        let result = open_file("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_optional_number() {
        assert_eq!(parse_optional_number(Some("42")).unwrap(), 42);
        assert_eq!(parse_optional_number(None).unwrap(), 0);
    }

    #[test]
    fn test_validate_number() {
        assert!(validate_number("10").is_ok());
        assert!(validate_number("-10").is_err());
    }

    #[test]
    fn test_double_positive_number() {
        assert_eq!(double_positive_number("10").unwrap(), 20);
        assert!(double_positive_number("-10").is_err());
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("10").unwrap(), 20);
        assert!(parse_and_double("-10").is_err());
    }

    #[test]
    fn test_find_word() {
        assert_eq!(find_word("hello world", "world"), Some(6));
        assert_eq!(find_word("hello world", "rust"), None);
    }

    #[test]
    fn test_parse_number_and_check() {
        assert_eq!(parse_number_and_check("42").unwrap(), Some(42));
        assert!(parse_number_and_check("abc").is_err());
    }

    #[test]
    fn test_open_file_with_context() {
        let result = open_file_with_context("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_numbers() {
        let inputs = vec!["1", "2", "abc"];
        let results = parse_numbers(inputs);
        assert_eq!(results[0], Ok(1));
        assert!(results[2].is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "Division by zero is not allowed!")]
    fn test_safe_divide_panic() {
        safe_divide(10, 0);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn test_read_file_and_double_numbers() {
        let result = read_file_and_double_numbers("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_and_validate() {
        assert!(parse_and_validate("42").is_ok());
        assert!(parse_and_validate("abc").is_err());
        assert!(parse_and_validate("-1").is_err());
    }
}
```

### Instructions for Running the Tests

1. **Create the Project Structure**: Create a new
   Rust library project:

   ```bash
   cargo new rust_error_handling --lib
   cd

   rust_error_handling
   ```

2. **Replace `lib.rs` with Provided Code**:
   Replace the contents of `src/lib.rs` with the
   code provided above.

3. **Create `tests.rs`**: Create a new file in
   `src/tests.rs` and copy the tests code into it.

4. **Run Tests**: Run the tests using Cargo:

   ```bash
   cargo test
   ```

### Conclusion

This example provides a comprehensive set of Rust
error-handling patterns using `Result`, `Option`,
custom error types, error conversions, and
interactions with external libraries. The
`tests.rs` file ensures that each function's error
handling is correct and demonstrates how to write
effective tests for error scenarios.
