#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use reqwest::blocking::Client;
use dotenvy::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
struct ExchangeRates {
    disclaimer: String,
    license: String,
    timestamp: i32,
    base: String,
    rates: HashMap<String, f64>
}
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
    let ex_client = Client::new();
    let app_id = read_env_get_appid(); 
    let mut base_url = String::from("https://openexchangerates.org/api/latest.json?app_id=");
    base_url.push_str(&app_id);
    println!("full url is: {}", base_url);
    // trying to get the response
    let resp_content = ex_client.get(base_url).send().unwrap().text().unwrap();
    // println!("{}", resp_content);
    let deserialized_content: ExchangeRates = serde_json::from_str(&resp_content).unwrap(); 
    println!("Deserialized content: {:?}", deserialized_content);
}