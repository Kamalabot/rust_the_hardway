use uuid::Uuid;

fn main() {
    println!("Hello, world!");
    println!("Getting added: {}", lib01::modadd::add(5, 6));
    let ids = Uuid::new_v4();
    println!("Getting UUID: {}", ids.to_string());
}

fn news() -> String {
    format!("{}", 87)
}
