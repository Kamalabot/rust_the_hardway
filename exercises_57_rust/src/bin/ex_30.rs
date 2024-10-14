use std::io::stdin;

fn main() {
    println!("cheking till getting valid input");
    loop {
        let mut rate = String::new();
        println!("provide the rate in your mind: ");
        stdin().read_line(&mut rate).unwrap();
        let srate = rate.trim();
        if srate.is_empty() {
            println!("Enter a rate to continue");
        } else if srate.chars().any(|f| f.is_alphabetic()) {
            println!("Rate can only contain numeric values");
        } else if srate == "0" || srate == "0.0" {
            println!("the rate cannot be 0");
        } else {
            let frate = srate.parse::<f32>().unwrap();
            let dbl_yr = 72.0 / frate;
            println!("The dbl year is: {dbl_yr}");
            println!("See you around");
            break;
        }
    }
}
