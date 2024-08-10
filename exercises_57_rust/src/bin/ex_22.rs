#[allow(dead_code)]

use std::io::*;

fn main() {
    let mut x = String::new(); 
    let mut y = String::new();
    let mut z = String::new();

    println!("Provide the 1st number: ");
    stdin().read_line(&mut x).unwrap();
    let x: i32 = x.trim().parse().expect("Has to be number");
    
    println!("Provide the 2nd number: ");
    stdin().read_line(&mut y).unwrap(); 
    let y: i32 = y.trim().parse().expect("Has to be number");

    println!("Provide the 3rd number: ");
    stdin().read_line(&mut z).unwrap();
    let z: i32 = z.trim().parse().expect("Has to be number");

    if x > y && x > z {
        println!("X is largest among three: {}", x);
    }
    else if y > x && y > z {
        println!("Y is largest among three: {}", x);
    }
    else {
        println!("Z is largest among three: {}", z)
    }
}
