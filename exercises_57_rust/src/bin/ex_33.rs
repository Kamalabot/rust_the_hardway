use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("guessing game... Three levels");
    chlng_choice();
}

fn chlng_choice() {
    let mut dfcl = String::new();
    println!("Provide the difficulty you want: 1, 2, 3");
    stdin().read_line(&mut dfcl).unwrap();
    let dfcl = dfcl.trim();
    match dfcl {
        "1" => simple(10),
        "2" => simple(100),
        "3" => simple(1000),
        _ => println!("Only values 1, 2, 3"),
    }
}

fn simple(upper: i32) {
    let inter = thread_rng().gen_range(1..=upper);
    println!("The inter is {inter}");
    loop {
        let mut your_guess = String::new();
        println!("Provide your guess between 1 to {upper}");
        stdin().read_line(&mut your_guess).unwrap();
        let nguess = your_guess.trim().parse::<i32>().unwrap();
        match nguess.cmp(&inter) {
            Ordering::Less => println!("Your guess is less than target"),
            Ordering::Greater => println!("Your guess is Grater than target"),
            Ordering::Equal => {
                // did not realize, how fast the logic grew
                println!("Your guess is equal to target");
                let mut cont = String::new();
                println!("You want to continue, Yes | No: ");
                stdin().read_line(&mut cont).unwrap();
                let tcont = cont.trim();
                if tcont == "yes" || tcont == "y" || tcont == "Y" {
                    // this call should take it back to top
                    chlng_choice();
                } else {
                    println!("See You...");
                    break;
                }
            }
        }
    }
}
