// Explanation:
// Cow<T> stands for "Clone on Write" and is an enum that represents either a reference to immutable data or an owned copy of the data. It is used to optimize situations where you want to avoid cloning data until absolutely necessary.
// Cow can be used with types like str and Vec, and it provides a way to handle immutable and mutable states efficiently.
// Challenges Solved:
// Efficient Cloning: Avoids unnecessary cloning by using a reference as long as possible and only cloning when the data needs to be modified.
// Performance: Helps in cases where you might be working with data that is frequently read but occasionally modified
use std::borrow::Cow;

fn main() {
    let s = "Hello, world!".to_string(); // owned data
    let cow1: Cow<str> = Cow::Borrowed("Hello, world!"); // borrowed data
    let cow2: Cow<str> = Cow::Owned(s); // owned data

    // Using Cow to work with both borrowed and owned data
    println!("cow1: {}", cow1);
    println!("cow2: {}", cow2);

    let mut data = Cow::Borrowed("Hello, world!");
    let mut_data = data.to_mut(); // Clones the data if it’s currently borrowed
    mut_data.push_str(" How are you?");
    println!("Modified data: {}", mut_data);
}
