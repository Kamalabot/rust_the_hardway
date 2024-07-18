use std::fmt::Display;

#[derive(Debug)]
pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn return_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{} by {}", self.author, self. headline)
    }
}

#[derive(Debug)]
pub struct Tweet{
    pub author: String,
    pub content: String,
    pub reply: String,
    pub retweets: u32 
}

struct Pair<T>{
    x:T,
    y:T
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self { x, y}
    }
}
// control to which type the impl matches with 
impl <T: PartialOrd + Display> Pair<T> {
    fn cmp_disp(&self){
        if self.x >= self.y{
            println!("Largest number is {}", self.x);
        } else {
            println!("Largest number is {}", self.y)
        }
    }
}
impl Summary for Tweet{
    fn return_author(&self) -> String {
        format!("{}", self.author)
    }
    // fn summarize(&self) -> String {
        // format!("{} by {}", self.author, self.content)
    // }
}
// Why the traits controls the structs?
    // because we have asked to impl trait for struct

pub trait Summary {
    fn summarize(&self) -> String{
        // used when the struct is not implementing the trait
        format!("THis is a basic update")
    }
    fn return_author(&self) -> String;

}

fn main() {
    println!("Work on Traits");
    let article1 = NewsArticle{
        author: String::from("Author1"),
        headline: String::from("Headline1"),
        content: String::from("Content1"),
    };
    println!("{}", article1.summarize());
    println!("{}", article1.return_author());

    let tweet1 = Tweet{
        content: String::from("content2"),
        author: String::from("author2"), 
        retweets: 6,
        reply: String::from("Reply1")
    };
    println!("{}", tweet1.summarize());
    println!("{}", tweet1.return_author());
}