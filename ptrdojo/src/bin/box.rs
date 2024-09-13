// Explanation:
// Box<T> is a smart pointer that provides
// ownership of a value allocated on the heap.
// It is used when you need a single owner
// of data and the size of the data is not known
// at compile time or is too large to be stored on the stack.
// Ownership: Box<T> provides unique ownership of the data it points to.
// When the Box goes out of scope, it automatically deallocates the heap memory.
// Challenges Solved:
// Heap Allocation: Handles large data that doesn’t fit on the stack.
// Recursive Types: Allows the creation of recursive types where the size of the type is not known at compile time.
// Box::new(y) creates a Box that owns the value y. It doesn’t just cut a reference to y; it moves the value into the heap.

fn main() {
    let x = Box::new(5); // Allocate an integer on the heap
    println!("x: {}", x);

    let y = Box::new(vec![1, 2, 3]); // Allocate a vector on the heap
    println!("y: {:?}", y);
}
