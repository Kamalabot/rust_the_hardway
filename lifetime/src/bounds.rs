// contains the function that show the usage of 
// lifetime and bounds


// the <T: Ord> states that function any vector 
// where the elements implement Ord trait
pub fn largest<T: Ord>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // start by considering the first elem
    // is the largest
    for item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

// the ann var have to have lifetime of 'a
// and it also implements the Display trait
pub fn large_with_display<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
where T: std::fmt::Display {
    println!("Ann: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// example of Box, Cow, Arc 
trait Animal {
    fn speak(&self);    
        // trait implements speak 
    // on object deriving it
}


struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!!!");
    }
}

fn make_animal() -> Box<dyn Animal> {
    Box::new(Dog)
    // Box pointer provides the single ownership
    // data when it size is unknown, in this case 
    // Dog's size is unknown, so using Box Pointer.

    // So Box<dyn Animal> is used to store any type 
    // that implements animal
}

// using Cow with Bounds
use std::borrow::Cow;

fn modify_string<'a>(input: &'a str) -> Cow<'a, str> {
    // even though the function pulls barrowed ref
    if input.contains(" ") {
        Cow::Owned(input.replace(" ", "_"))
        // Cow converts to owned, based on requirement
    } else {
        Cow::Borrowed(input)
    }
}


// using trait Bounds with Arc
use std::sync::Arc;
use std::thread;

trait AnimalArc: Send + Sync {
    fn speak(&self);
}

struct Cat;

impl AnimalArc for Cat {
    fn speak(&self) {
        println!("Meow!!!");
    }
}

pub fn async_exec() {
    let animal: Arc<dyn AnimalArc + Send + Sync> = Arc::new(Cat);
    // Arc is used for thread safety, and it is similar to 
    // reference cloned
    let animal_clone = Arc::clone(&animal);

    let handle = thread::spawn(move || {
        animal_clone.speak();
    });

    handle.join().unwrap();
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error")
    }
}

impl Error for MyError {}

fn error_prone_function() -> Result<(), Box<dyn Error>> {
    Err(Box::new(MyError))
}

use std::rc::Rc;

struct Bird;

impl Animal for Bird {
    fn speak(&self) {
        println!("Chirp!");
    }
}

pub fn bird_main() {
    let animal: Rc<dyn Animal> = Rc::new(Bird);

    let animal_clone = Rc::clone(&animal);
    
    animal.speak();
    animal_clone.speak(); // Both references work
}




