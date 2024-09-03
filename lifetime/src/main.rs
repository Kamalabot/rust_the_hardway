#![allow(warnings)]
#[allow(dead_code)]
use std::fs::File;
use anyhow::Result;

mod bounds;

use bounds::{largest, large_with_display};

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    // 'a is lifetime marker
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// below structs are used for complex examples
struct Important<'z> {
    part: &'z str,  // this is holding a ref
    // so need to provide the lifetime param
}

impl<'a> Important<'a> {
    fn level(&self) -> i32 {
        3
    }
} 

fn longest_with_ann<'a, 'b: 'a>(x: &'a str, y: &'b str, c: &str) -> &'a str {
    println!("Announcement: {}", c);
    if x.len() > y.len() {
        x
    } else {
        y // need ensure 'b is as long as 'a by 
        // above b': a' bound
    }
}

fn longest_correct_ann<'a>(x: &'a str, y: &'a str, c: &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() -> Result<(),>{
    let str1 = "long String";
    let str2 = "shorty";

    let result = longest(str1, str2);
    println!("Longest string is: {}", result);
    
    let novel = String::from("Call me Ishmael Some years ago..");
    let splitsent = novel.split(".").next().expect("There is no '.'");
    // next() pulls the first sentence alone
    println!("{:?}", splitsent);

    let ipt = Important { part: splitsent };

    println!("Level is: {:?}", ipt.level());

    let announce = "Here is a lifetime issue";

    let b = longest_correct_ann(str1, str2, announce);

    println!("longest wth announce: {}", b);

    use lifetime::{FileReader, FileProcessor};

    let mut raw = File::open("example.txt")?;

    let mut reader = FileReader { file: &mut raw };

    let content = reader.process()?;

    println!("File Content: {}", content);

    let nums = vec![1, 2, 3, 5, 6];

    println!("The largest is: {}", largest(&nums));

    large_with_display("hellowa", "tner", "not makin' sense");

    Ok(())
}





