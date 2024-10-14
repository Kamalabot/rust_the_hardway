fn main() {
    println!("Multiplication table:");
    for idx in 1..=12 {
        for jdx in 1..=12 {
            println!("{} X {} = {}", idx, jdx, idx * jdx);
        }
    }
}
