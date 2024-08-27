#[allow(dead_code)]

// if let statement is useful for handling 
// Option and Result types, w/o full match
// 
// while let is similar to above, but repeatedly 
// executes the block, as long as pattern matches, 
// useful in matching upto a None is met
//
fn main() {
    println!("if let in action");
    
    let some_option = Some(5);

    if let Some(value) = some_option {
        println!("The value is {}", value);
    } else {
        println!("No value is present");
    }

    let mut max_nums = Some(10);

    while let Some(num) = max_nums {
        println!("What is the current: {}", num);
        // do more work if required
        // reset maxNums 
        max_nums = if num > 0 { 
            Some(num - 1) 
        } else { 
                None 
            };
       // once it is None, loop exits
    } 
    // Match statement is exhaustive pattern matching

    match some_option {
            Some(opt_val) => println!("Val is: {}",
                                        opt_val),
            None => println!("No value found")
    } 

    // combining if let to be like match statement
    if let Some(val) = some_option {
        println!("if let multiple times: {}", val);
    // } else if let None = some_option {
    } else if some_option.is_none() {
        println!("that is error");
    }
    
    // combining if let to be like match statement
    if let Some(val) = some_option {
        println!("Single check execute: {}", val);
    // } else if let None = some_option {
    } else {
        println!("no question asked else...");
    }
    
    // let for destructing tuples

    let (a, b) = (1, 3);
    println!("a: {}, and b: {}", a, b);

}
