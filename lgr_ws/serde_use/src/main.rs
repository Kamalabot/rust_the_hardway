#[allow(dead_code)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point{
    x: i32,
    y: i32
}
fn main() {
    println!("Hello, world!");
    let point_ser = Point{x: 1, y: 5};
    let serialized_point = serde_json::to_string(&point_ser).unwrap();
    println!("Serialize = {}", serialized_point);
    let deserialized: Point = serde_json::from_str(&serialized_point).unwrap();
    println!("Deserialized: {:?}", &deserialized);
}
