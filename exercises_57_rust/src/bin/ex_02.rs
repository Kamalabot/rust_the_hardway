use std::io::*;

fn main() {
    println!("Enter the string for which you want to extract the length: ");
    // define a storage string
    let mut your_string = String::new();
    // extract the data from the input line
    stdin().read_line(&mut your_string).unwrap();
    let mut length: usize;
    length = your_string.len();
    print!("The length of {} is {} ", your_string, length);
}
