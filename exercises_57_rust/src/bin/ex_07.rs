#[allow(dead_code)]
use std::io::*;

fn main() {
    let mut width = String::new();
    let mut length = String::new();
    // get the data in
    println!("What is the width of the room: ");
    stdin().read_line(&mut width).unwrap();
    println!("What is the length of the room: ");
    stdin().read_line(&mut length).unwrap();
    // parse them to i32
    let width: i32 = width.trim().parse().unwrap();
    let length: i32 = length.trim().parse().unwrap();
    println!("You have entered {} and {} in feet", width, length);
    println!("The Area is: ");
    let area = width * length;
    println!("{area} square feet");
    let area_sqmt:f64 = f64::from(area) * 0.092903;
    println!("{area_sqmt} in square meters");
}
