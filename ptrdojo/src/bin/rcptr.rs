// Explanation:
// Rc<T> stands for "Reference Counted" and is a
// smart pointer used for shared ownership
// of data. Multiple Rc instances can own the
// same data, and the data is only deallocated
// when the last Rc reference is dropped.
// Single-Threaded: Rc is designed for single-threaded
// scenarios. It is not safe to use across threads.
// Challenges Solved:
// Shared Ownership: Allows multiple parts of a program
// to own and access the same data without needing to copy it.
// Immutability: Provides shared ownership while ensuring that the data remains immutable.
//
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);

    let a = Rc::clone(&data); // `a` and `data` share ownership
    let b = Rc::clone(&data); // `b` and `data` share ownership
    
    println!("data: {:?}", data);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

