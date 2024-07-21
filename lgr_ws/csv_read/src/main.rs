#[allow(dead_code)]
#[allow(unused_imports)]

use csv::{Reader, ReaderBuilder};

fn main() {
    let file = "csv_file.csv";
    let mut obj_builder = ReaderBuilder::new();
    obj_builder
        .double_quote(false)
        .comment(Some(b'-')) // look at the Some() wrapper
        .delimiter(b',')
        .has_headers(false);
    // let reader = Reader::from_path(file);
    let reader = obj_builder.from_path(file);

    if reader.is_err(){
        println!("Something is wrong...");
        std::process::exit(9); // exit with code 9
        // after above line, the exec will stop
    }

    let mut read_data = reader.unwrap();

    for data in read_data.records(){
        let car = data.unwrap();
        println!("Manufacturer: {}", car.get(0).unwrap());
        println!("Model: {}", car.get(1).unwrap());
        println!("qty: {}", car.get(2).unwrap());
    }
}
