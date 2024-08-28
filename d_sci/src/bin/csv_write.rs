#[allow(dead_code)]

use std::{error::Error, io};
use csv::Writer;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: i32
}


fn main() -> Result<(), Box<dyn Error>> {
    // Working on writing data to csv file
    let mut wtr = Writer::from_writer(io::stdout()); 

    // following wrote the data to file, with many warning
    wtr.write_record(["city", "region", "country", "population"])?;
    wtr.write_record(["SouthBorough", "MA", "United States", "652678"])?;
    println!("writing records by serializing..");

    // when writing the first record, the columns are written again 
    wtr.serialize(Record {
        city: "Hampshire".to_string(),
        region: "TX".to_string(),
        country: "United States".to_string(),
        population: 5680,
    })?;
    wtr.serialize(Record {
        city: "Pittsburg".to_string(),
        region: "Carolina".to_string(),
        country: "United States".to_string(),
        population: 6213,
    })?;
    wtr.flush()?;
    Ok(())
}
