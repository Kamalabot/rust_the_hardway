#[allow(dead_code)]

use anyhow::Result;  // there is no detail on the objects
// use std::error::Error

fn divide(a: i32, b: i32) -> Result<i32> {
    // we have the custom error taken care
    if b == 0 {
        return Err(anyhow::anyhow!("Ok its busted"));
    }
    let result = a / b;

    Ok(result) 
}

fn divide_raw(a: i32, b: i32) -> Result<i32, String> {
    // we have the custom error taken care
    if b == 0 {
        return Err(String::from("Ok its busted without Anyhow"));
    }
    let result = a / b;

    Ok(result) 
}

fn main() -> Result<()>{ 
   
    let val1 = divide(10, 2)?;
    println!("the result of first divide call: {}", val1);
    
    let val2 = divide(10, 0)?;
    println!("the result of second divide call: {}", val2);

    let val3 = divide_raw(10, 8).unwrap();

    println!("THis is val3 {}", val3);
 
    Ok(()) 

}

fn another_main() {
    match divide_raw(10, 0){
        Ok(d) => println!("the raw error route: {}", d),
        Err(e) => println!("got error: {}", e)
    }
}


// Purpose of the Error Trait:
// Standardization: The Error trait defines a standard interface that
// all error types should implement. This makes it easier to work
// with errors in a consistent way across different libraries and
// parts of your application.

// Rich Error Information: While a string provides a simple message,
// the Error trait can be used to convey richer information.
// For instance, an error type can include additional context,
// such as the underlying cause of the error, error codes, or
// other metadata.

// Chaining Errors: The Error trait supports chaining errors, where
// one error can be the cause of another. This is useful for tracing
// the root cause of complex errors, especially when working with
// multiple layers of abstraction (e.g., file handling errors leading
// to higher-level application errors).

// Interoperability: By implementing the Error trait, your custom
// error types can easily interact with other parts of the Rust
// ecosystem that expect Error trait objects, such as the anyhow 
// crate or when converting between error types using the ?
// operator.

// Implementing the Error Trait:

// By implementing the Error trait, our custom error type
// becomes compatible with Rust's error handling ecosystem.
// This allows for more complex error handling strategies like
// error chaining.



