// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn add_num() -> i32{
    let f = 12 + 13;
    f
}
// * Use a function to display the result
fn disp_add(){
    let x: i32 = add_num();
    println!("The value of addition is: {:?}", x);
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    disp_add();
}
