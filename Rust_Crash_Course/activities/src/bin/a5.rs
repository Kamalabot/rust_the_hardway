// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut id1 = 1;
    loop {
        println!("{:?}", id1);
        id1 += 1;
        // if id1 >= 5{
        if id1 == 4{
            break;
        }
    }
    println!("Loop is done!!!");
}
