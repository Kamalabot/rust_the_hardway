#![allow(warnings)]

use clap::Parser;
use rand::{seq::SliceRandom, thread_rng, Rng};
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
    // building the vec for nums, chars and spl chars
    let mut num_chars = Vec::new();
    let mut spl_chars = Vec::new();
    let mut alpha_chars = Vec::new();
    // Ensure password contains at least one of each category
    for _ in 0..pnum {
        num_chars.push(numbers[rng.gen_range(0..numbers.len())] as char);
    }
    for _ in 0..pspl {
        spl_chars.push(special_chars[rng.gen_range(0..numbers.len())] as char);
    }
    for _ in 0..(plen - pspl - pnum) {
        alpha_chars.push(letters[rng.gen_range(0..numbers.len())] as char);
    }
    alpha_chars.extend(spl_chars);
    alpha_chars.extend(num_chars);
    alpha_chars.shuffle(&mut thread_rng());
    let pass_str: String = alpha_chars.into_iter().collect();

    // Fill the remaining password with random characters from all categories
    // let all_chars = [numbers, letters, special_chars].concat();
    // password.extend(
    //     iter::repeat_with(|| all_chars[rng.gen_range(0..all_chars.len())] as char).take(length - 3),
    // );

    // // Bring all of it together
    // let password = [num_chars.join(""), special_chars.join(""), chars.join("")].concat();

    println!("Final password: {pass_str}");
}
