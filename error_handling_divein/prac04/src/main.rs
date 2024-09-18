#![allow(unused_imports)]
#![allow(warnings)]

use std::any::type_name;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::panic;

fn main() {
    println!("Hello, world!");
    let yr_g = panic::catch_unwind(|| er00(6));
    println!("Caught the panic type {}", type_of(yr_g));
    let yr_g1 = er01(5);
    println!("{:?}", yr_g1);
    let yr_g11 = er01(6);
    println!("{:?}", yr_g11);
    // extracting the result data, with unwrap
    let op1 = yr_g1.as_ref().unwrap();
    println!("Extracting OK: {:?}", op1);
    // map Ok
    let yr_g12 = er01(6);
    let _ = yr_g12
        .map(|x| println!("Mapping OK {:?}", x))
        .map_err(|err| println!("Error mapped {:?}", err));

    let yr_g12o = er01(5);
    let _ = yr_g12o
        .map(|x| println!("Mapping OK {:?}", x))
        .map_err(|err| println!("Error mapped {:?}", err));
    let yr_g22 = er02(5);
    match yr_g22 {
        Ok(v) => println!("thru, so no error"),
        Err(e) => println!("{}", e),
    }
    let yr_g22e = er02(6);
    match yr_g22e {
        Ok(v) => println!("thru"),
        Err(e) => println!("New error: {}", e),
    }
    fmopen00("numfile.txt");
    // fmopen00("numfile1.txt");
    let _ = fmopen01("numfile.txt").unwrap();
    // let _ = fmopen01("numfile1.txt").unwrap();
    let _ = fmopen03("numfile.txt").unwrap();
    // let _ = fmopen03("numfile1.txt").unwrap();
    let _ = fmopen04("numfile.txt").unwrap();
    let _ = fmopen04("numfile1.txt").unwrap();
}
// create panic with condition
// return erro based on condition
// return custom error based on condition
// handle file opening and its errors
// handle cli argo and its errors
//
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
// The ::<> notation is called turbofish, and it's used to disambiguate the type during parsing.

fn er00(x: i32) {
    if x == 5 {
        println!("cool..")
    } else {
        panic!("You have to respect 5")
    }
}

fn er01(x: i32) -> Result<(), String> {
    if x == 5 {
        println!("This is nice..");
        Ok(())
    } else {
        Err("Not cool at all".to_owned())
    }
}

#[derive(Debug)]
struct Newerror;
// Is the unit struct type that occupies 0
// memory. used for Error types or marker types
// contains no data

impl Display for Newerror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // only this will get printe
        write!(f, "new error from the file also")
    }
}

impl Error for Newerror {}

fn er02(x: i32) -> Result<(), Newerror> {
    if x == 5 {
        println!("Fine.. try to get errored out");
        Ok(())
    } else {
        Err(Newerror)
    }
}
use std::path::Path;
struct MyPath<P: AsRef<Path>> {
    path: P,
}
impl<P: AsRef<Path>> std::fmt::Debug for MyPath<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PathB {:?}", self.path.as_ref())
    }
}
// P type has to implement AsRef for Path obj
fn fmopen00<P: AsRef<Path> + std::fmt::Debug>(path: P) {
    println!("Opening file at {:?}", path);
    let mut contents = String::new();
    let mut open = File::open(path).unwrap();
    // the error thrown indicates the start of the
    // function name in the error line
    let read = open.read_to_string(&mut contents);
    println!("Contents are: {}", contents);
    ()
}

fn fmopen01<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    // send the result back with the error
    let mut open = File::open(path)?;
    let mut contents = String::new();
    open.read_to_string(&mut contents)?;
    println!("Reading file: {}", contents);
    Ok(())
}

fn fmopen03<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let mut open = File::open(path);
    let mut content = String::new();
    match open {
        Ok(mut r) => {
            let ct = r.read_to_string(&mut content);
            println!("content is: {}", content);
            match ct {
                Ok(v) => println!("read file"),
                // Err(e) => return Err(Newerror);
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => return Err(Box::new(Newerror)),
    }
    Ok(())
}
fn fmopen04<P: AsRef<Path>>(path: P) -> Result<(), Newerror> {
    let mut open = File::open(path);
    let mut content = String::new();
    match open {
        Ok(mut r) => {
            let ct = r.read_to_string(&mut content);
            println!("content is: {}", content);
            match ct {
                Ok(v) => println!("read file"),
                // Err(e) => return Err(Newerror);
                Err(e) => {
                    println!("Here is your error {:?}", e);
                    return Err(Newerror);
                }
            }
        }
        Err(e) => {
            println!("Error in opening part: {:?}", e);
            return Err(Newerror);
        }
    }
    Ok(())
}

fn ctch_ind_err<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let mut raw = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };
    let mut c = String::new();
    let _ = match raw.read_to_string(&mut c) {
        Ok(v) => println!("okay val: {}", v),
        Err(e) => return Err(e.to_string()),
    };
    println!("content is {}", c)
    Ok(())
}
