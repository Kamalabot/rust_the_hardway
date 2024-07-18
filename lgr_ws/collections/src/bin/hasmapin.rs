use std::collections::HashMap;

fn main(){
    let s1 = String::from("Make");
    let s2 = String::from("True");

    let mut hm = HashMap::new();
    hm.insert(s1, 10);
    hm.insert(s2, 25);
    let key = String::from("True");

    let score = hm.get(&key);
    match score {
        Some(val) => println!("Score is {}", val),
        _ => println!("Couldn't get the key")
    } 

    let mut newScore = HashMap::new();

    newScore.insert(String::from("Blue"), 20);
    newScore.insert(String::from("Blue"), 60);
    
    let h = newScore.entry(String::from("Yellow")).or_insert(25);
    newScore.entry(String::from("Yellow")).or_insert(75);
    let latest = String::from("Blue");
    let latest1 = String::from("Yellow");
    let newtest = newScore.get(&latest);
    let newtest1 = newScore.get(&latest1);
    match newtest {
        Some(val) => println!("{}", val),
        _ => println!("Not found")
    }
    match newtest1 {
        Some(val) => println!("{}", val),
        _ => println!("Not found")
    }
    let statement = "World is Hello wonderful world";
    let mut worldmap = HashMap::new();

    for text in statement.split_whitespace(){
        let cnt= worldmap.entry(text).or_insert(0);
        *cnt += 1;
    }
    println!("{:?}", worldmap);
}