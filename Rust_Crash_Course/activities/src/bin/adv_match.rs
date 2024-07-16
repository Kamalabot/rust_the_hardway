#[allow(dead_code)]
enum Discount{
    Percent(i32),
    Flat(i32)
}

struct Ticket{
    event: String,
    price: i32
} 

fn main(){
    let n = 3;
    // simple match
    match n {
        3 => println!("Its three... "),
        _ => println!("No Night")
    }
    let flat = Discount::Flat(25);
    match flat {
        Discount::Flat(25) => println!("Its flat 25"),
        Discount::Flat(amt) => println!("{}", amt),
        _ => println!("There no percent discount"),
    }

}