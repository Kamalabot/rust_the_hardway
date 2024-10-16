use std::fs::{self, File};
use std::io::Read;

fn main() {
    println!("Filtering by reading files");
    let mut rd_str = String::new();
    let mut rd_obj = File::open("line_num.txt").unwrap();
    rd_obj.read_to_string(&mut rd_str).unwrap();
    rd_str = rd_str.trim().to_string();
    let vec_str: Vec<&str> = rd_str.split(" ").collect();
    let vec_i32: Vec<i32> = vec_str
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .filter(|&x| x % 2 == 0)
        .collect();
    let fil_str: Vec<String> = vec_i32.into_iter().map(|x| x.to_string()).collect();
    let vec_fil = fil_str.join(" ");
    fs::write("num_fill.txt", vec_fil).unwrap();
}
