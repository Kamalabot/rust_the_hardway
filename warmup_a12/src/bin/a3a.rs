// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:

fn main() {
// * Use a variable set to either true or false
    let yb: bool = false;
// * Use an if..else block to determine which message to display
    if yb == true {
        // * Use the println macro to display messages to the terminal
        println!("Hello");
    } else {
        // * Use the println macro to display messages to the terminal
        println!("Goodbye");
    }
}
