use std::cell::RefCell;

struct Data {
    value: i32,
}

fn main() {
    let data = RefCell::new(Data { value: 10 });
    // the above data variable is immutable
    // Borrow mutably through RefCell
    *data.borrow_mut() = Data { value: 20 };
    println!("Updated value: {}", data.borrow().value);
}
