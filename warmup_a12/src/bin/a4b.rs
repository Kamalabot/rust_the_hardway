// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:

fn main() {
    // * Use a variable set to any integer
    let inte: i32 = 6;
    // * Use a match expression to determine which message to display
    match inte {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // * Use an underscore (_) to match on any value
        // trying to match it with a var and print it
        o => println!("Its o: , {:?}", o),
    }
}
