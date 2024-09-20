// create User type with name and age
// impl Display for the user
#![allow(warnings)]
#![allow(unused_imports)]

use std::fmt::Display;

#[derive(Debug)]
pub enum PaymentMethod {
    Card(i32),
    Mail(String),
}

impl Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Card(val) => write!(f, "card details are: {}", val),
            PaymentMethod::Mail(mail) => write!(f, "MAil details are: {}", mail),
        }
    }
}

impl From<&str> for PaymentMethod {
    fn from(value: &str) -> Self {
        if value.contains("@") {
            PaymentMethod::Mail(value.to_owned())
        } else {
            match value.parse::<i32>() {
                Ok(num) => PaymentMethod::Card(num),
                Err(e) => {
                    println!("Invalid number");
                    PaymentMethod::Card(0)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct User0 {
    pub name: String,
    pub age: u8,
}

impl User0 {
    pub fn new(name: String, age: u8) -> Self {
        User0 { name, age }
    }
    pub fn driving_age(&self) -> bool {
        // if the person meeting driving age
        if self.age > 18 {
            true
        } else {
            false
        }
    }
}

impl Display for User0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User name: {}, age: {}", &self.name, &self.age)
    }
}

pub fn use_idioms() {
    // parse &str to i32
    let num_s = "356";
    let num: i32 = num_s.parse().unwrap_or(32);
    println!("The num is {}", num);
    // extract the Some value from Option
    let mut argv = std::env::args();
    let opt1 = argv.next();
    // use ok and and_then
    let val1 = opt1.map(|f| println!("{}", f));
    // println!("Value is {}", val1)
    //
    // declare a vector of &str
    let vec_str = vec!["tri", "dos", "uno"];
    // declare a vector of i32
    let vec_i32 = vec![3, 2, 1];
    //
    // use map on the elements and process
    let count: Vec<usize> = vec_str.iter().map(|x| x.len()).collect();
    println!("Count of strings are: {:?}", count);
    //inspect and filter methods
    let c_ins: Vec<usize> = vec_str
        .iter()
        .inspect(|x| println!("value is: {}", x))
        .map(|x| x.len())
        .collect();
    // do the sum
    let sum_ins: usize = c_ins.iter().sum();
    println!("Sum of all the values are: {}", sum_ins);
    // join the vec
    let joinvec: String = vec_str.join(",");
    //
    println!("Joined string is: {}", joinvec);
    // implement methods for the types
    let us2 = User0::new("test1".to_owned(), 26);
    println!("{}", us2.driving_age());
    // implement PaymentMethod type
    let p1 = PaymentMethod::Card(125677);
    let p2 = PaymentMethod::Mail("your@id.com".to_owned());
    println!("Payment p1: {}", p1);
    println!("Payment p2: {}", p2);
    let p3: PaymentMethod = "5678".into();
    let p4: PaymentMethod = "loc@ne.in".into();
    println!("{}, {}", p3, p4)
}
