#[allow(dead_code())]
fn change_string(name: &mut String){
    name.push_str("Getting something in...");
    println!("Message: {}", name);
}

fn main() {
    // showing how the ownership works in rust
    let mut str1 = String::new();
    str1.push_str("Lets get something in...");
    let str2 = str1;

    println!("Hello copied: {}", str2);
    // original cannot be accessed
    // println!("Hello Original: {}", str1);
    // rustc: borrow of moved value: `str1`
    // value borrowed here after move 

    let str3 = "Lets String3".to_owned();
    println!("Even the printing will barrow: {}", str3); 
    
}


















