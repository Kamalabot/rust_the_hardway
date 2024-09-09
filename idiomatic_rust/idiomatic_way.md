```rust
// Example 1: Parse a string into an integer
// Real-World: Parsing user input as integers.
pub fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_test() {
        assert_eq!(parse_int("42"), Some(42));
        assert_eq!(parse_int("not a number"), None);
    }
}

// Example 2: Using Result to handle errors
// Real-World: Opening a file and managing errors.
use std::fs::File;
use std::io::{self, Read};

pub fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_content_test() {
        let result = read_file_content("Cargo.toml");
        assert!(result.is_ok());
    }
}

// Example 3: Handling Option gracefully with map()
// Real-World: Extracting values from Option and transforming.
pub fn option_to_uppercase(option: Option<&str>) -> Option<String> {
    option.map(|s| s.to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_to_uppercase_test() {
        assert_eq!(option_to_uppercase(Some("hello")), Some("HELLO".to_string()));
        assert_eq!(option_to_uppercase(None), None);
    }
}

// Example 4: Using iterators and map()
// Real-World: Doubling all elements in a vector.
pub fn double_elements(vec: Vec<i32>) -> Vec<i32> {
    vec.into_iter().map(|x| x * 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_elements_test() {
        assert_eq!(double_elements(vec![1, 2, 3]), vec![2, 4, 6]);
    }
}

// Example 5: Using filter on an iterator
// Real-World: Filter out even numbers.
pub fn filter_even(vec: Vec<i32>) -> Vec<i32> {
    vec.into_iter().filter(|&x| x % 2 == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_even_test() {
        assert_eq!(filter_even(vec![1, 2, 3, 4]), vec![2, 4]);
    }
}

// Example 6: Match expression with Option
// Real-World: Return default if Option is None.
pub fn option_with_default(opt: Option<i32>, default: i32) -> i32 {
    match opt {
        Some(val) => val,
        None => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_with_default_test() {
        assert_eq!(option_with_default(Some(5), 10), 5);
        assert_eq!(option_with_default(None, 10), 10);
    }
}

// Example 7: Using and_then for chaining Option
// Real-World: Process Option if it exists.
pub fn increment_if_even(opt: Option<i32>) -> Option<i32> {
    opt.and_then(|x| if x % 2 == 0 { Some(x + 1) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_if_even_test() {
        assert_eq!(increment_if_even(Some(4)), Some(5));
        assert_eq!(increment_if_even(Some(3)), None);
    }
}

// Example 8: Using match for comprehensive error handling
// Real-World: Parsing integers and returning detailed errors.
pub fn parse_integer(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Failed to parse as integer.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_test() {
        assert_eq!(parse_integer("42"), Ok(42));
        assert_eq!(parse_integer("abc"), Err("Failed to parse as integer.".to_string()));
    }
}

// Example 9: Using struct and impl block
// Real-World: Defining a user with methods.
struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: &str, age: u8) -> Self {
        Self { name: name.to_string(), age }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_test() {
        let user = User::new("Alice", 20);
        assert_eq!(user.is_adult(), true);
    }
}

// Example 10: Using Enum for state management
// Real-World: Handling different payment methods.
enum PaymentMethod {
    CreditCard(String),
    PayPal(String),
}

impl PaymentMethod {
    pub fn details(&self) -> String {
        match self {
            PaymentMethod::CreditCard(card) => format!("CreditCard: {}", card),
            PaymentMethod::PayPal(email) => format!("PayPal: {}", email),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payment_method_test() {
        let payment = PaymentMethod::CreditCard("1234".to_string());
        assert_eq!(payment.details(), "CreditCard: 1234");
    }
}

// Example 11: Using iterators for sum
// Real-World: Sum a list of numbers.
pub fn sum_list(vec: Vec<i32>) -> i32 {
    vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_list_test() {
        assert_eq!(sum_list(vec![1, 2, 3, 4]), 10);
    }
}

// Example 12: Using sort method on a vector
// Real-World: Sorting a list of names.
pub fn sort_names(mut names: Vec<&str>) -> Vec<&str> {
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_names_test() {
        assert_eq!(sort_names(vec!["Bob", "Alice", "Eve"]), vec!["Alice", "Bob", "Eve"]);
    }
}

// Example 13: Using HashMap for key-value storage
// Real-World: Store user login attempts.
use std::collections::HashMap;

pub fn track_logins() -> HashMap<String, u32> {
    let mut logins = HashMap::new();
    logins.insert("Alice".to_string(), 3);
    logins.insert("Bob".to_string(), 1);
    logins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_logins_test() {
        let logins = track_logins();
        assert_eq!(logins["Alice"], 3);
        assert_eq!(logins["Bob"], 1);
    }
}

// Example 14: Using iterators for string concatenation
// Real-World: Join names with a delimiter.
pub fn join_names(names: Vec<&str>) -> String {
    names.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_names_test() {
        assert_eq!(join_names(vec!["Alice", "Bob", "Eve"]), "Alice, Bob, Eve");
    }
}

// Example 15: Using map and Result
// Real-World: Parsing and validating multiple inputs.
pub fn parse_list(inputs: Vec<&str>) -> Result<Vec<i32>, String> {
    inputs.into_iter().map(|x| x.parse::<i32>().map_err(|_| "Invalid input".to_string())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_test() {
        assert_eq!(parse_list(vec!["1", "2", "3"]), Ok(vec![1, 2, 3]));
        assert_eq!(parse_list(vec!["1", "a", "3"]), Err("Invalid input".to_string()));
    }
}

// Example 16: Using Result for error propagation
// Real-World: Calculating division with error handling.
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_test() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err("Cannot divide by zero".to_string()));
    }
}

// Continue similarly for Examples 17-25...
```

The above is 16 examples of the idiomatic rust... that needs to be mastered
