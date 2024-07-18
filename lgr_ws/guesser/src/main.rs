#[allow(dead_code)]
use std::io;
use rand::prelude::*;
use std::cmp::Ordering;
use colored::Colorize;

fn main(){
    println!("Just don't sit there, enter a number: ");
    let rand_num = thread_rng().gen_range(0..101);
    println!("Random number generated is {}", rand_num);
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
    /* Error handling, by ignoring
    * let in_data : u32 = in_data
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => continue
        }
     */ 
    println!("After parsing: {}", in_data);

   /*  if in_data == 42 {
        println!("Printing u32 {}", in_data, );
    } else {
        println!("Number is not 42");
    } */
    // using cmp instead of using if
    match in_data.cmp(&rand_num) {
        // observe formatting specifier {} is required when colored 
        Ordering::Equal => println!("{}", "Win!!!".green()),
        Ordering::Less => println!("{}", "Small :( !!!".red()),
        Ordering::Greater => println!("{}", "Big :) !!!".red()),
    } 
    
}