use std::io;

fn main() {
    println!("Lets guess and error check some numbers");
    println!("Enter a number below: ");
    // need a variable to store the input data
    let mut your_input = String::new();
    // take the input
    io::stdin()
        .read_line(&mut your_input)
        .expect("Need to enter something");
    // convert the string to integer
    // you still need the let here
    let your_input: u32 = your_input
            .trim()
            .parse()
            .expect("Enter a number only");

    if your_input == 42 {
        println!("You know what is life...");
    } else {
        println!("Well try again...");
    }
}
