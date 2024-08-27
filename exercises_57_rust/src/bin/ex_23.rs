#[allow(unused_imports)]
#[allow(dead_code)]

use std::io::stdin;
use std::io::Read;

#[derive(Debug)]
struct Levels -> Box<(T, U)> {
    yes: (String, Levels),
    no: (String, Levels)
}

#[derive(Debug)]
enum Choice {
    Yes,
    No
}

fn main() {
    let mut ans1 = String::new();    
    let question1 = "Is the car silent when you turn the key on?".to_owned();

    println!("Question 1: {}", question1);
    
    let lvl11choice = Levels {
        yes: "Clean terminal & try starting again",
        no: "replace cables and try again"
    };

    let lvl12choice = Levels {
        yes: "Replace the Battery",
        no: "Does the car crank up and fail to start?"
    };

    let l1choice = Levels {
        yes: ("Are the battery terminals corroded?".to_owned(),
            lvl11choice),
        no: ("Does the car make clicking sound?".to_owned(),
            lvl12choice)
    };

    println!("Your choices are: {:?}", l1choice);

    stdin().read_line(&mut ans1).expect("Choose an option..");

    let question2 = match ans1.trim() == "yes" {
        true => l1choice.yes;
        false => l1choice.no,
    };

    println!("Question 2: {}", question2);

    // depending the choice made above, the 
    // next questions will be provided

    
}









