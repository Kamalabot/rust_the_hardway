#[allow(dead_code)]
#[allow(non_camel_case_types)]
// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
enum DrinkFlavor{
    Apple,
    Orange,
    Lemon,
    Strawberry,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink{
    flavor: DrinkFlavor,
    quantity: i32,
}
// * Use a function to print out the drink flavor and ounces
fn print_info(your_drink: Drink){
    let flav = your_drink.flavor;
    // * Use a match expression to print the drink flavor
    match flav{
        DrinkFlavor::Apple => println!("Apple flavor"),
        DrinkFlavor::Orange => println!("Orange flavor"),
        DrinkFlavor::Lemon => println!("Lemon flavor"),
        DrinkFlavor::Strawberry => println!("Strawberry flavor"),
    }
    println!("Drink quantity is: {}", your_drink.quantity);
}

fn main() {
    let drink1 = Drink{
        flavor: DrinkFlavor::Apple,
        quantity: 55
    };
    print_info(drink1);
}
