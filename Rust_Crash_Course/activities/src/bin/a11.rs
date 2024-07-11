// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
struct Grocery{
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn disp_qty(g1: &Grocery){
    println!("Quantity is {}", g1.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn disp_id(g1: &Grocery){
    println!("Id is {}", g1.id);
}

fn main() {
    let gr1 = Grocery{
        quantity: 32,
        id: 62
    };
    disp_id(&gr1);
    disp_qty(&gr1);
}
