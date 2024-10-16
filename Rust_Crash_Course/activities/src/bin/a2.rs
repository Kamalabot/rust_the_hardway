// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn add_num(in1: i32, in2: i32) -> i32{
    in1 + in2 // semi-colon to return this value..
}
// * Use a function to display the result
fn dis_res(res: i32) {
    println!("The res is : {}", res);
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let a = 587;
    let b = 26;
    let sumval = add_num(a, b);
    dis_res(sumval);
}
