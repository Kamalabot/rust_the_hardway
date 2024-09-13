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
use std::fmt::{self, Display};

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Value: {} left: {:?} right: {:?}",
            &self.value, &self.right, &self.right
        )
    }
}

fn main() {
    let f = 5;
    // when allocating the f variable a copy
    // of the value is sent to the heap
    let mut x = Box::new(f); // Allocate an integer on the heap
                             // unable to add the integer to box
                             // x = *x + 1;
                             // x = x + Box::new(1);
    println!("x: {}", x);
    *x += 2;

    println!("x: {}", x);

    println!("Trying to print f: {:?}", f);

    let y = Box::new(vec![1, 2, 3]); // Allocate a vector on the heap
    println!("y: {:?}", y);

    let outbox = add_to_box(x);

    println!("Outbox: {}", outbox);
    let treenode = TreeNode::new(15, None, None);
    println!("Trying to debug print TreeNode: {:?}", treenode);
    println!("Trying to display print TreeNode: {}", treenode);
}

fn add_to_box(invar: Box<i32>) -> i32 {
    // invar + 2
    *invar + 2
}
// cannot access the value of x inside
// below function
// fn add_to_x_box() {
//     *x + 1
// }
