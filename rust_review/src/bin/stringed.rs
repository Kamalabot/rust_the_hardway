fn main() {
    // growable string
    let mut str1 = String::new();

    // can insert a Char

    str1.push('M');
    println!("Lets see M: {str1}");

    // lets enter a larger string 
    str1.push_str(" THis comes after M");
    
    println!("Lets see After M: {str1}");

    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = str1.replace("M", "An M");
    println!("Lets see After M: {}", st2);
    
    let st3 = str1.remove(6);
    println!("Lets see the char: {}", st3);
    println!("Remove after string: {}", str1);

    // Create more strings 
    let str4 = String::from("X r b m T f t p");

    let mut v1: Vec<char> = str4.chars().collect();

    println!("lets print the st4 characters: {:?}", v1);
    
    for val in v1.iter(){
        println!("These are vals: {}", val);
    }

    println!("Got the list printing with for looping...");

    v1.sort();
    for val in v1{
        println!("These are vals: {}", val);
    }

    let st4: &str = "Random String";

    let mut st5 = st4.to_string();

    println!("this is a created string...: {}", st5);

    let byte_arr1 = st5.as_bytes();

    println!("this is important: {:?}", byte_arr1);
    // this process was explained by lgr too

    let st6 = &st5[0..6];

    println!("There are str: {}", st6);

    // working on getting the strings added together
    let str6 = String::from("one first");
    let str7 = String::from("Two first");

    // one of the string will be unusable
    let str8 = str6 + &str7;

    println!("Got the concatenated string: {}", str8);
    let int1: u8 = 5;
    let int5: u32 = int1 as u32;

    println!("This is int1: {} and int2: {}", int1, int5);
}













