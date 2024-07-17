#[allow(dead_code)]
use std::io;

fn main(){
    println!("Just don't sit there, enter a number: ");
    let mut in_data = String::new();
    io::stdin()
        .read_line(&mut in_data)
        .unwrap();
    println!("Before parsing: {}", in_data);
    // shadow change
    let in_data : u32 = in_data
        .trim()
        .parse()
        .unwrap();
    
    println!("After parsing: {}", in_data);

    if in_data == 42 {
        println!("Printing u32 {}", in_data);
    } else {
        println!("Number is not 42");
    }
    
}