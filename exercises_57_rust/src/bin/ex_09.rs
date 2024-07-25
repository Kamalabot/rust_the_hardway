#[allow(dead_code)]
use std::io::*;

fn main() {
    let mut width = String::new();
    let mut length = String::new();
    println!("What is the width:");
    stdin().read_line(&mut width).unwrap();
    println!("What is the length:");
    stdin().read_line(&mut length).unwrap();
    // parse the input to integers
    let width:i32 = width.trim().parse().unwrap();
    let length:i32 = length.trim().parse().unwrap();
    // calculate the number of gallons required
    let area = width * length;
    let gallon: f64 = f64::from(area) / 350.0; 
    // using .ceil() to round it up
    println!("you will need {} gallon to cover {} area", gallon.ceil(), area); 
    // using .floor() to round it down
    println!("{} gallon will not be enough to cover {} area", gallon.floor(), area); 
}