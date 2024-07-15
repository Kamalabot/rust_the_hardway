// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:

fn main() {
    // * Use a mutable integer variable
    let mut w1 = 5;
    // * Use a while statement
    while w1 > 0 {
        // * Print the variable within the while loop
        println!("{}", w1);
        w1 -= 1;
    }
    // * Do not use break to exit the loop
}
