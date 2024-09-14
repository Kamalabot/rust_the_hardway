// using this for practice
//
#![allow(unused_imports)]
#![allow(warnings)]

use core::fmt;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};
use std::num::{self, ParseIntError};
use std::panic;
use std::path::Path;
// Objective of all these Error Types is that we
// have to avoid panics in production code &
// handle the error that are thrown...
fn main() {
    println!("Hello, world!");

    let io_error: io::Error = io::Error::last_os_error();
    let parse_error: num::ParseIntError = "not number".parse::<i32>().unwrap_err();
    // the above is printing the raw error struct

    println!("Io Error: {:?}", io_error);
    println!("Parse Error: {:?}", parse_error);
    // gathering some knowledge
    println!("Display Parse Int Error: {}", parse_error);

    println!("Stringing it IO Error: {}", io_error.to_string());
    println!("Stringing it Int Error: {}", parse_error.to_string());

    let er1: Box<dyn Error> = Box::from(io_error);
    let er1: Box<dyn Error> = Box::from(parse_error);
    // we have moved the error to the heap, but what will
    // I do with it?
    // ChatGPT update: You can return both the errors without
    // worrying about the return type.
    // dyn Error defers error determination until runtime
    // When boxed the compiler forgets the error types
    // Boxed or not, the output is same.
    println!("Boxed error: {}", er1);
    // understand what a panic does, first catch it...
    let yr_guess = panic::catch_unwind(|| guess(6));

    match yr_guess {
        Ok(res) => println!("Got bool: {:?}", res),
        Err(e) => println!("caught panic {:?}", e),
    }

    let yr_guess = panic::catch_unwind(|| guess(16));

    match yr_guess {
        Ok(res) => println!("Got bool: {:?}", res),
        // e contains the string that is printed on the stack trace
        Err(e) => println!("caught panic {:?}", e),
    }

    // the simplest form of error returning is with string
    // which doesn't allow you to use any of the Error method
    let work_result = send_error(6);
    match work_result {
        Ok(res) => println!("Returned boolean: {}", res),
        Err(s) => println!("Returned s: {}", s),
    }
    // let new_work_o = send_custerror(3).unwrap();
    // custom error with unwrap panics and aborts
    let new_work_e = send_custerror(7).unwrap_or_else(|e| false);
    println!("Returned error but no panic: {}", new_work_e);
    let new_work_e = send_custerror(3).unwrap();
    // unwrap internally calls panic!

    println!("So this is a new value: {}", new_work_e);

    // let new_string_e = send_error_str(7).unwrap();
    // println!("Returned error string no panic: {}", new_work_e);
    // panics and bails out here, and nothing below will work
    // let new_string_e = send_error_str(7).unwrap_or_else(|e| "logic chk");
    let new_string_e0 = send_error_str(7).ok();
    let new_string_o = send_error_str(3).unwrap();
    // unwrap internally calls panic!
    println!("Returned value at the end: {}", new_string_o);
}
#[derive(Debug)]
struct CustomError;
// the struct has to be plain or has to be enum

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "this is a string from custom error")
    }
}
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Jump out of the main: {}", n)
        // how to catch a panic?
    }
    n == 6
}

fn take_args_unwrap() {
    // first get the bin name
    let mut argv = std::env::args();

    let arg: String = argv.nth(0).unwrap();
}

fn send_error(n: i32) -> Result<bool, &'static str> {
    if n < 5 {
        Ok(true)
    } else {
        Err("This is a simple error")
    }
}

fn send_error_str(n: i32) -> Result<bool, String> {
    if n < 5 {
        Ok(true)
    } else {
        Err("This is a simple error".to_string())
    }
}
fn send_custerror(n: i32) -> Result<bool, CustomError> {
    if n < 5 {
        Ok(true)
    } else {
        Err(CustomError)
    }
}
