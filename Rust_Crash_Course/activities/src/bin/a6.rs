// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:

fn main() {
    // repetition using while 

    // * Use a mutable integer variable
    let mut wd1 = 5;
    // * Use a while statement
    // * No need to use break to exit the loop
    while wd1 > 0{
    // * Print the variable within the while loop
        println!("{:?}", wd1);
        wd1 -= 1;
    }
    println!("done");
}
