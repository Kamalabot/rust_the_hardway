#[allow(unused_imports)]

pub struct User {
    pub name: String,
    pub age: u8,
}

impl User {
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

pub fn idiomatic1() {
    // parsing string as integer
    let s = "10";
    let n: Option<i32> = s.parse().ok();
    println!("n: {:?}", n);

    println!("Another way to extract option data");
    let opt: Option<&str> = Some("New String");
    let opt_str = opt.map(|x| x.to_uppercase()).unwrap();
    // applies function to Some(value) contained
    // or returns None
    println!("Extracting something from Opt_str: {}", opt_str);

    let vec1: Vec<i32> = vec![66, 52, 32, 64];
    // map on vectors, using the into_iter()
    // and taking ownership
    // just iter() barrows the vector and uses
    // it for processing
    let nvek1: Vec<i32> = vec1.into_iter().map(|x| x * 2).collect();
    println!("Owned nvek1: {:?}", nvek1);
    // println!("try printing vec1: {:?}", vec1)
    let vec2: Vec<i32> = vec![66, 52, 32, 64];
    // even for filtering we have to take ownership
    let filvek: Vec<i32> = vec2.into_iter().filter(|&x| x % 2 == 0).collect();
    let vec3: Vec<i32> = vec![66, 52, 32, 64];
    // even for inspecting have to take ownership
    let insvek: Vec<i32> = vec3
        .into_iter()
        .inspect(|x| println!("inspect_ {}", x))
        .filter(|&x| x % 2 == 0)
        .collect();
    // extracting from option,
    let opt2: Option<i32> = Some(56);
    match opt2 {
        Some(val) => println!("Val is: {}", val),
        None => println!("Nothing.."),
    }
    // let opt2 = Some(86);
    let opt3 = opt2.and_then(|x| {
        if x % 2 == 0 {
            println!("Using up...");
            Some(x)
        } else {
            None
        }
    });
    println!("Extracting opt3 {:?}", opt3);

    let st: &str = "63";
    match st.parse::<i32>() {
        Ok(n) => println!("Parsed: {}", n),
        Err(_) => println!("Parse fail"),
    }

    println!("Iterators for sum");
    let mut vec4: Vec<i32> = vec![66, 52, 32, 64];
    let s: i32 = vec4.iter().sum();
    println!("Sum: {}", s);
    vec4.sort(); // cannot directly push into println!
    println!("{:?}", vec4);
    let strvec: Vec<&str> = vec!["50", "ath", "65"];
    // joining vector elms
    // vec4.join(','), trait bounds on i32 will bail out
    let joined: String = strvec.join(",");
    println!("Joined: {}", joined);

    // cannot do the below, and try assigning
    // let res_vec: Result<Vec<&str>, _> =
    //     Result(Ok(vec!["5", "6", "8", "7"]), Err("something is wron"));

    // let res_vec: Result<Vec<&str>, &str> = Ok(vec!["5", "6", "8", "7"]);

    // let out_res = res_vec;

    // let proc_out: Result<Vec<&str>, String> = out_res
    //     .into_iter()
    //     .map(|x| x.parse::<i32>().map_err(|_| "invalid_input".to_owned()))
    //     .collect();
    // println!("Output Res: {:?}", proc_out);
    // can the map_err take a ParseIntError
    // nope it will error
}

use std::collections::HashMap;

pub fn collection_galore() {
    let mut logs = HashMap::new();
    logs.insert("Alice".to_string(), 2);
    logs.insert("Bot".to_string(), 12);
    logs.insert("Tim".to_string(), 72);
    println!("Storing data in HM: {:?}", logs);
}
pub fn idioming_types() {
    let user = User::new("loc", 56);
    println!("is adult: {}", user.is_adult());
    let card = PaymentMethod::CreditCard("5678".to_owned());
    println!("{}", card.details());
}

use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

pub fn read_file_content(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// managing state with enum
// placing and extracting data from enum
enum PaymentMethod {
    CreditCard(String),
    PayPal(String),
}

impl PaymentMethod {
    pub fn details(&self) -> String {
        match self {
            PaymentMethod::PayPal(email) => format!("{}", email),
            PaymentMethod::CreditCard(card) => format!("{}", card),
        }
    }
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
