#[allow(dead_code)]
// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
enum Drinks {
    Mango,
    Apple,
    Orange,
    Banana
}
// * Use a struct to store drink flavor and fluid ounce information
struct YourDrink{
    flavor: Drinks,
    ounce: f64
}
// * Use a function to print out the drink flavor and ounces
fn print_drink(mydrink: YourDrink){
    let ounce = mydrink.ounce;
    // * Use a match expression to print the drink flavor
    let flav = match mydrink.flavor {
        Drinks::Mango => "Mango Flavour",
        Drinks::Apple => "Apple Flavour",
        Drinks::Orange => "Orange Flavour",
        Drinks::Banana => "BananaFlavour",
    };
    println!("Your flavour: {} is and dring qty is: {} ounces", flav, ounce);
}

fn main() {
    let mdrk= YourDrink{
        flavor: Drinks::Apple,
        ounce: 56.7,
    };
    print_drink(mdrk);
}
