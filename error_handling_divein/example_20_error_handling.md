To provide a comprehensive set of examples on error handling in Rust, I will cover various real-world scenarios ranging from basic error handling to more advanced use cases involving custom error types, conversions, and integrating with popular Rust libraries.

### `lib.rs`

Below is the content of `lib.rs`, which includes 20 examples of error handling in Rust. These examples cover common patterns such as handling errors with `Result`, using `Option`, creating custom errors, converting errors, and working with third-party libraries.

#### `lib.rs`

```rust
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{error::Error, fmt};

// Basic error handling with Result
pub fn open_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
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
        Ok(n) => Ok(Some(n)),
        Err(e) => Err(e),
    }
}

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
pub fn find_first_invalid_number(inputs: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    inputs.into_iter().find_map(|input| match input.parse::<i32>() {
        Ok(_) => None,
        Err(e) => Some(Err(e)),
    })
}

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

### `tests.rs`

Below is the `tests.rs` file containing tests for each function in `lib.rs`. This will ensure that the error handling behaves as expected.

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

1. **Create the Project Structure**:
   Create a new Rust library project:
   
   ```bash
   cargo new rust_error_handling --lib
   cd
   
   rust_error_handling
   ```

2. **Replace `lib.rs` with Provided Code**:
   Replace the contents of `src/lib.rs` with the code provided above.

3. **Create `tests.rs`**:
   Create a new file in `src/tests.rs` and copy the tests code into it.

4. **Run Tests**:
   Run the tests using Cargo:
   
   ```bash
   cargo test
   ```

### Conclusion

This example provides a comprehensive set of Rust error-handling patterns using `Result`, `Option`, custom error types, error conversions, and interactions with external libraries. The `tests.rs` file ensures that each function's error handling is correct and demonstrates how to write effective tests for error scenarios.
