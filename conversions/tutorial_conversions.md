Here’s a comprehensive series of tutorials on **Rust conversions** from basic to advanced use cases, including the use of traits, error handling, and type casting. This tutorial includes 20 examples, all implemented in `lib.rs`, and results demonstrated in `main.rs`.

### Project Structure

```
my_conversion_project/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

### 1. **Cargo.toml**

```toml
[package]
name = "my_conversion_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### 2. **lib.rs**

```rust
use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;

/// Example 1: Using `as` for basic type casting
pub fn basic_as_conversion() {
    let int_val = 10;
    let float_val = int_val as f64;
    println!("Integer: {}, Float: {}", int_val, float_val);
}

/// Example 2: Using `From` for basic type conversion
pub fn from_conversion() {
    let string_val = String::from("Hello, Rust!");
    let str_slice: &str = String::from("Hello, world!").as_str();
    println!("String: {}, Slice: {}", string_val, str_slice);
}

/// Example 3: Using `Into` trait
pub fn into_conversion() {
    let int_val: i32 = 5;
    let float_val: f64 = int_val.into();
    println!("Into Conversion: {}", float_val);
}

/// Example 4: Custom Type with `From` trait
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Celsius {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

pub fn custom_from_conversion() {
    let f = Fahrenheit(100.0);
    let c: Celsius = f.into();
    println!("Fahrenheit to Celsius: {}", c.0);
}

/// Example 5: `TryFrom` for fallible conversions
impl TryFrom<&str> for Celsius {
    type Error = ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let temp = s.parse::<f64>()?;
        Ok(Celsius(temp))
    }
}

pub fn try_from_conversion() {
    match Celsius::try_from("37.0") {
        Ok(c) => println!("Converted to Celsius: {}", c.0),
        Err(_) => println!("Failed to convert string to Celsius"),
    }
}

/// Example 6: Parsing string to integers with `FromStr`
pub fn from_str_conversion() {
    let int_res = i32::from_str("42");
    match int_res {
        Ok(n) => println!("Parsed successfully: {}", n),
        Err(e) => println!("Failed to parse: {}", e),
    }
}

/// Example 7: Using `ToString` and `to_string()`
pub fn to_string_example() {
    let num = 42;
    let string_val = num.to_string();
    println!("Number to String: {}", string_val);
}

/// Example 8: Implementing `Display` for custom type
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

pub fn display_conversion() {
    let point = Point { x: 10, y: 20 };
    println!("Display for Point: {}", point);
}

/// Example 9: Custom error handling with `TryFrom`
#[derive(Debug)]
struct ParseCoordinateError;

impl fmt::Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing coordinates")
    }
}

impl From<ParseIntError> for ParseCoordinateError {
    fn from(_: ParseIntError) -> Self {
        ParseCoordinateError
    }
}

struct Coordinate {
    x: i32,
    y: i32,
}

impl TryFrom<&str> for Coordinate {
    type Error = ParseCoordinateError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParseCoordinateError);
        }
        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;
        Ok(Coordinate { x, y })
    }
}

pub fn custom_tryfrom_error_handling() {
    match Coordinate::try_from("10,20") {
        Ok(coord) => println!("Parsed Coordinate: ({}, {})", coord.x, coord.y),
        Err(e) => println!("Error: {}", e),
    }
}

/// Example 10: `From` for Option to Result
pub fn option_to_result() {
    let some_val: Option<i32> = Some(10);
    let result: Result<i32, &str> = some_val.ok_or("Value not found");
    println!("Option to Result: {:?}", result);
}

/// Example 11: `From` for Result to Option
pub fn result_to_option() {
    let result: Result<i32, &str> = Ok(10);
    let option_val = result.ok();
    println!("Result to Option: {:?}", option_val);
}

/// Example 12: Custom type implementing `FromStr`
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl FromStr for Person {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid format");
        }
        let name = parts[0].to_string();
        let age = parts[1].parse::<u32>().map_err(|_| "Invalid age")?;
        Ok(Person { name, age })
    }
}

pub fn fromstr_custom_type() {
    let person: Person = "Alice,30".parse().unwrap();
    println!("Parsed Person: {:?}", person);
}

/// Example 13: `Into` for automatic type conversion
pub fn automatic_into_conversion() {
    let val: i32 = 10;
    let float_val: f64 = val.into();
    println!("Automatic Into Conversion: {}", float_val);
}

/// Example 14: Error handling in `TryInto`
pub fn try_into_example() {
    let result: Result<u32, _> = "42".parse::<i32>().try_into();
    match result {
        Ok(val) => println!("Parsed u32: {}", val),
        Err(_) => println!("Failed to convert i32 to u32"),
    }
}

/// Example 15: Using `?` operator in conversions
pub fn question_mark_operator() -> Result<(), Box<dyn std::error::Error>> {
    let val: i32 = "42".parse()?;
    println!("Parsed with `?`: {}", val);
    Ok(())
}

/// Example 16: String slices to `String`
pub fn slice_to_string() {
    let s: &str = "hello";
    let string_val: String = s.into();
    println!("String Slice to String: {}", string_val);
}

/// Example 17: Vec to slice conversion
pub fn vec_to_slice() {
    let v = vec![1, 2, 3];
    let slice: &[i32] = &v;
    println!("Vec to Slice: {:?}", slice);
}

/// Example 18: Slicing arrays
pub fn array_slicing() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("Array Slicing: {:?}", slice);
}

/// Example 19: Using `From` and `Into` with tuples
pub fn tuple_conversion() {
    let tuple = (10, 20);
    let point: Point = tuple.into();
    println!("Tuple to Point: {}", point);
}

/// Example 20: Complex conversion with `TryInto`
pub fn complex_conversion() {
    let result: Result<Celsius, _> = "100.0".try_into();
    match result {
        Ok(celsius) => println!("Converted to Celsius: {}", celsius.0),
        Err(e) => println!("Failed to convert: {:?}", e),
    }
}
```

### 3. **main.rs**

```rust
fn main() {
    // Example 1: Basic `as` conversion
    println!("\n--- Example 1: Basic `as` Conversion ---");
    my_conversion_project::basic_as_conversion();

    // Example 2: Using `From` for type conversion
    println!("\n--- Example 2: Using `From` ---");
    my_conversion_project::from_conversion();

    // Example 3: Using `Into` trait
    println!("\n--- Example 3: Using `Into` ---");
    my_conversion_project::into_conversion();

    // Example 4: Custom Type with `From`
    println!("\n--- Example 4: Custom `From` Conversion ---");
    my_conversion_project::custom_from_conversion();

    // Example 5: `TryFrom` for fallible conversions
    println!("\n--- Example 5: `TryFrom` ---");
    my_conversion_project::try_from_conversion();

    // Example 6: Parsing string

 to integers with `FromStr`
    println!("\n--- Example 6: `FromStr` ---");
    my_conversion_project::from_str_conversion();

    // Example 7: Using `ToString`
    println!("\n--- Example 7: `ToString` ---");
    my_conversion_project::to_string_example();

    // Example 8: Implementing `Display`
    println!("\n--- Example 8: `Display` Trait ---");
    my_conversion_project::display_conversion();

    // Continue running other examples similarly...
}
```

This Rust project demonstrates basic-to-advanced use cases of conversions with traits like `From`, `Into`, `TryFrom`, `TryInto`, `FromStr`, and more. Each example shows different forms of conversion, handling errors gracefully, and handling custom types with conversions.
