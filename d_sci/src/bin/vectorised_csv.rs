use std::io::{self, BufRead};
use csv::ReaderBuilder;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut reader = ReaderBuilder::new().from_reader(handle);

    let mapvec: Vec<_> = reader
        .records()
        .map(|r| r.expect("issue in mapping records"))
        .collect();
    
    println!("Total number of records: {}", mapvec.len());

    println!("First record is: {:?}", mapvec[0]);
}
