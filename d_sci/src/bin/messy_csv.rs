#[allow(dead_code)]
#[allow(unused_imports)]

// disable headers, change delimiters
// change quote strategy
// allow different record lengths
// ignore lines

use csv::ReaderBuilder;
use std::{io, error::Error};


fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
   if let Err(e) = run() {
        println!("The reader bailed: {:?}", e);
    } 
}
