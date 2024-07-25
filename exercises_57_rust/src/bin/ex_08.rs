#[allow(dead_code)]
use std::io::*;

fn main() {
    let mut people = String::new();
    let mut pizzas = String::new();
    // get the inputs for the user
    println!("How many people are there?");
    stdin().read_line(&mut people).unwrap();
    println!("How many pizzas are there?");
    stdin().read_line(&mut pizzas).unwrap();
    let people:i32 = people.trim().parse().unwrap();
    let pizzas:i32 = pizzas.trim().parse().unwrap();
    println!("{people} people with {pizzas} pizzas");
    let total_pcs = pizzas * 8;
    let per_guy = total_pcs / people;
    let left_pcs = total_pcs % people;
    println!("Each person gets {per_guy} pieces of pizza.");
    println!("There are {left_pcs} pieces of pizza are left.");
}