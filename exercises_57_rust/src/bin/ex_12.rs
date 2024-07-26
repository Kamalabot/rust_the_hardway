#[allow(unused_imports)]
#[allow(dead_code)]

fn calc_simple_interest(principal: f64, interest: f64, duration: f64) -> f64{
    let interest = (principal * interest * duration) / 100.0;
    println!("principle: {}", principal);
    println!("Interest: {}", interest);
    println!("Duration: {}", duration);
    println!("Simple interest is {}", interest);
    interest
}

fn calc_compound_interest(principal: f64, interest: f64, duration: f64, cmpd_time: i32) -> f64{
    let prd_cmpd: f64 = duration * f64::from(cmpd_time); // cast it and then multiply
    let rbyn: f64 = 1.0 + (interest / (f64::from(cmpd_time) * 100.0));
    let mut int_pd: f64 = principal * rbyn.powf(prd_cmpd);
    int_pd = (int_pd * 100.0).round() / 100.0; 
    println!("principle: {}", principal);
    println!("Interest: {}", interest);
    println!("Duration: {}", duration);
    println!("Times compounded: {}", cmpd_time);
    println!("Let compound interest = {}", int_pd);
    int_pd
}
fn main() {
    // test case are doing the heavy lifting...
}

#[cfg(test)]
mod tests{
    use crate::{calc_compound_interest, calc_simple_interest};

    #[test]
    fn test_si(){
        assert_eq!(calc_simple_interest(1000.0, 5.0, 1.0), 50.0);
        assert_eq!(calc_simple_interest(1000.0, 4.3, 4.0), 172.0);
        assert_eq!(calc_simple_interest(1500.0, 4.3, 4.0), 258.0);
    }
    #[test]
    fn test_ci(){
        assert_eq!(calc_compound_interest(1500.0, 4.3, 6.0, 4), 1938.84);
    }
}