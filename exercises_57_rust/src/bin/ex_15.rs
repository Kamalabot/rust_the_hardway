#[allow(dead_code)]
use std::io::*;

fn main() {
    let mut age = String::new();
    println!("What is your age?");
    stdin().read_line(&mut age).unwrap();
    let age: i32 = age.trim().parse().expect("Age is expected to be a number");
    let your_stat = check_driving_age(age);
    println!("Your status is {your_stat}"); 
}

fn check_driving_age(your_age: i32) -> String {
    if your_age >= 16{
        "You are old enough to drive".to_owned()
    } else {
        "You are not old enough to drive".to_owned()
    }
}

#[cfg(test)]
mod tests{
    use crate::*;

    #[test]
    fn test_cda(){
        assert_eq!(check_driving_age(16), "You are old enough to drive".to_owned());
        assert_eq!(check_driving_age(6), "You are not old enough to drive".to_owned());
    }
}