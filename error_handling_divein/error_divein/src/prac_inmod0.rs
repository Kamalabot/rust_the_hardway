// contains important error handling and
// throwing tricks practice
// map_err for file read and open
//
// create InvalidError struct with details
// implement new, Display, Error traits
//
// use parse() with result output to return
// Ok(res) or Err(InvalidError) type
//
// write a log_error function, that creates a error.log
// file
//
// Write a DoubleVariant Enum that takes care ParseIntError
// Io::error

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read, Write};
// write is required for writing data to file
use std::num::ParseIntError;

pub fn log_error(err: &str) {
    File::create("error.log").and_then(|mut f| f.write_all(err.as_bytes()));
}
#[derive(Debug)]
pub struct InvalidError {
    details: String,
}

impl InvalidError {
    fn new(msg: &str) -> Self {
        InvalidError {
            details: msg.to_owned(),
        }
    }
}

impl Display for InvalidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Error: {}", self.details)
    }
}

impl Error for InvalidError {}

pub fn pmod0() -> Result<String, io::Error> {
    let path = "numfile.txt";
    let mut raw = File::open(path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to open {} due to {}", path, e)))?;
    let mut content = String::new();
    raw.read_to_string(&mut content)
        .map_err(|e| io::Error::new(e.kind(), format!("Unable to read {} due to {}", path, e)))?;
    Ok(content)
}

pub fn valid_num_parse(ipn: &str) -> Result<i32, InvalidError> {
    match ipn.parse::<i32>() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err(InvalidError::new("only positive")),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    // observe this below sqr bracket
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub fn read_parse_error(path: &str) -> Result<i32, MyError> {
    // errors will automatically converted... Neat
    let mut file = File::open(path)?;
    let mut cont = String::new();
    file.read_to_string(&mut cont);
    let sum: i32 = cont
        .lines()
        .map(|l| l.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    Ok(sum)
}

#[derive(Debug)]
pub enum DoubleError {
    IoError(std::io::Error),
    ParseError(ParseIntError),
}

impl From<io::Error> for DoubleError {
    fn from(value: io::Error) -> Self {
        DoubleError::IoError(value)
    }
}

impl From<ParseIntError> for DoubleError {
    fn from(value: ParseIntError) -> Self {
        DoubleError::ParseError(value)
    }
}

fn read_double_error(file_path) -> Result<i32, DoubleError>{
    // no need to use Box<dyn Error>
    let mut file = File::open(file_path)?;
    let mut cont = String::new();
    file.read_to_string(&mut cont)?;
    let num: i32 = cont.trim().parse()?;
    Ok(num)
}
