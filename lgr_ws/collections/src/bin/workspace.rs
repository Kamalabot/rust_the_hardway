#[allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug)]
struct Marks{
    english: i32,
    math: i32,
    chem: i32
}

impl Marks {
    fn new(english: i32, math: i32, chem: i32) -> Self{
        Self {
            english,
            math,
            chem
        }
    }
    fn total_marks(&self) -> i32 {
        return self.english + self.math + self.chem
    }
}

fn main() {
    let mut my_map = HashMap::new();

    let name1 = String::from("name1");
    let name2 = String::from("name2");
    let name3 = String::from("name3");

    let mk1 = Marks::new(25, 67, 86);
    let mk2 = Marks::new(62, 85, 36);
    let mk3 = Marks::new(62, 85, 36);

    let total_mk1 = mk1.total_marks();
    let total_mk2 = mk2.total_marks();
    let total_mk3 = mk3.total_marks();

    println!("Total Mark 1: {total_mk1}");
    println!("Total Mark 1: {total_mk2}");
    println!("Total Mark 1: {total_mk3}");

    my_map.insert(name1, mk1);
    my_map.insert(name2, mk2);
    my_map.insert(name3, mk3);

    for (key, value) in &my_map{
        println!("{} has scored {:?}", key, value);
    } 

    let mk4 = Marks::new(36, 52, 23);
    let extract = my_map.get("name1").unwrap(); 
    println!("{}, {}, {}", extract.math, extract.chem, extract.english);

    let name3 = String::from("name3");
    
    // my_map.entry(name3).or_insert(mk4);
    // the above doesn't insert as the key contains the value already
    // my_map.insert(name3, mk4);

    // let mk5 = Marks::new(63, 42, 52); 
    
    let temp_to_deref = my_map.entry(String::from("name2")).or_insert(mk4);
    // *temp_to_deref = mk5;
    temp_to_deref.chem += 15;
    temp_to_deref.math += 15;
    temp_to_deref.english += 15;

    println!("This is a weird way of assigning... {:?}", temp_to_deref);    

    println!("This map is after name3 marks are updated... to mk5");

    for (key, value) in &my_map{
        println!("{} has scored {:?}", key, value);
    }

    println!("{}", my_map.contains_key("name1"));

}