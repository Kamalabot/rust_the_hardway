#[allow(unused_imports)]
#[allow(dead_code)]

use std::io::stdin;


fn main() {
    let mut ans1 = String::new();    
    let question1 = "Is the car silent when you turn the key on?".to_owned();

    println!("Question 1: {}", question1);
    
    stdin().read_line(&mut ans1).expect("Choose an option..");
    // level 1 yes block 
    if ans1.trim() == "yes"{
        let l1y_question = "Are the battery terminals corroded?".to_owned();
        println!("Next Question: {}", l1y_question);
        let mut ansl1y = String::new();    
        stdin().read_line(&mut ansl1y).expect("Choose an option..");

        // level1 y->y block
        if ansl1y.trim() == "yes" {
            println!("Clean terminal & try starting again");
        } else {
            println!("replace cables and try again");
        }
    }
    // level 1 no block 
    else {
        let l1n_question = "Does the car make clicking sound?".to_owned();
        println!("Next Question: {}", l1n_question);
        let mut ansl1n = String::new();    
        stdin().read_line(&mut ansl1n).expect("Choose an option..");

        // level1 n-y block
        if ansl1n.trim() == "yes" {
            println!("Replace the Battery");
        // level1 n-n block
        } else {
            println!("Does the car crank up and fail to start?");
            let mut ansl2 = String::new();    
            stdin().read_line(&mut ansl2).expect("Choose an option..");
            //level2 n-n-y block
            if ansl2.trim() == "yes"{
                println!("Check the spark plug connection");
            //level2 n-n-n block
            } else {
                println!("Does the engine start and dies?");
                let mut ansl3 = String::new();
                stdin().read_line(&mut ansl3).expect("Provide yes or no");
                // level3 n-n-n-y block
                if ansl3.trim() == "yes" {
                    println!("Does your car have fuel injection?");
                    let mut ansl4 = String::new();
                    stdin().read_line(&mut ansl4).expect("Make a choice");
                    // level 4 n-n-n-y-y block
                    if ansl4.trim() == "yes" {
                        println!("Ensure the choke is opening and closing");
                    } else {
                        println!("Get it in for servicing... End of line");
                    }
                // level3 n-n-n-n block not present
                }
            }
        }
    }
        
}









