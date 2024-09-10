use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::{error::Error, fmt};

pub fn open_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content);
    Ok(content)
}
// Error Type creation
#[derive(Debug)]
pub struct InvalidInputError {
    details: String,
}
// Implementing supporting functions
impl InvalidInputError {
    fn new(msg: &str) -> InvalidInputError {
        InvalidInputError {
            details: msg.to_string(),
        }
        // Error is created with a new name
    }
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Input: {}", self.details)
    }
}

impl Error for InvalidInputError {}
// the above will implement a source fun
// that helps in error chaining

pub fn validate_number(input: &str) -> Result<i32, InvalidInputError> {
    match input.parse::<i32>() {
        Ok(nym) if nym > 0 => Ok(nym),
        _ => Err(InvalidInputError::new("give positive nums.")),
    }
}

#[cfg(feature = "reqwest")]
pub async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[cfg(feature = "serde_json")]
pub fn parse_json(input: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(input)
}

// Wrapping errors for more context
pub fn open_file_with_context(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to open {}: {}", file_path, e)))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read {}: {}", file_path, e)))?;

    Ok(contents)
}

//let _ = File::create("error.log")
//.and_then(|mut f| f.write_all(error.as_bytes()));

// Iterators Error handling
// inputs.into_iter().map(|s| s.parse::<i32>())
//  .collect()

pub fn log_error(error: &str) {
    let _ = File::create("error.log").and_then(|mut f| f.write_all(error.as_bytes()));
}

// Using unwrap_or_else for custom error handling
pub fn safe_divide(a: i32, b: i32) -> i32 {
    // panic if dividend is 0
    if b == 0 {
        panic!("Division by zero is not allowed!");
    }
    a / b
}

// Working with Result and ? operator in functions that return Option
pub fn maybe_parse_number(input: &str) -> Option<Result<i32, ParseIntError>> {
    // return Option of a result
    Some(input.parse::<i32>())
}

// Returning custom error types from functions
pub fn parse_and_validate(input: &str) -> Result<i32, InvalidInputError> {
    let number = input
        .parse::<i32>()
        .map_err(|_| InvalidInputError::new("Parse error"))?;
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
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub fn check_input(input: &str) -> Result<(), MyError> {
    if input.is_empty() {
        Err(MyError::InvalidInput("Input cannot be empty".to_string())) // Custom InvalidInput error
    } else if input == "invalid" {
        Err(MyError::InvalidInput(
            "Input cannot be 'invalid'".to_string(),
        )) // Custom invalid case
    } else {
        Ok(())
    }
}

// Function to open and read a file
pub fn read_file(file_path: &str) -> Result<String, MyError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Function to parse a string to an integer
pub fn parse_number(input: &str) -> Result<i32, MyError> {
    let num: i32 = input.parse()?;
    Ok(num)
}
