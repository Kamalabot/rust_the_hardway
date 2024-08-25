#[allow(dead_code)]

use std::io::{self, Read};
use std::error::Error;
use anyhow::Result;

use std::fmt;

// Define a custom error type using an enum.
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeLogarithm,
}

// Implement the `fmt::Display` trait for our custom error type.
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MathError::DivisionByZero => write!(f, "Attempted to divide by zero"),
            MathError::NegativeLogarithm => write!(f, "Logarithm of a negative number is undefined"),
        }
    }
}

// Implement the `std::error::Error` trait for our custom error type.
impl Error for MathError {}

// Define a function that can return our custom error.
fn divide_me(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

#[derive(Debug)]
enum MyError {
    DivisionByError
}

// implementing the Display Trait for the MyError
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MyError::DivisionByError => write!(f, "Division by zero Error."),
        }
    }
}

impl Error for MyError {}

fn divide(a: i32, b: i32) -> Result<i32, MyError> {
    // we have the custom error taken care
    if b == 0 {
        return Err(MyError::DivisionByError);
    }
    let result = a / b;

    Ok(result) 
}

use std::fs::File;

// I need an error that is thrown by Rust itself 
fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    // attempting to open file
    // Box<dyn Error>, a function can return different types of errors that all
    // implement the Error trait. This is useful in functions that might 
    // produce different kinds of errors but still want to return a single Result type.
    // Flexibility, Heterogeneous Types, Size Efficiency, and Error Composition
    let mut file = File::open(filename)?; 

    // creating a holding string
    let mut content = String::new();

    // Read contents to string, again its an attempt
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn read_file_bdyn_explainer (filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)
        .map_err(|e| format!("Failed to open file: {}", e))?; // Propagate any `io::Error` as a `Box<dyn Error>`.
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?; // Same here for read error.

    Ok(contents)
}

fn read_file_ah(filename: &str) -> Result<String> {
    // attempting to open file
    let mut file = File::open(filename)?; 

    // creating a holding string
    let mut content = String::new();

    // Read contents to string, again its an attempt
    file.read_to_string(&mut content)?;

    Ok(content)
}


fn main() -> Result<()>{
    // panic!("This is super awesome..."); 
    let newfile = "sl.txt";

    let output = File::create(newfile);

    let output = match output {
        Ok(file) => file,
        Err(e) => {
            panic!("This is the error:{:?}", e);
        }
    };

    println!("This is the file: {:?}", output);
    
    println!("Now lets look at some function that throws the errors");
    
    match divide(10, 2) {
        Ok(val) => println!("The division result: {}", val),
        Err(e) => println!("There goes... {}", e)
    }

    match divide(5, 0) {
        Ok(_val) => println!("This is failing..."),
        Err(e) => println!("Failure: {}", e)
    }

    // reading an existing file
    
    match read_file("sl.txt") {
        Ok(data) => println!("data: {data:?}"),
        Err(e) => println!("Error: {}", e)
    }

    match read_file("slt.txt") {
        Ok(data) => println!("data: {data:?}"),
        Err(e) => println!("Error: {}", e)
    }
    
    let file_data = read_file_ah("sl.txt")?;

    println!("Got the out put through anyhow: {}", file_data);

    Ok(())
}










