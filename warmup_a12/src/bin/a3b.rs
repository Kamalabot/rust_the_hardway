// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:

fn main() {
    // * Use a variable set to any integer value
    let xi: i32 = 65;
    // * Use an if..else if..else block to determine which message to display
    if xi < 6{
        // * Use the println macro to display messages to the terminal
        println!("Less than 6");
    } else if xi > 6 {
        // * Use the println macro to display messages to the terminal
        println!("Greater than 6");
    } else {
        // * Use the println macro to display messages to the terminal
        println!("Equal to 6");
    }
}
