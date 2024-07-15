#[allow(dead_code)]
// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
enum Color{
    Red,
    Green,
    Blue,
    Yellow
}
// * Use a function to print the color name
fn print_name(colorme: Color){
    // * The function must use the enum as a parameter
    match colorme {
        // * Use a match expression to determine which color
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Yellow => println!("Yellow"),
    }
}

fn main() {
    //   name to print
    // let name = Color::Red;
    let name = Color::Blue;
    print_name(name);
}

