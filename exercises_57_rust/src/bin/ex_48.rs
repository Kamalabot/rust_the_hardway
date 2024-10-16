use reqwest::get;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Astronaut {
    craft: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct People {
    people: Vec<Astronaut>,
}

fn main() -> Result<(), Error> {
    println!("Who is in Space");
    let url = "http://api.open-notify.org/astros.json";
    let resp = get(url)?.json();
    println!("{:?}", resp);
}
