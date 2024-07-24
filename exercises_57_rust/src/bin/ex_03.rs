use std::io::*;
fn main(){
    // start by defining the strings
    let mut quote = String::new();
    let mut author = String::new();
    // get the quotes using stdin()
    println!("What is the quote:...");
    stdin().read_line(&mut quote).unwrap();
    println!("Who said it?...");
    stdin().read_line(&mut author).unwrap();
    // since the stdin will have new_lines, trim them
    let trim_author: &str = author.trim();
    let trim_quote : &str = author.trim();
    // build the final sentence
    print!("The {trim_author} says {quote}");
}