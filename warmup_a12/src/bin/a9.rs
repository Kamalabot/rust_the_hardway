// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
fn tuple_return() -> (i32, i32){
    (5, 8)
}

fn main() {
    // * Destructure the return value into two variables
    let num: (i32, i32) = tuple_return();
    // * Use an if..else if..else block to determine what to print 
    if num.1 < 5 {
        println!("< 5");
    } else if num.1 > 5 {
        println!("> 5");
    } else {
        println!("=5");
    }
}
