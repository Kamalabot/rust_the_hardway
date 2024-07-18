use unicode_segmentation::UnicodeSegmentation;
fn main(){
    let mut s1 = String::new();
    let s2 = "sliced string";
    let s3 = String::from("From Maxed");
    let s4 = s2.to_string();

    println!("{} {} {} {}", &s1, s2, s3, s4);

    s1.push_str(" entering more...");
    s1.push('G');

    println!("{}", &s1);

    let s5 = format!("{} {}", s2, s4);

    println!("{}", s5);

    // cannot extract the single char using the array idx
    // &s5[3] is not acceptable

    let hello = String::from("नमस्ते");

    println!("Getting the parts of the string");

    for b in "नमस्ते".bytes(){
        println!("{}", b);
    }
    for b in "नमस्ते".chars(){
        println!("{}", b);
    }
    for b in "नमस्ते".graphemes(true){
        println!("{}", b);
    }
}