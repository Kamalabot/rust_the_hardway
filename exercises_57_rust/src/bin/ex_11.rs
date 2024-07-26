#[allow(dead_code)]
#[allow(unused_imports)]

use std::io::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;
// logic calls for base_currency rate, rate_from has to be considered
// as 1
// getting the json with the exchange rates will require the app_id
// so decided to store it in env and extract it.

/// reads the env files and returns the APP_ID for using in 
/// exchange rate extraction
fn read_env_get_appid() -> String{
    dotenv().expect("No .env files were found...");
    // env::vars() will return a hashmap, so just looping the map
    for (key, val) in env::vars(){
        if key == "APP_ID"{
            return val;
        }
    }
    "not found".to_owned()
}

fn main() {
    // read the .env file file or update failure
    let app_id = read_env_get_appid();
    println!("Printing the app_id: {app_id}");

    let mut amount_from = String::new(); 
    let mut rate_from = String::new();
    let mut rate_to = String::new();
    println!("Provide the amout in Euro: ");
    stdin().read_line(&mut amount_from).unwrap();
    println!("Provide the exchange rate from Euro: ");
    stdin().read_line(&mut rate_from).unwrap();
    println!("Provide the exchange rate to USD: ");
    stdin().read_line(&mut rate_to).unwrap();
    // parse the data into numbers for calculation
    let amount_from: f64 = amount_from.trim().parse().unwrap();
    let rate_from: f64 = rate_from.trim().parse().unwrap();
    let rate_to: f64 = rate_to.trim().parse().unwrap();
    let amount_to = (amount_from * rate_from) / rate_to;
    println!("The amount in USD in for {amount_from} Euro is {amount_to}");
}