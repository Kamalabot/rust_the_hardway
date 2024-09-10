use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{error::Error, fmt};

mod inmod;

use inmod::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let path = "error_handling_notes.md";
    // the inmod has open_file implemented with
    // io::Error throw, which is caught here
    let cont: String = open_file(path)?;
    println!("Extracted string: {}", cont);

    // using unwrap_or following unwrap on return value
    let opt1 = Some("65");
    let parse_opt1 = opt1.unwrap_or("0").parse::<i32>();
    println!("Unwrap and parse: {:?}", parse_opt1.unwrap());

    let num = validate_number("66")?;
    println!("Double num: {}", num * 2);

    // working on options and unwrapping them
    let instr = "This is a long worded sentence.";
    let word = "long";
    let present = instr.find(word);
    println!("Unwrapping the present: {}", present.unwrap());

    // working with iterators
    let invec = vec!["6", "5", "9", "3"];
    let parse_vec: Vec<Result<i32, ParseIntError>> =
        invec.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Printing parsed vector {:?}", parse_vec);
    let _ = err_main();
    Ok(())
}

fn err_main() -> Result<(), MyError> {
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
