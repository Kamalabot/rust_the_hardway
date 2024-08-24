use std::fmt::format;

#[allow(unused_variables)]

fn main() {
    let try_enter = |ht :i32| {
        if ht > 50 {
            // format!("Cannot Enter...")
            "Cannot Enter...".to_owned()
        } else {
            // format!("There is more you, come in")
            "There is more to you, come in".to_owned()
        }

    };

    println!("Can I enter:? {}", try_enter(5));
    
    // let fn_name = |pram1, pram2| {
    //      body of the function 
    //      that uses the params
    // }
    // the data can be outside also, closures barrow by
    // default
    let mut samp1 = 65;

    let testprint = || println!("This will print: {}", samp1);

    testprint();
    // if you change then it becomes FnMut
    // the fun itself has to be mutable
    let mut chng_print = || {
        samp1 += 1;
        println!("Updated and printed: {}", samp1);
    };
    chng_print();
    
    fn use_fun<T>(a: i32, y: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, y)
    }
    // use_fun is wrapper on top muliple functions that 
    // can use two variables
    let sumator = |a: i32, b: i32| a + b; 
    let productor = |a: i32, b: i32| a * b; 

    let prdt1 = use_fun(6, 10, sumator);
    let prdt2 = use_fun(6, 10, productor);

    println!("The reply for summator function: {}", prdt1);
    println!("The reply for productor function: {}", prdt2);

}















