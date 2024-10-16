use std::fs::{self, File};
use std::io::Read;

fn main() {
    println!("Reading, sorting & writing to file");
    let mut in_names = String::new();
    let mut rd_obj = File::open("names_input.txt").unwrap();
    rd_obj.read_to_string(&mut in_names).unwrap();
    let mut in_vec: Vec<&str> = in_names.split("\n").collect();
    in_vec.sort();
    let list_len = in_vec.len();
    println!("Sorted names: {:?}", in_vec);
    let heading = format!("Total {list_len} names\n");
    let names_list = in_vec.join("\n");
    let data_out = [heading, names_list].concat();
    fs::write("out_names.txt", data_out).unwrap();
}
