use idiomatic::*;

fn main() {
    idiomatic1();
    let path = "idiomatic_way.md";
    let file_data = read_file_content(path).unwrap();
    // println!("Using file_data {:?}", file_data);
    // idioming_types();
}
