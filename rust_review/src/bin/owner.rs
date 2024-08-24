#[allow(dead_code)]

fn change_string(name: &mut String) -> &mut String{
    name.push_str("Getting something in...");
    println!("Message: {}", name);
    name
}

fn ref_print(sub: &String) {
    println!("This is printing sub String: {}", sub);
}

fn own_print(sub: String) {
    println!("This is printing sub String: {}", sub);
}

fn dbl_vector(in_vec: Vec<i32>) -> Vec<i32> {

    let mut ret_vec = Vec::new();

    for idx in in_vec.iter() {
        // remember you can't change iterators
        println!("{}", idx)
    }
    for mdx in &in_vec {
        ret_vec.push(mdx * 2)
    }
    ret_vec
}




fn main() {
    // showing how the ownership works in rust
    let mut str1 = String::new();
    str1.push_str(" Lets get something in...");
    let str2 = str1;

    println!("Hello copied: {}", str2);
    // original cannot be accessed
    // println!("Hello Original: {}", str1);
    // rustc: borrow of moved value: `str1`
    // value borrowed here after move 

    let mut str3 = "This is Str3.".to_owned();
    println!("Even the printing will barrow: {}", str3); 

    println!("This will print: {}", str3); 
    
    println!("Sending it through a function and change str3");

    let ch_str3 = change_string(&mut str3);
    
    println!("Lets print chStr3: {}", ch_str3);

    println!("You will see str3 is also mutated: {}", str3);    
    let str1 = String::from("World");
    // this moves the str1 to str2
    let srt2 = str1;
    // following will error out
    // println!("Hello {}", str1);

    ref_print(&srt2); // sending the reference, so wont error below
    // now try printing srt2, no error
    println!("No error: {}", srt2);
    own_print(srt2); // sending the reference, so wont error below
    // now try printing srt2, it will error
    // println!("Will error: {}", srt2);

    println!("Lets work on Vectors moving...");

    let vec_one = vec![1, 2, 3, 5, 6, 7, 9];

    let vec_two = dbl_vector(vec_one);
    
    println!("This is vec_two: {:?}", vec_two);

    // following print won't work
    // println!("This is vec_one: {:?}", vec_one);
    
}






















