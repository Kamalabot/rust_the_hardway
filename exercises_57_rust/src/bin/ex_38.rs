use clap::Parser;
use rand::{thread_rng, Rng};
use std::iter;

#[derive(Parser, Debug)]
#[command(about, long_about, version = None)]
struct PGen {
    /// Length of password required
    length: i32,
    /// How many spl characters
    spl_char: i32,
    /// Count of numericals in the passwd
    nums: i32,
}
fn main() {
    println!("Password generator...");
    let cli = PGen::parse();
    let plen = cli.length;
    let pspl = cli.spl_char;
    let pnum = cli.nums;
    let mut rng = thread_rng();

    // Define character pools
    let numbers = b"0123456789";
    let letters = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let special_chars = b"!@#$%^&*()_+-=[]{}|;:'\",.<>?/";

    // Ensure password contains at least one of each category
    let mut num_chars = vec![numbers[rng.gen_range(0..numbers.len())] as char; pnum];
    let mut spl_chars = vec![special_chars[rng.gen_range(0..numbers.len())] as char; pspl];
    let mut chars = vec![letters[rng.gen_range(0..numbers.len())] as char; (length - pnum - pspl)];

    // Fill the remaining password with random characters from all categories
    let all_chars = [numbers, letters, special_chars].concat();
    password.extend(
        iter::repeat_with(|| all_chars[rng.gen_range(0..all_chars.len())] as char).take(length - 3),
    );

    // Bring all of it together
    let password = [num_chars.join(""), special_chars.join(""), chars.join("")].concat();

    // Collect into a String
    let fin: String = password.into_iter().collect();

    println!("Final password: {fin}");
}
