// Explanation:
// println! is used to print formatted text to the console, followed by a newline.
// It’s a simple yet very commonly used macro in Rust.
// How It’s Coded:
// println! uses the format_args! macro
// internally to generate a string and then sends it to the standard output with a newline.

macro_rules! println {
    ($($arg:tt)*) => (
        print!("{}\n", format_args!($($arg)*));
    )
}

// Explanation:
// vec! is used to create a Vec (vector) with a given list of elements.
// It’s a shorthand for initializing vectors in Rust.
// How It’s Coded:
// The vec! macro can create a vector with specified elements or with a repeated value.

macro_rules! vec {
    ($($x:expr),*) => ( // For a list of elements
        <[_]>::into_vec(Box::new([$($x),*]))
    );
    ($elem:expr; $n:expr) => ( // For a repeated element
        $crate::vec::from_elem($elem, $n)
    );
}

// Explanation:
// format! is used to create a formatted string without printing it. It works like
// println! but returns the formatted string.
// How It’s Coded:
// format! also uses format_args! to generate a string, but it returns the string instead of printing it.

macro_rules! format {
    ($($arg:tt)*) => (
        format_args!($($arg)*).to_string()
    )
}

// assert! checks if a condition is true. If it’s not, the macro panics.
// assert_eq! checks if two expressions are equal. If they aren’t, it panics.
// assert_ne! checks if two expressions are not equal. If they are, it panics.
// How They’re Coded:
// These macros are essential for testing and validating assumptions in code.
//
macro_rules! assert {
    ($cond:expr) => {
        if !$cond {
            panic!("assertion failed: {}", stringify!($cond));
        }
    };
}

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("assertion failed: `(left == right)`\n  left: `{}`,\n right: `{}`", $left, $right);
        }
    };
}

macro_rules! assert_ne {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("assertion failed: `(left != right)`\n  left: `{}`,\n right: `{}`", $left, $right);
        }
    };
}

// macro_rules! is the macro used to define other macros in Rust. It allows you to create custom macros with flexible patterns.
// How It’s Coded:
// macro_rules! itself is a macro that lets you match various patterns and expand them into code.
// Define a simple macro to add two numbers
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    let sum = add!(5, 10);
    println!("Sum: {}", sum);
}

// Explanation:
// derive is a macro that automatically implements certain traits for a struct or enum, such as Debug, Clone, Copy, Eq, and PartialEq.
// How It’s Coded:
// It’s typically used with the #[derive] attribute to automatically generate implementations.

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

    

fn main() {
    println!("Hello, world!"); // Simple message
    let name = "Alice";
    println!("Hello, {}!", name); // Formatted message
    //
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    
    println!("{:?}", p1);
    assert_eq!(p1, p2);
}
