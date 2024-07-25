#[allow(dead_code)]
#[allow(non_snake_case)]

use reqwest::blocking::{Client, ClientBuilder};
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use std::collection::Hashmap;
#[derive(Serialize, Deserialize, Debug)]
struct Typicode {
    userId: i32,
    id: u64,
    title: String,
    completed: bool 
}
fn main() {
    let my_client = Client::new();
    let test_url = "https://jsonplaceholder.typicode.com/todos/";

    let request_content = my_client.get(test_url).send().unwrap().text().unwrap();

    println!("{}", &request_content);
    // above request content is json, which we will deserialize using serde_json
    // since the response will be a having multiple jsons, 
    let resp_vector: Vec<Typicode> = from_str(&request_content).unwrap();

    println!("{:?}", resp_vector[0]); // printing only one data point..

    let mockoon_url = "http://localhost:3000/sendhere";

    let body_str = "{\"name\":\"Kamal\"}";

    let mockoon_resp = my_client
                                .post(mockoon_url)
                                .body(body_str)
                                .send()
                                .unwrap()
                                .text()
                                .unwrap();
    println!("{}", mockoon_resp);
}