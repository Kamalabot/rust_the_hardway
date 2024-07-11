#[allow(dead_code)]
#[allow(non_camel_case_types)]

enum Access{
    Full,
}

fn one_two_three() -> (i32, i32, i32){
    (1, 2, 3)
}

fn main(){
    // getting it as a tuple
    let nums = one_two_three();
    // unpacking tuplej
    let (x, y, z)= one_two_three();
    println!("{}, {}", x, nums.0);
    println!("{}, {}", y, nums.1);
    println!("{}, {}", z, nums.2);

    // more examples
    let coords = (6, 7);
    let (x, y) = coords;
    println!("X: {}", x);
    println!("Y: {}", y);
    let favs = ("red", "TX", "pizza", "Home");
    let state = favs.1;
    let food = favs.2;
    println!("Fav state: {}", state);
    println!("Fav Food: {}", food);
}