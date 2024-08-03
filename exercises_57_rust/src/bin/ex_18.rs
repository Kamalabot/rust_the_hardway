#[allow(dead_code)]

// https://stackoverflow.com/questions/37472684/is-it-possible-to-compare-a-char-with-char-or-mut-charhttps://stackoverflow.com/questions/37472684/is-it-possible-to-compare-a-char-with-char-or-mut-charhttps://stackoverflow.com/questions/37472684/is-it-possible-to-compare-a-char-with-char-or-mut-char
// https://users.rust-lang.org/t/what-is-the-best-way-to-compare-the-chars-of-string/64056/6
// https://stackoverflow.com/questions/71373308/how-can-i-declare-a-default-constructed-char
// finally one route
// https://users.rust-lang.org/t/reading-a-character-from-stdin/70101/3

// use approx::relative_eq;

// use assert_approx_eq::assert_approx_eq;

use std::io::*;

fn cel_to_faren(cel: f64) -> f64 {
    /* Temperature in celsius will be taken and 
       Farenheit will be returned */

    (cel * 9.0) / 5.0 + 32.0
}

fn faren_to_cel(faren: f64) -> f64 {
    /* Temperature in Farenheit will be taken and 
       celsius will be returned */

    (faren - 32.0) * (5.0 / 9.0)
}

fn main() {
    /* you can work here */
    println!("To convert from Celsius to Farenheit, Press C ");
    println!("To convert from Farenheit to Celsius, Press F ");
    // stdin().read(conv).unwrap();
    let mut conv = String::new(); 
    stdin().read_line(&mut conv).unwrap();
    // readline as usual
    let conv = conv.chars().next();
    // get the next character and assign to character
    let got_char = match conv {
        Some(c) => c,
        _ => panic!("No input")
    }; // extract the character from Option
    // use the character for further matching
    match got_char {
        'F' => {
            let mut temp = String::new();
            println!("Enter the Temperature in Farenheit: ");
            stdin().read_line(&mut temp).unwrap();
            let ftemp: f64 = temp.trim().parse().expect("Enter Temperature in with decimal places.");
            let ctemp = faren_to_cel(ftemp);
            println!("The converted Temperature in Celsius is: {}", ctemp);
        },
        'C' => {
            let mut temp = String::new();
            println!("Enter the Temperature in Celsius: ");
            stdin().read_line(&mut temp).unwrap();
            let ctemp: f64 = temp.trim().parse().expect("Enter Temperature in with decimal places.");
            let ftemp = faren_to_cel(ctemp);
            println!("The converted Temperature in Farenheit is: {}", ftemp);
        },
        _ => panic!("Enter either F or C only")
    }

}

#[cfg(test)]
mod tests {

    use crate::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_c2f(){
        assert_eq!(cel_to_faren(25.0), 77.0);
        assert_eq!(cel_to_faren(55.0), 131.0);
    }
    
    #[test]
    fn test_f2c(){
        // relative_eq!(faren_to_cel(25.0), -3.88889);
        assert_approx_eq!(faren_to_cel(25.0), -3.889, 1f64);
        assert_approx_eq!(faren_to_cel(55.0), 12.7778, 1f64);
    }
}
