use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() {
    println!("Magic 8 Ball");
    // a simple array is sufficient, as it is not growing or reducing
    let future = ["Yes", "No", "May Be", "Ask again later"];
    let mut quest = String::new();
    println!("Welcome to Magic 8, Ask your question");
    stdin().read_line(&mut quest).unwrap();
    let rnd = thread_rng().gen_range(0..future.len());
    let your_fut = future[rnd];
    println!("{your_fut}");
}
