#[allow(dead_code)]

use std::io::*;

fn operate_numbers(d1: i32, d2: i32) {
    println!("{} + {} = {}", d1, d2, d1 + d2); 
    println!("{} - {} = {}", d1, d2, d1 - d2); 
    println!("{} * {} = {}", d1, d2, d1 * d2); 
    // casting the integer to f64
    let d3 = f64::from(d1); 
    let d4 = f64::from(d2); 
    println!("{} / {} = {}", d3, d4, d3 / d4); 
}

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    // operator is not required
    // let mut ope = String::new();
    // move the inputs to the variables
    println!("Provide the first number: ");
    stdin().read_line(&mut num1).unwrap();
    println!("Provide the second number: ");
    stdin().read_line(&mut num2).unwrap();
    // println!("Provide the Operation to perform: ");
    // stdin().read_line(&mut ope).unwrap();
    // trim and parse the strings, and shadow the 
    // variables
    let num1: i32 = num1.trim().parse().unwrap();
    let num2: i32 = num2.trim().parse().unwrap();
    // let ope: String = ope.trim().to_string();
    println!("{} {}", num1, num2);
    // call the function that prints all the 
    // operation and its outputs
    operate_numbers(num1, num2);
}