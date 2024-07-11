#[allow(dead_code)]
#[allow(unused_variables)]
// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
fn return_coords() -> (i32, i32) {
    (5, 6)
}

fn main() {
    // * Destructure the return value into two variables
    let (v1, v2) = return_coords();
    // * Use an if..else if..else block to determine what to print
    if v2 < 5{
        println!("Less than 5");
    } else if v2 > 5 {
        println!("Greater than 5"); 
    } else {
        println!("Equal to 5");
    } 
}
