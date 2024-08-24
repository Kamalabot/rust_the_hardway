#[allow(dead_code)]


// struct TreeNode<T> {
//     pub left: TreeNode<T>,
//     pub right: TreeNode<T>,
//     pub key: T,
// }
// rustc: recursive without indirection [E0072]
// rustc: insert some indirection 
// (e.g., a `Box`, `Rc`, or `&`) to break the cycle: `Box<`, `>` [E0072]

struct TreeNode<T> {
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
    pub key: T,
}

impl<T> TreeNode<T> {
    pub fn new(key: T) -> Self {
        TreeNode {
            left: None,
            right: None,
            key,
        }
    }

    pub fn left(mut self, node: TreeNode<T>) -> Self{
        self.left = Some(Box::new(node));
        self
    }
    
    pub fn right(mut self, node: TreeNode<T>) -> Self{
        self.right = Some(Box::new(node));
        self
    }
}

fn main() {
    // Box pointer used when have a large data
    // and it is stored in the heap
    //
    let b_int1 = Box::new(10);

    println!("What is in the box b_int: {}", b_int1);
    
    let rootnode = TreeNode::new(1)
        .right(TreeNode::new(2))
        .left(TreeNode::new(3));


}
