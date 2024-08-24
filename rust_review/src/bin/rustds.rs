#[allow(dead_code)]
#[allow(unused_variables)]

enum Devices {
    Phone,
    Remote,
    TV,
    Mouse,
    Laptop
}

impl Devices {
    fn is_laptop(&self) -> bool {
        match self {
            Devices::Laptop => true,
            _ => false
        }
    }
}

// Idea is to double a referenced vector
// into another vector
fn dbl_refvec(in_vec: &mut Vec<i32>) -> &mut Vec<i32> {
    // need to barrow as mutable
    let vec2 = in_vec; 
    // testing push 
    vec2.push(1);
    // looping push
    let mut sdx = 0;
    loop {
        vec2.push(sdx);
        sdx += 5;
        if sdx > 25{
            break;
        }
    }
    vec2 // we don't need to return it
}


fn main(){
    println!("This is going to contains a lot ");
    println!("example of the collections and datastructures");
    println!("************     More Data Structures ***********");

    let device = Devices::Laptop;

    println!("so is the device a laptop: {}", device.is_laptop());

    match device {
        Devices::Laptop => println!("This is a Laptop"),
        Devices::TV => println!("This is a TV"),
        Devices::Phone => println!("This is a Phone"),
        Devices::Mouse => println!("This is a Mouse"),
        Devices::Remote=> println!("This is a Remote"),
    }

    println!("Trying wrap my head around the Vectors");

    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(5);

    println!("This is vec1: {:?}", vec1);

    let mut vec2 = vec![1, 6, 7, 9];

    vec2.push(5);
    
    println!("This is vec2 0th element: {:?}", vec2[0]);

    let sec: &i32 = &vec2[1];

    match vec2.get(10) {
        Some(second) => println!("This is second: {}", second),
        None => println!("THere is no value")
    }
    // we can cycle and change value
    for val in &mut vec2 {
        *val *= 6;
    }

    println!("Can get the len, {}", vec2.len());
    
    println!("Pop the last element in left , {:?}", vec2.pop());

    let mut vec5 = vec![8, 6, 3, 9, 5, 4, 10];

    // we are mutably barrowing vec5 & playing with it
    println!("Before mutating the vec5...");
    println!("{:?}", vec5);

    dbl_refvec(&mut vec5);

    println!("Before mutating the vec5...");
    println!("{:?}", vec5);
    
    use std::collections::HashMap;

    println!("This is start of HashMap... ");

    let mut cities = HashMap::new();

    cities.insert("city1", "NewJersey");
    cities.insert("city2", "Texas");
    cities.insert("city3", "Ikon");
    cities.insert("city4", "California");

    // iterating over hashmap
    for(k, v) in cities.iter() {
        println!("Key: {}, Value: {}", k, v);
    }

    // if city1 is there, then enter the loop
    if cities.contains_key(&"city1"){
        // Get value with key
        let bman = cities.get(&"city5");

        match bman {
            Some(N) => println!("NewJersey is in..{}", N),
            None => println!("Jersey is not there") 
        }
    }
    println!("Work on Structs");
    // struct is more like a class definition

    #[derive(Debug)] 
    struct Stuff {
        name: String,
        location: String,
        weight: f32 
    }

    let mut stf1 = Stuff {
        name: "table".to_owned(),
        location: String::from("hall"),
        weight: 256.8
    };

    println!("Accessing stf1: {:?}", stf1);    
    
    #[derive(Debug)]
    struct Rect<T, U> {
        length: T,
        weight: U
    }

    let rc1 = Rect{
        length: 25.0,
        weight: 56
    };

    println!("Show rc1: {:?}", rc1);
    
    println!("Starting of Traits, is more like ABC with method requirement");
    println!("Collecting a set of function denitions and attaching");
    println!("it to the Structs / Enums");

    trait Shape {
        fn new(lt: f32, wd: f32) -> Self;

        fn area(&self) -> f32;
    }

    struct Tangle {length: f32, width: f32}
    struct Cle {radius: f32, thickness: f32}

    impl Shape for Tangle {
        fn new(lg: f32, wd: f32) -> Tangle {
            Tangle{length: lg, width: wd}
        }

        fn area(&self) -> f32 {
            self.length * self.width
        }
    }
    impl Shape for Cle{
        fn new(lg: f32, wd: f32) -> Cle {
            Cle {radius: lg, thickness: wd}
        }

        fn area(&self) -> f32 {
            self.radius * self.radius * 22.0 / 7.0
        }
    }

    let cl1 = Cle::new(25.6, 6.8);
    let tg1 = Tangle::new(43.7, 10.8);

    println!("Lets see cl1 area: {}", cl1.area());
    println!("Lets see tg1 area: {}", tg1.area());

    
}










