// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a function to print the messages
fn print_msg(in_bool: bool) {
    match in_bool {
    // * Use a match expression to determine which message
        true => println!("<= 100"),
        //  to print
        false => println!("> 100"),
    }
}

fn main() {
// * Use a boolean variable set to the result of
    let ybol: i32 = 100;
//   an if..else expression to store whether the value
    let y_res: bool;
//   is > 100 or <= 100
    if ybol <= 100 {
        y_res = true;
    } else {
        y_res = false;
    }
    print_msg(y_res);
}
