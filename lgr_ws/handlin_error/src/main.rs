use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

fn main() {
    // panic!("Lets go kumbaya...");
    // a();
    fn a(){
        b();
    }
    fn b(){
        c(21);
    }
    fn c(n: i32){
        if n == 21{
            panic!("21 in... Go in panic mode...");
        }
    }
    let f = File::open("hello.txt"); 
    let p = match f {
        Ok(fval) => fval, // this value is move into the p variablee..
        Err(e) => panic!("Ok file is missing or awol.. {}", e),
        // we can use ErrorKind enum variant, which will take more match pattens.
    };

    let f1 = File::open("newello.txt").unwrap(); // similar to using ? operator
     
    let f2 = File::open("more.txt").expect("lets make the file...");

    fn read_uname_from_file() -> Result<String, io::Error>{
        let mut temp = File::open("new_fun.txt")?;
        let mut newstring = String::new();
        temp.read_to_string(&mut newstring)?;
        Ok(newstring)
        // File::open("hello.txt")?.read_to_string(&mut newstring)?;
    }
}        