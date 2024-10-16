use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Deserialize, Serialize)]
struct Astronaut {
    craft: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct People {
    people: Vec<Astronaut>,
}

fn main() {
    println!("Who is in Space");
    let client = Client::new();
    let url = "http://api.open-notify.org/astros.json";
    let resp = client.get(url).send().unwrap().text().unwrap();
    let ppl_obj: People = from_str(&resp).unwrap();
    println!("{:?}", ppl_obj);
    println!("Name     |Craft");
    for ppl in ppl_obj.people.iter() {
        println!("{}   |{}", ppl.name, ppl.craft);
    }
}
