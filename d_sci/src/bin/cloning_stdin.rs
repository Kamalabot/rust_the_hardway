use std::io::{self, BufRead, Read};
use csv::{Reader,ReaderBuilder};

fn process_string<R: BufRead>(reader: &mut Reader<R>) {
    println!("Entering fn1");
    for result in reader.records(){
        let rec = result.expect("read record failed");
        println!("process in fn 1: {:?}", rec);
    }
}

fn process_string_2<R: BufRead>(reader: &mut Reader<R>) {
    println!("Entering fn2");
    for result in reader.records(){
        let rec = result.expect("read record failed");
        println!("process in fn 2: {:?}", rec);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    // let input = stdin.lock()
    //     .lines()
    //     .collect::<Result<String, _>>()
    //     .expect("reading stdin failed");

    stdin.lock().read_to_string(&mut input).expect("reading failed");   
    // the above collects it as string
    println!("{}", input);
    let mut reader1 = ReaderBuilder::new().from_reader(input.as_bytes());
    process_string(&mut reader1);
    
    let mut reader2 = ReaderBuilder::new().from_reader(input.as_bytes());
    process_string_2(&mut reader2);
}










