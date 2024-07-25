#[allow(dead_code)]
use std::io::*;

fn main() {
    let mut qty1 = String::new();
    let mut qty2 = String::new();
    let mut qty3 = String::new();
    let mut price1 = String::new();
    let mut price2 = String::new();
    let mut price3 = String::new();
    // get the data
    println!("Enter the price of item 1:");
    stdin().read_line(&mut price1).unwrap();
    println!("Enter the qty of item 1:");
    stdin().read_line(&mut qty1).unwrap();
    println!("Enter the price of item 2:");
    stdin().read_line(&mut price2).unwrap();
    println!("Enter the qty of item 2:");
    stdin().read_line(&mut qty2).unwrap();
    println!("Enter the price of item 3:");
    stdin().read_line(&mut price3).unwrap();
    println!("Enter the qty of item 3:");
    stdin().read_line(&mut qty3).unwrap();
    let qty1 :f64 = qty1.trim().parse().unwrap(); 
    let qty2 :f64 = qty2.trim().parse().unwrap();
    let qty3 :f64 = qty3.trim().parse().unwrap();
    let price1 :f64 = price1.trim().parse().unwrap();
    let price2 :f64 = price2.trim().parse().unwrap();
    let price3 :f64 = price3.trim().parse().unwrap();
    let subtotal = (qty1 * price1) + (qty2 * price2) + (qty3 * price3); 
    let tax: f64 = (subtotal * 5.5) / 100.0;
    let total: f64 = subtotal + tax;
    println!("Subtotal: {}", subtotal);
    println!("Tax: {}", tax);
    println!("Total: {}", total);
}