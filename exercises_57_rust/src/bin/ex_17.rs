#[allow(dead_code)]

/*
takes alcohol consumed, body Weight, 
gender, hours since consumed and 
returns blood alcohol content. 
If BAC is less than 0.08, legal to drive
 */
fn ba_calc(consumed: f64, weight: f64, hrs: f64, gender: &str) -> bool {
    /*Will return bac f64 value */
    // get the adr based on the gender
    let prc_gender = gender.to_lowercase();
    let gdr = prc_gender.as_str();
    // let male = "male".to_owned();
    // let female = "female".to_owned();
    // check if male or female
    let adr: f64 = match gdr {
        "male" => 0.73,
        "female" => 0.66, 
        _ => panic!("has to male or female")
    };
    let bac: f64 = ((consumed * 5.14) / (weight * adr)) - (0.015 * hrs);
    println!("Just showing bac: {}", bac);
    if bac < 0.08 {
        false
    } else {
        true
    }
}

fn main() {
    /* Work on functions here */
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_bac() {
        assert!(ba_calc(15.0, 65.0, 3.0, "female"));
        assert!(ba_calc(15.0, 65.0, 3.0, "male"));
    }
}