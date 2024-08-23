#[allow(dead_code)]

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

}










