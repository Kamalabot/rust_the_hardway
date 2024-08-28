#[allow(dead_code)]
#[allow(unused_imports)]

use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    io::Read,
    process
};


fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        // we can see how the Err is built here
        None => Err(From::from("Expected one argument after executable name...")),
        Some(f_path) => Ok(f_path)
    }
}

// returns nothing or Error of any kind
fn run() -> Result<(), Box<dyn Error>> {
    let f_path = get_first_arg()?;

    let mut file = File::open(&f_path)?;

    let mut fdata = String::new();
    file.read_to_string(&mut fdata)?;
    // print the file data
    println!("File data is: {:?}", fdata);

    let readagain = File::open(f_path)?;

    // cheked if the reader can still feed the data
    // nope, it doesnt
    let mut rdr = csv::Reader::from_reader(readagain);

    for rc in rdr.records() {
        let record = rc?;
        // what will happen if there is error 
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    // unwrap or expect doesn't send the error upwards
    // however these methods may not be available to the impl
    let mut rdr = csv::Reader::from_reader(std::io::stdin());

    for result in rdr.records(){
        // let rec = result.expect("Awaiting a csv file");
        // the above expect will simply panic
        if let Ok(rec) = result {
            println!("This is the record: {:?}", rec);
        } else {
            println!("There issue in the record");
        }
        // idea is above will continue record processing
    }

    let header = rdr.headers().expect("Issue with recovering header");
    println!("The headers is: {:?}", header);

    // cargo run --bin csv_tutorial invalid.csv < invalid.csv 

    if let Err(err) = run() {
        // run() processes the first cli argument
        println!("Only the error: {}", err);
        // throws this error when processing 
        process::exit(1);
    }

}








