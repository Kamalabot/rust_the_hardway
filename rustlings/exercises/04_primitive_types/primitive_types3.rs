fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [0; 101]; // learnt about this, no its just 101 0 
    for elem in a{
        let temp = format!("{:?}", elem);
        println!("{}", temp);
    }
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
