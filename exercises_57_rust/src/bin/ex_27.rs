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
    // the percentage has to be converted to decimal

    println!("Months to pay: {n}");
}

fn get_months_updtd(bal: f32, pmt: f32, rate: f32) -> f32 {
    println!("Pmt: {pmt}, bal: {bal}, rate: {rate}");
    if pmt <= 0.0 || rate <= 0.0 || bal <= 0.0 {
        println!("Invalid input: payment, rate, and balance must be greater than 0.");
        return f32::NAN;
    }

    let daily_rate = rate / 365.0; // Monthly interest rate
    println!("daily rate: {daily_rate}");
    let monthly_ratefactor = (1.0 + daily_rate).powi(30);
    // Ensure that (pmt - bal * monthly_rate) is positive to avoid invalid log arguments
    if pmt <= bal * (monthly_ratefactor - 1.0) {
        println!("Invalid input: Payment is too small to cover even the interest.");
        return f32::NAN;
    }

    let numerator = pmt / (pmt - bal * (monthly_ratefactor - 1.0));
    println!("Numerator for log: {numerator}");

    // Compute the number of months using the amortization formula
    let months = (numerator.ln()) / monthly_ratefactor.ln();
    println!("Months to pay off: {months}");

    months
}

fn get_months(bal: f32, pmt: f32, rate: f32) -> f32 {
    // println!("the bal: {bal}, pmt: {pmt}, rate: {rate}");
    let apr = rate * 0.01 / 365.0;
    println!("Apr: {apr}");
    // add 1 to apr
    let mut p1 = 1.0 + apr;
    // raise it to 30th power
    p1 = p1.powi(30);
    println!("p1: {p1}");
    // substract it with 1
    let p2 = 1.0 - p1;
    // multiply it with bal / pmt
    let p3 = bal / pmt;
    println!("p3: {p3}");
    let p4 = 1.0 + (p3 * p2);
    println!("p4: {p4}");
    // take log of p4
    let logp4 = p4.log2();
    println!("logp4: {logp4}");
    // divide log numerator by log (1 + apr)
    let div1 = logp4 / (1.0 + apr).log(10.0);
    (-1.0 / 30f32) * div1
}

// n is the number of months. •
// i is the daily rate (APR divided by 365).
// • b is the balance.
// • p is the monthly payment
