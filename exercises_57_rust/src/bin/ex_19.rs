#[allow(dead_code)]

use std::io::*;

fn the_calc(wt: f64, ht: f64) -> String {
    println!("Your weight is {}", wt);
    println!("Your height is {}", ht);
    let bmi = (wt / (ht * ht)) * 703.0;
    if bmi < 18.5 {
        println!("Your BMI is {}", bmi);
        "You are underweight, visit doctor...".to_owned()
    } else if bmi > 25.0 {
        println!("Your BMI is {}", bmi);
        "you are overweight, visit doctor...".to_owned()
    } else {
        println!("Your BMI is {}", bmi);
        "You are fine go home and stay home".to_owned()
    }
}

fn bmi_calc(weight: f64, height: f64, unit: &str) -> String {
    println!("Your measurement system is {}", unit);

    // if unit == "metric" {
    //     let wt = weight * 2.20462;
    //     let ht = height * 0.393701;
    //     the_calc(wt, ht)
    // } else {
    //     println!("Yep it is going thru here... with {}", unit);
    //     the_calc(weight, height)
    // }
    match unit {
        "metric" => the_calc(weight * 2.20462, height * 0.39370), 
        "imperial" => the_calc(weight, height), 
        _ => panic!("Either Metric or Imperial")
    }
}

fn main() {
    let mut your_wt = String::new();
    let mut your_ht = String::new();
    let mut unit_std = String::new();

    println!("Enter the units of measurement metric / imperial");
    stdin().read_line(&mut unit_std).unwrap();
    let unit_std = unit_std.trim();

    println!("Enter your current weight: ");
    stdin().read_line(&mut your_wt).unwrap();
    
    println!("Enter your current height: ");
    stdin().read_line(&mut your_ht).unwrap();

    let your_wt: f64 = your_wt.trim().parse().expect("Enter only decimal weights");
    let your_ht: f64 = your_ht.trim().parse().expect("Enter only decimal heights");

    println!("Your Status: {}", bmi_calc(your_wt, your_ht, unit_std));
}

// #[cfg(tests)]
// mod tests {
//     use crate::*;

//     #[test]
//     fn test_bmic() {
//         assert_eq!(bmi_calc())
//     }
// }
// Adding a new line
// another new line
