#[allow(dead_code)]
#[allow(unused_variables)]

enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    let my_val = 3;
    // expr can contain the logic, and values 
    // are assigned to the variable 
    let expr_val = if my_val < 5 {
        true;
    } else {
        false;
    };

    let message = match my_val {
        3 => "Three me",
        _ => "Something else"
    };

    println!("Value of expr_val is {:?}", expr_val);
    // the above is printing () ??
    println!("Value of message is {}", message);

    let access_level = Access::Admin;
    let can_access_file = match access_level {
       Access::Admin => true,
       _ => false 
    };
    if can_access_file{
        println!("Has access");
    } else {
        println!("Nope, not accessible");
    }
}
// Expression allows nested logic with both if and match
// avoid using them for more than 3 levels