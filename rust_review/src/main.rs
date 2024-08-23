#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[deny(warnings)]

// use std::io::{ Write, BufReader, BufRead, ErrorKind, stdin,};
use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;
use std::fs::File;

fn print_sizes_of_types() {
    println!("Max of usize: {}", usize::MAX);
    println!("Min of usize: {}", usize::MIN);
    println!("Max of u8: {}", u8::MAX);
    println!("Min of u8: {}", u8::MIN);
    println!("Max of i8: {}", i8::MAX);
    println!("Min of i8: {}", i8::MIN);

    println!("Max of u64: {}", u64::MAX);
    println!("Min of u64: {}", u64::MIN);
    println!("Max of i64: {}", i64::MAX);
    println!("Min of i64: {}", i64::MIN);
    println!("Max of f64: {}", f64::MAX);
    println!("Min of f64: {}", f64::MIN);
    println!("Max of f32: {}", f32::MAX);
    println!("Min of f32: {}", f32::MIN);
}

fn say_hello() {
    println!("This is nice!!!")
}

fn get_sum(x: i32, y: i32) {
    println!("{:?} added to {:?} = {}", x, y, x + y)
}


fn return_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn return_multi(a: u8, b: u8) -> (u8, u8) {
    return (a + b, b);
}

fn return_string(name: String) -> String {
    println!("This is a : {}", name);
    format!("That can be returned: {}", name)
}

fn mutab_string(newstr: &mut String) -> String {
    newstr.push_str("A new string");
    format!("Lets mutate: {}", newstr)
}

fn sum_list(lister: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in lister.iter(){
        sum += &val;
    }
    sum
}

fn main() {
    say_hello();
    get_sum(25, 57);
    let tuple: (u8, u8) = return_multi(8, 9);

    println!("This is a tuple {:?}", tuple);

    let formatted: String = return_string("supernova".to_owned());

    println!("Lets see formating: {:?}", formatted);
   
    // we are expecting it to be mutated 
    let mutated: String = mutab_string(&mut "Super...".to_owned());

    println!("So there goes: {}", mutated);

    let newlister: &[i32] = &[5, 7, 8, 9, 8];

    println!("Lets print the lister: {:?}", newlister);

    let sumed_lister = sum_list(newlister);

    println!("The sum of the list is {}", sumed_lister);

    // from here the things are done in the main...
    
    let mut name = String::new();

    name = "The mutable name is updated to Darksied".to_owned();

    println!("Its fine to print as much we want: {}", name);
    
    // this is a static string
    let greet = "There is a nice car...";
   
    // this process doesn't overwrite the string, it 
    // appends to it
    // stdin().read_line(&mut name)
    //     .expect("there has to be a value");
    
    println!("Lets see the updated name: {}", name.trim());
    
    use std::f32::consts::PI;

    println!("The pi value is : {:?}", PI);
    
    // learning to parse the numbers

    let side = "25";

    let mut side: i16 = side.trim()
                            .parse()
                            .expect("got alphabet... need number");
    println!("So the integer side is: {}", side);
    
    // print_sizes_of_types();

    let num1: f32 = 1.111111;
    // if the number of decimal points exceed 6
    // then warning of excessive precision comes
    
    println!(" This is where math operation are shown...");

    let num1 = 567;
    let num2 = -625;

    println!("num1 + num2 = {}", num1 + num2);
    println!("num1 - num2 = {}", num1 - num2);
    println!("num1 / num2 = {}", num1 / num2);
    println!("num1 * num2 = {}", num1 * num2);
    println!("num1 % num2 = {}", num1 % num2);

    let num1:f32 = 567.0;
    let num2:f32 = -625.0;

    println!("After f32ing the num1 / num2 = {}", num1 / num2);
    

    let randum = rand::thread_rng().gen_range(1..100);

    println!("Randum between 1 and 100: {} ", randum);

    let ctrl = 68;

    if ctrl > 50 {
        println!("THis is more than 50...");
    } else if ctrl > 500 {
        println!("This is too much...");
    } else if ctrl < 10 {
        println!("This is too little.");
    } else {
        println!("This is unknown.");
    }
    // let can_dig = if ctrl >= 18 {
    //     true
    // } else {
    //     false
    // };
    let can_dig = ctrl >= 18;

    println!("Can dig? {}", can_dig);

    println!("Bringing in the Matchers and Movers");

    let matchctl = 6;

    match matchctl {
        1..=18 => println!("this is between 10"),
        21 | 5 => println!("Can be odd"),
        65..=i32::MAX=> println!("THis can be super awesome"),
        _ => println!("Rest is history")
    }
    
    // another ways of comparing
    match matchctl.cmp(&65) {
        Ordering::Equal => println!("Thats easily equal"),
        Ordering::Less => println!("Thats lousily less"),
        Ordering::Greater => println!("Thats nastily big"),
    }

    // --- Lets go Loopy----
    let arr = [1,2, 3, 5,7, 62, 5, 67, 89, 86, 34, 5];
    let mut idx = 0;

    // have to use only usize for the for idx, 
    // other int types don't have indexSlicing trait
    println!("Starting to loop");
    loop {
        if arr[idx] % 2 == 0 {
            idx += 1;
            println!("There is {}", arr[idx]);
            continue;
        }
        if arr[idx] > 9 {
            break;
        }
        idx += 1;
        println!("There is {}", arr[idx]);
    }

    let mut whldx = 0;
    
    while whldx < arr.len() {
        println!("Well i am only {}", whldx);
        whldx += 1;
    }

    for val in arr.iter() {
        println!("lets dance... {val}");
    }

    // Data structures start here
    let newtuple: (u8, String, i64) = (6, 
        "heavy cargo".to_owned(), 867);
    
    let (v0, v2, v4) = newtuple;

    println!("I think v0 : {}", v0);

}





























