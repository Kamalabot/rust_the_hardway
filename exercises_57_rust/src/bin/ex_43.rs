use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Lets play parsing");
    let mut in_raw = String::new();
    let mut file_obj = File::open("parser_input.txt").unwrap();
    file_obj.read_to_string(&mut in_raw).unwrap();
    // trim the raw data, else it will create challenge later
    let in_cln = in_raw.trim();
    // split the data into lines of string
    let line_vec: Vec<&str> = in_cln.split("\n").collect();
    // loop on the lines, split them and feed them into hashmap and
    // then into the list
    let keys = ["First", "Last", "Salary"];
    let mut data_store = Vec::new();
    // starting loop
    for line in line_vec {
        let mut map_line = HashMap::new();
        let row_vec: Vec<&str> = line.split(",").collect();
        for (idx, val) in row_vec.into_iter().enumerate() {
            map_line.insert(keys[idx], val);
        }
        data_store.push(map_line)
    }
    let data_len = data_store.len();
    println!("There are {data_len} records");

    println!("Last     First     Salary");
    println!("--------------------------");

    for data in data_store {
        println!("{:?}", data);
        let last = data.get("Last").unwrap();
        let first = data.get("First").unwrap();
        let salary = data.get("Salary").unwrap();
        println!("{last}   {first}   {salary}");
    }
}
