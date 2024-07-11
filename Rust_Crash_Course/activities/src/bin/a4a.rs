// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let mut someval = true;
    // will warn to remove mu-t
    // all the different values someval can have
    // must be accounted for
    match someval {
        true => println!("Nice true"),
        false => println!("hold on, its false..."),
    }
    // someval = false; will raise unused arg
}
