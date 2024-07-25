#[allow(dead_code)]
use std::io::*;

fn main() {
    let year = 2024;
    let mut your_age = String::new();
    let mut ret_age = String::new();
    println!("Provide your current age: ");
    stdin().read_line(&mut your_age).unwrap();
    println!("When you like to retire: ");
    stdin().read_line(&mut ret_age).unwrap();
    // parse the recieved ages into numbers
    let your_age: i32 = your_age.trim().parse().unwrap();
    let ret_age: i32 = ret_age.trim().parse().unwrap();
    // calculate the avbl age 
    let avbl = ret_age - your_age;
    // present the findings to the user
    println!("You have {avbl} years before you retire");
    let ret_year = year + avbl;
    println!("Its {year} now, you can retrire by {ret_year}");
}