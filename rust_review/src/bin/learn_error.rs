#[allow(dead_code)]
#[allow(unused_variables)]

use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

fn main() {
    // panic!("This is super awesome..."); 
    let newfile = "sl.txt";

    let output = File::create(newfile);

    let mut output = match output {
        Ok(data) => data, 
        Err(er) => {
            panic!("There is an error...{:?}", er);
        }
    };
    write!(output, "Lets push something to file...").expect("Writing Error");

    let getout = File::open(newfile).unwrap();
    // use it to the bufreader
    let buffed = BufReader::new(getout); 

    for line in buffed.lines(){
        println!("THis is a line: {}", line.unwrap());
    }

    // let out2 = File::create("rand.txt");
    
    let out2 = File::create("rand.txt");
    
    // we are doing file open again
    let out = match out2 {
        Ok(file) => file,
        Err(er) => match er.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("No file: {:?}", e)
            },
            _other_err => panic!("Problem:")
        },
    };

    println!("The file is: {:?}", out);

}














