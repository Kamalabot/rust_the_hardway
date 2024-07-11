// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
fn print_msg(get_num: i32) {
    let boolvar = if get_num < 100 {
        false
    } else {
        true
    };
    // * Use a match expression to determine which message
    match boolvar {
        true => println!("Its big"),
        false => println!("its small"),
    };
}
//   to print

fn main() {
    let yourval = 157;
    print_msg(yourval);
}
