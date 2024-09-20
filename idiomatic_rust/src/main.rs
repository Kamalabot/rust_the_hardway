#![allow(warnings)]
#![allow(unused_imports)]

use idiomatic::*;
mod prac_mod0;
use prac_mod0::*;

fn main() {
    idiomatic1();
    let path = "idiomatic_way.md";
    let file_data = read_file_content(path).unwrap();
    // println!("Using file_data {:?}", file_data);
    // idioming_types();
    let us1 = User0::new("Ganapati".to_owned(), 100);
    // after implementing Display trait
    println!("us1 is {}", us1);
    use_idioms();
}
