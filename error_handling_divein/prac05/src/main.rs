#![allow(warnings)]
#![allow(unused_imports)]

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::panic::catch_unwind;
use std::path::Path;

fn main() {
    let _ = file_handle("numfile.txt");
    let x1 = send_option(5);
    println!("{:?}", x1);
    // there is no issue if trying print the option
    let _ = catch_unwind(|| {
        let x1val = x1.unwrap();
        // println!("caught unwrap on None");
        // println!("{}", x1val);
    });
    // this will panic as the x1 value is None let x2 = send_error(5);
    let x2 = send_error(6);
    println!("{:?}", x2);
    match x2 {
        Ok(v) => println!("Got i32: {}", v),
        Err(e) => println!("Error is: {}", e),
    }

    let x3 = send_error(2);
    match x3 {
        Ok(v) => println!("Got i32: {}", v),
        Err(e) => println!("Error is: {}", e),
    }
    let x4 = send_custerr(2);
    println!("Error is {:?}", x4);
    let _ = x4
        .map(|x| println!("value is {}", x))
        .map_err(|e| println!("error is {}", e));
    // unwrap will panic only when Error is hit
    let file_res = file_handle_er("numfile.txt");
    // println!("Got the numfile in main: {}", &file_res.unwrap());
    file_res
        .map(|op| println!("{}", op))
        .map_err(|e| println!("{}", e));
    // doesn't matter what error is incoming, it will get handled
    let file_res0 = file_handle_er("numfile1.txt");
    file_res0
        .map(|op| println!("{}", op))
        .map_err(|e| println!("{}", e));
}
// work on the file opening directly
// work on returning option
// work on catch_unwind for option unwrap
// work on returning results
// work on custom error returning
// work on map and map_err
// work on file opening that returns dyn error
// need to work on composite error
use std::io;

#[derive(Debug)]
enum cliError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

impl From<io::Error> for cliError {
    fn from(value: io::Error) -> Self {
        cliError::IoError(value)
    }
}

impl From<std::num::ParseIntError> for cliError {
    fn from(value: std::num::ParseIntError) -> Self {
        cliError::ParseError(value)
    }
}

#[derive(Debug)]
struct customError;

impl Display for customError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error is CustomMade")
    }
}

impl Error for customError {}

fn file_handle<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    let mut raw = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut data = String::new();
    match raw.read_to_string(&mut data) {
        Ok(i) => println!("Contents are: {}", data),
        Err(e) => println!("Error is {:?}", e),
    }
    Ok(())
}

fn file_handle_er<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut raw = File::open(path)?;
    let mut data = String::new();
    match raw.read_to_string(&mut data) {
        Ok(i) => println!("Contents are: {}", data),
        Err(e) => println!("Error is {:?}", e),
    }
    Ok(data)
}
fn send_option(x: i32) -> Option<i32> {
    if x > 5 {
        Some(5)
    } else {
        None
    }
}

fn send_error(x: i32) -> Result<i32, String> {
    if x > 5 {
        Ok(5)
    } else {
        Err("It less...".to_string())
    }
}

fn send_custerr(x: i32) -> Result<i32, customError> {
    if x > 5 {
        Ok(5)
    } else {
        Err(customError)
    }
}

fn explore_clierr<P: AsRef<Path>>(path: P) -> Result<(), cliError> {
    let mut raw = File::open(path)
        
    Ok(())
}
