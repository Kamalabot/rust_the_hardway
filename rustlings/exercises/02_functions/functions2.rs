// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num:i32) -> i32 {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
    let mut x = 0;
    for i in 0..num {
        x += i;
    }
    x
}

fn main() {
    let f = call_me(7);
    println!("Summed up {}", f);
}
