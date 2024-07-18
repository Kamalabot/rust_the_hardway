#[allow(dead_code)]
// enum IpAddrKind{
    // V4,
    // V6
    // }
#[derive(Debug)]
enum IpAddrKind{
    V4(String),
    V6(String)
}
#[derive(Debug)]
enum OperatingSystem{
    Linux,
    Mac,
    Window,
    RaspberryPi
}
// struct YourAddr{
    // kind: IpAddrKind,
    // address: String,
    // }
#[derive(Debug)]
struct YourAddr{
    kind: IpAddrKind,
    os: OperatingSystem,
}
#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    Changecolor(i32, i32, i32)
}
// what are the purposes of enum needs to be listed
// - Creating a data structure with fixed set of choices
// - Each of the choice can be any of Rust Data Types, including enums
// - How to access the data type inside the enum?
//      - using the match enumobj and get at the values

// very interesting use case
#[derive(Debug)]
enum States{
    Alabama,
    Florida,
    Texas,
    Phoenix,
    LosAngeles
}

#[derive(Debug)]
enum Coins{
    Penny,
    Nickel,
    Dime,
    Quarter(States)
}
fn main() {
    // let ipv1 = IpAddrKind::V4;
    // let ipv6 = IpAddrKind::V6;

    let ipv1 = IpAddrKind::V4("127.0.0.1".to_owned());

    let lhost = YourAddr{
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        os: OperatingSystem::Linux,
    };
    println!("{:?}", lhost);
    println!("{:?}", ipv1);

    match ipv1 {
        IpAddrKind::V4(val) => println!("{:?}", val),
        IpAddrKind::V6(val) => println!("{:?}", val),
    }
    match lhost.kind{
        IpAddrKind::V4(val) => println!("{:?}", val),
        IpAddrKind::V6(val) => println!("{:?}", val),
    }

    // introducing Option Enum
    let x = 62;
    let y:Option<i32> = Some(6);

    // let sum_opt = x + y; // will error
    let sum_opt = x + y.unwrap_or(0); 
    println!("Sum of int and option is: {}", sum_opt); 

    let coin1 = Coins::Dime;
    let c1 = value_coin(coin1);
    println!("{}", c1); // 1

    let coin2 = Coins::Quarter(States::Alabama);
    let c2 = value_coin(coin2);
    println!("{}", c2); // 1

}

fn value_coin(in_coin:Coins)->u8{
    match in_coin {
        Coins::Dime => {
            println!("I'm a dime");
            1
        },
        Coins::Nickel => 5,
        Coins::Penny => 10,
        Coins::Quarter(state) => {
            println!("{:?}", state); 
            25
        },
    }
}
