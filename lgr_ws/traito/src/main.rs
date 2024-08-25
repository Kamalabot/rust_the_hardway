// A trait object is a way to refer to any type that implements a particular
// trait without knowing the concrete type at compile time.

// In Rust, dyn is used to create a trait object.
// When you use dyn Error, you're saying, "I don't care about the exact type of the error, as long as it implements the Error trait." This enables dynamic 
// dispatch, where the specific method implementation is determined at runtime rather than compile time.
//
// Traits are similar to interfaces in other object-oriented languages, like Java or C#.

// They define a set of methods that a type must implement
//
//Defining Shared Behavior:

// Traits allow you to define a common interface for different types. Any type that implements a trait guarantees that it will provide certain behaviors (methods), allowing you to write code that operates on many types generically.
// Generic Programming:

// Traits enable you to write generic functions and types. You can use traits as bounds to specify that a type parameter must implement a particular trait, ensuring that the generic code can call specific methods on that type.
// Polymorphism:

// While Rust doesn't have traditional inheritance, traits enable polymorphism. You can use trait objects (dyn Trait) to create a form of dynamic dispatch, where the exact method implementation is chosen at runtime.
// Extending Types:

// You can use traits to extend existing types with new methods. This is similar to adding extension methods in languages like C#.
// Marker Traits:

// Some traits don’t define any methods and are used to mark types for special treatment by the compiler or libraries. For example, the Send and Sync traits indicate whether a type is safe to transfer across thread boundaries.

// Trait objects are not "implemented" in to he same way that methods are. Instead, they are used to refer to objects that implement a particular trait. They allow for dynamic dispatch, where the method implementation to call is determined at runtime.
//
// The impl Trait for Type 
// syntax is how you actually provide the concrete implementation of the methods defined by the trait for a specific type.

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

// Define the `Perimeter` trait with a method `perimeter`.
trait Perimeter {
    fn perimeter(&self) -> f64;
}

// Define the `ShapeInfo` trait that extends both `Area` and `Perimeter`.
trait ShapeInfo: Area + Perimeter {
    fn info(&self) {
        println!("Area: {}", self.area());
        println!("Perimeter: {}", self.perimeter());
    }
}

// Implement the `Perimeter` trait for `Rectangle`.
impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Implement the `ShapeInfo` trait for `Rectangle`.
impl ShapeInfo for Rectangle {}

// Implement the `Perimeter` trait for `Circle`.
impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Implement the `ShapeInfo` trait for `Circle`.
impl ShapeInfo for Circle {}


// A function that takes a trait object `&dyn Area`.
fn print_area_dyn(shape: &dyn Area) {
    println!("The area is: {}", shape.area());
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

    let rect = Rectangle { width: 10.0, height: 5.0 };
    let circ = Circle { radius: 3.0 };

    rect.info();
    // Output:
    // Area: 50
    // Perimeter: 30

    circ.info();
    // Output:
    // Area: 28.274333882308138
    // Perimeter: 18.84955592153876
    //
    let rect = Rectangle { width: 10.0, height: 5.0 };
    let circ = Circle { radius: 3.0 };
    //  The &dyn Area is a trait object that can hold any type that implements the Area trait.
    //  This allows print_area_dyn to accept both Rectangle and Circle without knowing their concrete types.
    print_area_dyn(&rect as &dyn Area); // The area is: 50
    print_area_dyn(&circ as &dyn Area); // The area is: 28.274333882308138
}
