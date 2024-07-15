#[allow(unused_variables)]
// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
fn print_num(in_num: i32){
    match in_num {
        1 => println!("Its one..."),
        2 => println!("Its Two..."),
        3 => println!("Its Three..."),
        4 => println!("Its Four..."),
        o => println!("Its other..."),
    } 
}

fn main() {
    // * Use a mutable integer variable
    let mut m2 = 1;
    // * Use a loop statement
    loop {
        print_num(m2);
       // * Print the variable within the loop statement
        println!("{}", m2);
        m2 += 1;
        if m2 > 4 {
            // * Use break to exit the loop
            break;
        }
    }
}
