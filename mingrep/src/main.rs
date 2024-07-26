#[allow(dead_code)]

use std::env;
// use std::fs;
use std::process;
// use std::error::Error;
use mingrep::Config;

fn main() {
    let collect_strings: Vec<String> = env::args().collect(); 
    println!("All cli words {:?}", collect_strings);

   //  let query = &collect_strings[2];
    // let filename = &collect_strings[1];

    // println!("file to search: {:?}", filename);
    // println!("All cli words {:?}", query);
    
    // println!("***** File Data *********");
    // need to read the file into strings
    let config = Config::parse_args(&collect_strings).unwrap_or_else(|err| {
        println!("Issue in argument parsing: {}", err);
        process::exit(1);
    });   
    // let filedata = fs::read_to_string(config.filename)
        // .expect("The file required is not present in directory");

    // println!("{}", filedata);
    // run(config);
    if let Err(e) = mingrep::run(config) {
        println!("Error in processing config {:?}", e);
        process::exit(1);
    }
}
 