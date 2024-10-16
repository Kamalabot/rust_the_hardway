use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Word frequency dashboad");
    let mut rw_data = String::new();
    let mut fobj = File::open("word_freq.txt").unwrap();
    fobj.read_to_string(&mut rw_data).unwrap();
    let data = rw_data.trim().replace("\n", " ");
    let mut count_map = HashMap::new();
    let data_vec: Vec<&str> = data.split(" ").collect();
    for word in data_vec {
        match count_map.get(word) {
            Some(curr) => count_map.insert(word, curr + 1),
            None => count_map.insert(word, 1),
        };
    }
    println!("{:?}", count_map);
    for val in count_map.iter() {
        print!("{}: ", val.0);
        // *val.1 was used to deref the address of the i32
        for _ in 0..*val.1 {
            print!("*");
        }
        println!();
    }
}
