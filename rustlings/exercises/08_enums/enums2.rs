#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Move(Point),
    Echo(String),
    Resize{
        width: i32,
        height: i32
    },
    ChangeColor(i32, i32, i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];
    
    let msg1 = Message::Move(Point{
        x:25,
        y:35
    });
    let msg2 = Message::Resize{
        width:56,
        height:76
    };
    // experimenting with match to extract the data from the 
    // structs
    match msg1 {
        Message::Move(val_stkt) => {
            println!("{:?}", val_stkt);
            println!("x value: {}", val_stkt.x);
            println!("y value: {}", val_stkt.y);
        },
        _ => println!("Nothing to see.")
    }
    match msg2 {
        Message::Resize{
            width:val1,
            height: val2
                        } => {
            println!("{}, {}", val1, val2);
        },
        _ => println!("Nothing to see.")
    }
    for message in &messages {
        message.call();
    }
}
