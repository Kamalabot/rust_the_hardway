// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
struct Grocery{
    // * Use two i32 fields for the quantity and id number
    qty: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_qty(in_grcy: &Grocery){
    println!("Quantity is: {}", in_grcy.qty);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(in_grcy: &Grocery){
    println!("Identity is: {}", in_grcy.id);
}

fn main() {
    let grk1: Grocery = Grocery{
        qty:5,
        id: 1,
    };
    display_id(&grk1);
    display_qty(&grk1);
}
