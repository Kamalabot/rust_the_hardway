use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{stdin, Read};

#[derive(Debug, Deserialize, Serialize)]
struct Product {
    name: String,
    price: f64,
    quantity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Products {
    products: Vec<Product>,
}

fn main() {
    println!("Manipulating Json");
    let mut f_obj = File::open("product_data.json").unwrap();
    let mut js_rw = String::new();
    f_obj.read_to_string(&mut js_rw).unwrap();
    let pdt_list: Products = serde_json::from_str(&js_rw).unwrap();
    println!("Product list: {:?}", pdt_list);
    let mut srch_raw = String::new();
    stdin().read_line(&mut srch_raw).unwrap();
    let srch = srch_raw.trim();
    let pdt_maps = pdt_list.products;
    let result = pdt_maps.iter().find(|x| x.name.contains(srch));
    match result {
        Some(val) => println!("Found the product: {:?}", val),
        None => println!("Product not found"),
    }
}
