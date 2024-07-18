mod front_of_house {
    // mod hosting { // default it is private
    // top modules cannot see what is inside the 
    // child modules / function
    pub mod hosting {
        fn add_waitlist() {}

        // fn seat_at_table() {}
        pub fn seat_at_table() {}
    }

    mod dining {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_hotel() {
    // abs path
    crate::front_of_house::hosting::seat_at_table()
    // rel path
    front_of_house::hosting::seat_at_table();
}
// this is at crate level
fn server_order() {}

mod back_of_house{
    // struct Breakfast{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast{
        // fn summer(toast: String) -> Breakfast{
        pub fn summer(toast: String) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: "Peaches".to_owned()
            }
        }
    }
    pub enum Appetizer{
        Soup,
        Salad
    }
    fn fix_incorrect_order() {
        cook_order(); // visible
        super::server_order(); // need to look thru crate(super)
    }
    fn cook_order() {}
}

// two ways to import modules
use crate::back_of_house::Breakfast;
use self::back_of_house::Appetizer;

pub fn eat_at_table() {
    let mut meal = Breakfast::summer("Rye".to_owned());
}

pub fn eat_appetizer() {
    let mut apt = Appetizer::Soup;
}

// rename the imports
use std::fmt:Result;
use std::io::Result as IoResult;

// adding rand to the module
// nested imports
use rand::{Rng, CryptoRng, ErrorKind::Transient}