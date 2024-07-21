#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    pub fn area(&self) -> i32{
        self.width * self.height
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        other.height < self.height && other.width < self.width
    }
}

pub fn greet_name(name: String) -> String {
    format!("Greetings {}", name)
}

pub struct Guess {
    guess: i32
}

pub fn func_that_panics(guess: i32) -> Guess {
    // if guess < 0 || guess > 100 {
    if guess < 0 {
        panic!("Raise havok in the test cases...");
    }
    Guess { guess }
}
