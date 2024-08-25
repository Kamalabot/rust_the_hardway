fn main() {
    let a = vec![1, 2, 3, 4, 5];
    
    // Example 1: Unnecessary allocation (`vec!` could be replaced by a slice)
    let _b = a.clone(); // Clippy will suggest using a slice instead of cloning the vector.

    // Example 2: Inefficient string manipulation (`to_string` called on a literal)
    let _s = "Hello".to_string(); // Clippy will suggest using the literal directly.

    // Example 3: Using `.collect::<Vec<_>>()` unnecessarily
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect(); // Clippy might suggest using a simpler approach.

    // Example 4: Redundant clone
    let _v = squares.clone(); // Clippy will suggest avoiding the clone here if it's unnecessary.

    // Example 5: Using `unwrap()` on an `Option` or `Result`
    let opt_val = Some(10);
    let _x = opt_val.unwrap(); // Clippy will suggest handling the `None` case or using a safer method.

    // Example 6: Explicit returns at the end of functions
    let sum = add_numbers(5, 10);
    println!("Sum: {}", sum);

    // Example 7: Unnecessary `&` in method call
    let len = get_length(&a);
    println!("Length: {}", len);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    return x + y; // Clippy will suggest removing the `return` keyword for idiomatic Rust.
}

fn get_length(v: &Vec<i32>) -> usize {
    v.len() // Clippy will suggest taking `&[i32]` instead of `&Vec<i32>`.
}
