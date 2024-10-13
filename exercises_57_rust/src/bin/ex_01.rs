#![allow(warnings)]
use std::fs::*;
use std::io::*;

fn main() {
    let mut name = String::new();
    println!("What is your name? ");
    stdin().read_line(&mut name).unwrap();
    println!("Hello {} its nice to meet you", name);
}
