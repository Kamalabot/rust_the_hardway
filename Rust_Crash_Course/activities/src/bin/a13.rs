// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:

fn main() {
    // * Use a vector to store 4 numbers
    let str_num = vec![10, 20, 30, 40];
    // * Iterate through the vector using a for..in loop
    for elem in &str_num{
        // * Determine whether to print the number or print "thirty" inside the loop
        match elem {
            10 => println!("{}", 10), 
            20 => println!("{}", 20), 
            30 => println!("{}", "thirty"), 
            40 => println!("{}", 40),
            _  => println!("{}", "other") 
        }
    }
    // * Use the .len() function to print the number of elements in a vector
    print!("The number of elements in str_num is {}", &str_num.len());
}
