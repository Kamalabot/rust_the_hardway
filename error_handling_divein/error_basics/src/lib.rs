#![allow(warnings)]
use core::num;
use std::fs::File;
use std::io::Read;
use std::path::Path;



pub fn libmain() -> f32 {
    42.0
}

pub fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("not a valid number: {}", n)
    }
    n == 6
}

// Gets the cli args, and then processes it with match
pub fn take_args_match() {
    let mut argv = std::env::args();
    let mut args = argv.into_iter();
    println!("The args are: {:?}", args);
    // using match which unwrap does for us
    let first = match args.nth(0){
        Some(val) => val,
        None => "No val".to_owned() 
        // we can panic here, then it become unwrap() 
    };
    // this makes sense, when nth(0) is called, the val is consumed
    let second = match args.nth(0){
        Some(val) => val,
        None => "No val".to_owned()
    };

    println!("First: {}, Second: {}", first, second);
}

// Takes the cli vals and uses unwrap
pub fn take_args_unwrap() {
    let mut argv = std::env::args();
    // needs additional arg to the cli

    // let arg0: String = argv.nth(0).unwrap();

    // println!("Got bin name {}", arg0);

    let arg: String = argv.nth(0).unwrap();

    let arg: String = argv.nth(0).expect("got nothing, expecting a number");
    // and it has to be a number
    let n: i32 = arg.parse().unwrap(); // will throw error 
    println!("Double: {}", n * 2);
}

// The return of the Option, implicitly contains the None
// check this guys usage
pub fn find(haystack: &str, needle: char) -> Option<usize> {
    for(offset, c) in haystack.char_indices() {
        // above will loop on the string characters
        if c == needle {
            return Some(offset)
        }
    }
    None
}

pub fn extension_exp(filename: &str) -> Option<&str> {
    match find(filename, '.'){
        None => None,
        Some(i) => Some(&filename[i+1..]),
    }
}

pub fn map<F, T, A>(option:Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(val) => Some(f(val)),
    }
}

// returns the extension of the filename, where extension is 
// defined as all characters after first .
pub fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}
// Following code is commented as it conflicts with std::result::Result
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// impl<T, E: ::std::fmt::Debug> Result<T, E> {
//     fn unwrap(self) -> T {
//         match self {
//             Result::Ok(val) => val,
//             Result::Err(err) =>
//               panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
//         }
//     }
// }

use std::num::ParseIntError;
use std::result::Result;

type localResult<T> = Result<T, ParseIntError>;

pub fn double_number(num_str: &str) -> localResult<i32> {
    match num_str.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(err) => Err(err)
    }
}


pub fn double_arg(mut argv: std::env::Args) -> Result<i32, String> {
     
    argv.nth(1)
        .ok_or("please give one arg in cli".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|i| i * 2)
}


pub fn ok_or<T, E>(option: Option<T>, err: E) -> Result<T, E> {
    match option {
        Some(val) => Ok(val),
        None => Err(err),
    }
}

// function that simply opens file and panics if there is erro
// 3 errors, opening, reading file, and parsing file data 
// all are diffrent errors to be handled
fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap(); // error 1
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // error 2
    let n: i32 = contents.trim().parse().unwrap(); // error 3
    2 * n
}

fn file_double_result<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    // all of the process is done on the file, 
    File::open(file_path)
         .map_err(|err| err.to_string())
         .and_then(|mut file| {
              let mut contents = String::new();
              file.read_to_string(&mut contents)
                  .map_err(|err| err.to_string())
                  .map(|_| contents)
         })
         .and_then(|contents| {
              contents.trim().parse::<i32>()
                      .map_err(|err| err.to_string())
         })
         .map(|n| 2 * n)
    // after all the above 14 lines of code, i32 is retuned as Result
}

fn file_double_simpler<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

// try is deprecated, use the ? operator
// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
//     let mut file = try!(File::open(file_path)
//         .map_err(|e| e.to_string()));

//     let mut contents = String::new();

//     try!(file.read_to_string(&mut contents)
//         .map_err(|e| e.to_string()));

//     let n = try!(contents.trim().parse::<i32>().map_err(|e| e.to_string()));
//     Ok(2 * n)
// }

fn file_double_tryop<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = File::open(file_path)
        .map_err(|e| e.to_string())?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let n = contents
        .trim()
        .parse::<i32>()
        .map_err(|e| e.to_string())?;

    Ok(2 * n)
}

#[derive(Debug)]
enum CliError {
    IoError(std::io::Error),
    ParseError(num::ParseIntError)
}

use std::io;

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IoError(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::ParseError(err)
    }
}

fn file_double_comp<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = File::open(file_path).map_err(CliError::IoError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(CliError::IoError)?;
    let n: i32 = contents.trim()
        .parse()
        .map_err(CliError::ParseError)?;
    Ok(n * 2)
}
// best way to handle the error
fn file_double_final<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>()?;
    Ok(2 * n)
}

macro_rules! fatal {
    ($($tt:tt)*) => {{
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($tt)*).unwrap();
        ::std::process::exit(1)
    }}
}
