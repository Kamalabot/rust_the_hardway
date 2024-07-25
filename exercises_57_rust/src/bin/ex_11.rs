#[allow(dead_code)]
use std::io::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
// logic calls for base_currency rate, rate_from has to be considered
// as 1
// getting the json with the exchange rates will require the app_id
// so decided to store it in env and extract it.
fn retrieve_json(){
    let mut url = String::from("https://openexchangerates.org/api/latest.json?");
    let mut app_id = String::new();
    println!("Enter your app_id to complete the request: ");
    stdin().read_line(&mut app_id).unwrap();
    // push the app_id back into url
    let app_id = format!("app_id={app_id}");
    url.push_str(&app_id);
    println!("the full url is {url}");
}
fn main() {
    let mut amount_from = String::new(); 
    let mut rate_from = String::new();
    let mut rate_to = String::new();
    // println!("Provide the amout in Euro: ");
    // stdin().read_line(&mut amount_from).unwrap();
    // println!("Provide the exchange rate from Euro: ");
    // stdin().read_line(&mut rate_from).unwrap();
    // println!("Provide the exchange rate to USD: ");
    // stdin().read_line(&mut rate_to).unwrap();
    // // parse the data into numbers for calculation
    // let amount_from: f64 = amount_from.trim().parse().unwrap();
    // let rate_from: f64 = rate_from.trim().parse().unwrap();
    // let rate_to: f64 = rate_to.trim().parse().unwrap();
    // let amount_to = (amount_from * rate_from) / rate_to;
    // println!("The amount in USD in for {amount_from} Euro is {amount_to}");
    retrieve_json();
}