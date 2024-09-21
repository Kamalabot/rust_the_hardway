#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(warnings)]

use core::num;
use std::fmt::{self, Display};
use std::io::stdin;

// practice sending &str, string and customerror

#[derive(Debug)]
struct CustomError;

#[derive(Debug)]
struct AnotherError;

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Erroring out")
    }
}

fn is_greater(x: i32) -> Result<i32, CustomError> {
    if x < 5 {
        Ok(x)
    } else {
        Err(CustomError)
    }
}

fn is_greater_er(x: i32) -> Result<i32, String> {
    if x < 5 {
        Ok(x)
    } else {
        Err(format!("{:?}", AnotherError))
    }
}
fn cliargs() {
    let mut argv = std::env::args();
    // let arg1 = argv.nth(0).unwrap();
    let arg1 = argv.next().unwrap();
    let arg2 = argv.next().unwrap();
    println!("arg1 {} and arg2 {}", arg1, arg2)
}

fn cliargs_error() -> Result<String, String> {
    let mut argv = std::env::args();
    let _arg1 = argv.next();
    // idea is to throw error if no 2nd arg
    let arg2 = argv.next();
    // let mut argvec: Vec<String> = Vec::new();
    match arg2 {
        Some(arg) => Ok(format!("pushed to vec {}", arg)),
        None => Err("No args".to_string()),
    }
}

fn cliargs_str() -> Result<String, &'static str> {
    let mut argv = std::env::args();
    let _arg1 = argv.next();
    // idea is to throw error if no 2nd arg
    let arg2 = argv.next();
    // let mut argvec: Vec<String> = Vec::new();
    match arg2 {
        Some(arg) => Ok(format!("pushed to vec {}", arg)),
        None => Err("No args"),
    }
}
use std::error::Error;
fn cli_i32() -> Result<String, CustomError> {
    let mut argv = std::env::args();
    let _arg1 = argv.next();
    let arg2 = argv.next();
    match arg2 {
        Some(val) => Ok(val),
        None => Err(CustomError),
    }
}

fn parse_str(num: String) -> Option<i32> {
    let intnum = num.parse::<i32>();
    // let intnum = num.parse::<f32>();
    // when  calling the parse, the error type shows up
    // above gives me a result
    intnum.ok()
}
use std::fs::File;
use std::io::Read;
use std::path::Path;

// P implements AsRef trait and fn is ret i32
fn file_reader<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let n: i32 = content.trim().parse().unwrap();
    n
}

fn file_reterr<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents
                .trim()
                .parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| n * 2)
    //
}

fn ret_enum_error<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut raw = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
        // why adding return made the analyzer to
        // accept seemingly incompatible return types
    };
    let mut contents = String::new();
    if let Err(err) = raw.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    // The ? operator does the same thing as the
    // match with return—it propagates the error
    // if there is one. It's essentially syntactic
    // sugar to make error handling less verbose.
    Ok(2 * n)
    // below wont worck
    // let n = match contents.trim().parse::<i32>() {
    //     Ok(n) => n,
    //     Err(e) => Err(e.to_string()),
    // };
}
fn easier_file<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut raw = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
        // why adding return made the analyzer to
        // accept seemingly incompatible return types
    };
    let mut contents = String::new();
    if let Err(err) = raw.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    // The ? operator does the same thing as the
    // match with return—it propagates the error
    // if there is one. It's essentially syntactic
    // sugar to make error handling less verbose.
    Ok(2 * n)
    // below wont worck
    // let n = match contents.trim().parse::<i32>() {
    //     Ok(n) => n,
    //     Err(e) => Err(e.to_string()),
    // };
}
// return inside match: The return statement inside the
// Err(err) arm does not raise an error because it
// is exiting the function early and returning an error
// result (Err). This is typical in Rust when handling
// errors in functions that return a Result.
// The Err(err.to_string()) converts the error to a string and propagates it up the call stack.
//
// Basically this help to convert and return error
#[derive(Debug)]
enum CliError {
    IoError(std::io::Error),
    ParseError(num::ParseIntError),
}
use std::io;

// implementing from method to Custom CliError
// so it can deliver io::Error
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IoError(err)
    }
}
impl From<std::num::ParseIntError> for CliError {
    fn from(err: std::num::ParseIntError) -> CliError {
        CliError::ParseError(err)
    }
}

fn read_number_from_file(file_path: &str) -> Result<i32, CliError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}
// The map_err method is used to transform one type of
// error into another. In this case, CliError is an
// enum, and the function uses map_err to convert
// different kinds of standard errors (e.g.,
// std::io::Error, std::num::ParseIntError) into the
// corresponding CliError variants.

//Enum Variants as Functions: In Rust, enum variants that take
//arguments are essentially functions that can be used to
//wrap values of their argument types. So CliError::IoError is
//a function that takes a std::io::Error and returns a
//CliError::IoError(std::io::Error). Similarly, CliError::ParseError
//works for std::num::ParseIntError.

fn main() {
    println!("Hello, world!");
    // let mut input = String::new();
    // println!("Give one input...");
    // stdin().read_line(&mut input).unwrap();
    // println!("Input is: {}", input);
    println!("Extracting the cli arguments");
    cliargs();
    let secarg = cliargs_error().unwrap();
    // let okarg = cliargs_error();
    println!("Looking at 2nd arg: {}", secarg);
    // converting the recieved Result to option
    // and print it
    let secarg1 = cli_i32().ok();
    println!("Again looking for 2nd arg: {:?}", secarg1);
    // let arg1val = secarg1.as_ref().map(|x| x);
    // println!("Arg1Val: {:?}", arg1val);
    println!("parse the cli if it is a number");
    let numarg1 = parse_str(secarg1.unwrap()).take();

    println!("The numarg1 is {:?}", numarg1);
    let _temp = cli_i32().unwrap();
    let temp = cli_i32();
    println!("Custom Error: {:?}", temp);
    // let isgrt = is_greater(numarg1.unwrap()).ok();
    // Above converts the result to option, where the error is converted to None
    // let isgrt = is_greater(numarg1.unwrap()).unwrap();
    //
    let isgrt = is_greater(numarg1.unwrap()).map_err(|_| "Got error");

    println!("Trying to use the customError: {:?}", isgrt);
    // this file has to be on the workspace root
    let readnum = file_reader("numfile.txt");

    println!("read num is: {}", readnum);
    let x = is_greater_er(6);
    println!("Got x: {:?}", x)
}
