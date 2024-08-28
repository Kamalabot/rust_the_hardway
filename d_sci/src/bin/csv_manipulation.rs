#[allow(dead_code)]

// extern crate csv;
// Above states csv is unresolved, and extern is unknow
// Above was used before 2018, currently we have to 
// add csv to the current crate and then use it.

use csv::{Reader, ReaderBuilder}; //, Result};
// use serde::{Serialize, Deserializer}
use std::{error::Error, io, process};
use serde::{Deserialize, Serialize};

fn example() -> Result<(), Box<dyn Error>> {
    // this line will ask for user input
    println!("Give me a file to process: ");
    let mut rdr = Reader::from_reader(io::stdin()); 

    // stdin values are sent through cargo 
    // run --bin excutable < /path/to/file.csv

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    } 
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>
}

fn example_serde(give_path: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(give_path)?; 
    for res in rdr.deserialize(){
        // automating deserialization
        let rec: Record = res?;
        println!("Deserialize value: {:?}", rec);
        // printing one record and breaking
        // break
    }
    Ok(())
}

fn example_noheader(give_path: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(give_path)?; 
    for res in rdr.deserialize(){
        // automating deserialization
        let rec: Record = res?;
        println!("Deserialize value: {:?}", rec);
    }
    Ok(())
}




fn main() -> Result<(), Box<dyn Error>>{
    // lets read a simple csv file
    let mut rdr = Reader::from_path("./data/Iris.csv")?; 
    let mut idx = 1; 
    for rec in rdr.records() {
        // println!("Record {} is {:?}", idx, rec);
        // let record = rec?;
        idx += 1;
        // simple match it and extract
        match rec {
            Ok(rc) => println!("Here we have rec {} and it is {:?}", idx, rc),
            Err(e) => println!("Some failure {:?}", e)
        }
        if idx > 5{
            break
        }
    }
    // ** Serializing the rows when there are no headers
    // if let Err(e) = example_noheader("./data/smallpop-no-headers.csv".to_owned()) {
    //     println!("Error in processing w/o header csv file:{}", e)
    // }
     
    // ** Serializing the csv rows into structs
    // if let Err(e) = example_serde("./data/smallpop.csv".to_owned()) {
    //     println!("The path must be not found: {}", e);
    //     // process::exit(1);
    // }

    // ** Base example where the csv file is sent in through stdin 
    // if the value returned by the example fn 
    // is error then block will execute
    // if let Err(err) = example() {
    //     println!("There is error: {}", err); 
    //     // inform there is error, and then bailout
    //     // process::exit(1);
    // }
    let mut singlerec = Reader::from_path("./data/smallpop-no-headers.csv")?;
    if let Some(rec) = singlerec.records().next() {
        println!("The record is: {:?}", rec?);
    };
    
    // building the reader for csv files w/o header
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path("./data/smallpop-no-headers.csv");
    let mut records = rdr?; // extracting the record
    
    // printing records by deserialization, when no headers are given
    for rec in records.deserialize() {
        let one_rec: Record = rec?;
        println!("Record from csv with no headers: {:?}", one_rec);
    }
    
    
    Ok(())

}
