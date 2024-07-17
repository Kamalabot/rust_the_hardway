use std::collections::HashMap;

#[allow(dead_code)]
fn main(){
    let mut hm1 = HashMap::new();
    hm1.insert(0, "Dosas".to_owned());
    hm1.insert(1, "Vadas".to_owned());
    hm1.insert(2, "Ladoos".to_owned());
    hm1.insert(3, "Idlies".to_owned());
    hm1.insert(4, "Idiyappam".to_owned());

    let get_val = hm1.get(&3);
    println!("{:?}", get_val);
    match get_val {
        Some(val) => println!("{:?} is the value", val), 
        _ => println!("Key not located..."),
    }
    let get_val1 = hm1.get(&5);
    println!("{:?}", get_val1);
    match get_val1 {
        Some(val) => println!("{:?} is the value", val), 
        _ => println!("Key not located..."),
    }

    for (idx, val) in hm1.iter(){
        println!("{} is key and value is {}", idx, val);
    }
    
    for idx in hm1.keys(){
        println!("{} is key ", idx);
    }
    for val in hm1.values(){
        println!("{} is value ", val);
    }
}