#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(warnings)]

fn main() {
    println!("Hello, world!");
    let chkpass = cp_base("mypassis01".to_owned());
    println!("base pass is {}", chkpass);
    let chkpass1 = cp01("mypassis01".to_owned());
    match chkpass1 {
        Some(val) => println!("pass is {:?}", val),
        _ => println!("pass is None"),
    }
    let cp2 = cp02("Yourpassis".to_owned());
    match cp2 {
        Ok(val) => println!("Extracted result is: {}", val),
        Err(e) => println!("Error is {}", e),
    }
    let cp21 = cp02("Youris".to_owned());
    match cp21 {
        Ok(val) => println!("Extracted result is: {}", val),
        // Err(e) => println!("{}", e),
        Err(e) => println!("{}", e),
    }
    // the above will process the error and continue
    let cp3 = cp02("chckunwr".to_owned()).unwrap();
    println!("Cp3 is {}", cp3);
    let cp31 = cp02("chck12345".to_owned()).unwrap();
    // the above line will panic, if the function throw error
    // along with the Err(str) that has been coded into the function
    println!("Cp31 is {}", cp31);
    let cpnc0 = cp_panic("nopanicwd".to_owned());
    println!("Cpanic is {}", cpnc0);
    // let cpnc1 = cp_panic("nopan".to_owned());
    // println!("Cpanic is {}", cpnc1);

    let _ = file_read("numfile.txt");
    let _ = file_read("numfile1.txt");
    println!("This gets printed");

    rc();
    rc1()
}
// Create a function that returns bool / value type
// Create a function that returns Option
// Update the function to return Result with &str
// Update that with CustomError
// Update that with String
//
// Write a function to read the file and parse the
// data to integers.
// Update the same with map and map_err
//
fn cp_base(word: String) -> bool {
    if word.len() >= 8 {
        true
    } else {
        false
    }
}
fn cp_panic(word: String) -> bool {
    // true else panic
    let wl = word.len();
    match wl {
        x if x > 8 => true,
        _ => panic!("length is less than 8. Panic!!!"),
    }
}
fn cp01(word: String) -> Option<i32> {
    if word.len() >= 8 {
        let l = word.len();
        Some(l as i32)
    } else {
        None
    }
}

fn cp02(word: String) -> Result<i32, &'static str> {
    if word.len() >= 8 {
        let l = word.len();
        Ok(l as i32)
    } else {
        Err("Length is less than 8")
    }
}

fn cp03(word: String) -> Result<i32, String> {
    if word.len() >= 8 {
        let l = word.len();
        Ok(l as i32)
    } else {
        Err("Length is less than 8".to_owned())
    }
}
use core::fmt;
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_read<P: AsRef<Path>>(yourpath: P) -> Result<(), Box<dyn Error>> {
    // just print content, by reading step by step with error handling
    let mut content = String::new();
    // faced issue in attaching error back to reader variable
    // which cannot be done. You have to return it
    // else you have to process the file into string
    // let reader = match File::open(yourpath) {
    //     Ok(file) => file,
    //     Err(h) => Err(h),
    // };

    if let Err(e) = File::open(&yourpath) {
        println!("Error: {}", e);
    };

    if let Ok(mut file) = File::open(yourpath) {
        file.read_to_string(&mut content)?;
        // the above line can throw an error
        // so using the ? operator
        println!("Content: {}", content)
    };
    // the above will also not work, as the error is
    // not same as type of file,
    Ok(())
}

fn rc() {
    let mut argv = std::env::args();
    // this is mainly options that are either
    // value or none
    let c1 = argv.nth(0);
    match c1 {
        Some(c) => println!("first value is {}", c),
        _ => println!("no value"),
    }
    println!("See what happens when Some is extracted");
    if let Some(c) = argv.nth(0) {
        println!("when there is 2nd arg {}", c)
    }
}

fn rc1() {
    let mut argv = std::env::args();
    let k1 = argv.next();
    match k1 {
        Some(v) => println!("Value is: {}", v),
        _ => println!("No value"),
    }
    let k3 = argv.next();
    match k3 {
        Some(v) => println!("Value is: {}", v),
        _ => panic!("No 3rd arg!!!"),
    }
}

#[derive(Debug)]
struct CliError;

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no arg provided")
    }
}

// impl From<Error> for CliError {
//     fn from(err: Error) -> CliError {
//         CliError
//     }
// }

impl Error for CliError {}

// can't return an error, as it is a trait

fn rc3() -> Result<(), Box<dyn Error>> {
    // converting option to error and returning
    let mut argv = std::env::args();
    let c1 = argv.next();
    match c1 {
        Some(x) => {
            println!("{}", x);
            Ok(())
        }
        _ => Err(Box::new(CliError)),
    }
}
