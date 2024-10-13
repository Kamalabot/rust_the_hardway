use std::io::stdin;
fn main() {
    println!("Lets Calculate credit months");
    let mut i = String::new();
    let mut bal = String::new();
    let mut pmt = String::new();

    println!("Provide the balance");
    stdin().read_line(&mut bal).unwrap();
    let fbal = bal.trim().parse::<f32>().unwrap();

    println!("Provide the daily rate");
    stdin().read_line(&mut i).unwrap();
    let rate = i.trim().parse::<f32>().unwrap();

    println!("Provide the monthly payment");
    stdin().read_line(&mut pmt).unwrap();
    let fpmt = pmt.trim().parse::<f32>().unwrap();

    let n = get_months(fbal, fpmt, rate);

    println!("Months to pay: {n}");
}

fn get_months(bal: f32, pmt: f32, rate: f32) -> f32 {
    println!("the bal: {bal}, pmt: {pmt}, rate: {rate}");
    0.0
}

// n is the number of months. •
// i is the daily rate (APR divided by 365).
// • b is the balance.
// • p is the monthly payment
