// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    if num > 5{
        num
    } else {
        num * num
    }
}

fn main() {
    let answer = square(6);
    println!("The square of 3 is {}", answer);
}
