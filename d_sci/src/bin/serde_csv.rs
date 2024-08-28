#[allow(unused_variables)]

use std::{
    io,
    error::Error, 
    collections::HashMap
};
// use serde::{Serialize, Deserialize};
use csv::{Reader, StringRecord};

type SimpleRecord = (String, String,
                        Option<u64>, f64, f64);

// fn run() -> Result<(), Box<dyn Error>>{
// The R: std::io::Read trait bound is used to ensure that 
// the Reader can work with any type that implements 
// the Read trait.
fn run<R: std::io::Read>(csv_reader: &mut Reader<R>) -> Result<(), Box<dyn Error>>{
    // can we move the reader to main
    // barrow it for different functions?
    // When we are sending it as function parameter, Rust is 
    // throwing error
    // let mut rdr = Reader::from_reader(io::stdin());

    // create a iterator with Deserialize menthod
    for res in csv_reader.deserialize(){
        let rec: SimpleRecord = res?;
        println!("Simple Record: {:?}", rec);
    }
    Ok(())
}

type HashmapRec = HashMap<String, String>;
// observe the place where the mut keyword is used
fn run_hashmap<R: std::io::Read>(mut csv_reader: Reader<R>) -> Result<(), Box<dyn Error>> {
    for res in csv_reader.deserialize(){
        let rec: HashmapRec = res?;
        println!("Hashmaped: {:?}", rec);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let hdl = stdin.lock();

    let main_rdr = Reader::from_reader(hdl);
    // variable main_rdr need not be mutable
    // once the function takes ownership, there it is made mutable
    
    if let Err(e) = run_hashmap(main_rdr) {
        println!("Error in processing: {}", e);
    }

    // let mut main_rdr2 = Reader::from_reader(io::stdin());
    println!("Again creating the input...");
    
    // ** Below part of the code doesn't work
    let hdl2 = io::stdin().lock();
    let mut main_rdr2 = Reader::from_reader(hdl2);

    
    if let Err(e) = run(&mut main_rdr2) {
        println!("Error in processing: {}", e);
    }

    Ok(())
    
}







