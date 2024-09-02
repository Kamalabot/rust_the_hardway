// some basic practice on refactoring
#![allow(warnings)]
#[allow(unused_imports)]

use error_basics::*;
mod refactored;
mod folder;
mod prac;

use std::error::Error;
use std::fs;
use std::io;
use std::num;

// We have to jump through some hoops to actually get error values.
let io_err: io::Error = io::Error::last_os_error();

let parse_err: num::ParseIntError = "not a number".parse::<i32>().unwrap_err();

// OK, here are the conversions.
let err1: Box<Error> = From::from(io_err);
let err2: Box<Error> = From::from(parse_err);
// ar, their underlying type is erased from the 
// compiler’s knowledge, so it truly sees err1 and err2 as exactly the same.

fn main() {
   // guess(76);  // will panic and aborts 
    guess(6);  // will go through
    take_args_unwrap(); // wil panic for 2 reasons
    // the match can simply be called in the code
    match find("find.rs", '.') {
        None => println!("None value"),
        Some(val) => println!("Got the value: {}", val)
    }

    let ext = extension_exp("find.csv").unwrap();
    
    let ext1 = extension_exp("find.rs").unwrap_or("rs");

    assert_eq!(ext, "csv");
    assert_eq!(ext1, "rs");

    // idea of "if let" is to check for the result or error
    if let Ok(n) = double_number("10"){
        println!("The doubled value is : {}", n)
    };

}
