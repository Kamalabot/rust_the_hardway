#[allow(unused_imports)]
#[allow(unused_variables)]

use std::{io, error::Error};
use csv::Reader;

fn run() -> Result<(), Box<dyn Error>>{
    let mut rdr = Reader::from_reader(io::stdin());
    for rec in rdr.records(){
        let row = rec?;

        let city = &row[0];
        let state = &row[1];
        // if there will be missing vals 
        // choose Option
        let pop: Option<u64> = row[2].parse().ok();
        let _lon: f64 = row[4].parse()?;
        let _lat: f64 = row[3].parse()?;

        println!("{{city: {:?}, \
                  state: {:?}, \
                  pop: {:?}}}", city, state, pop);
    } 
    Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("issue in reading:{}", e);
    }
}
