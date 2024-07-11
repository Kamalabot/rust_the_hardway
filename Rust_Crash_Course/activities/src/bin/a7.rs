#[allow(dead_code)]
#[allow(non_camel_case_types)]
// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
enum ColorNames{
    red,
    green,
    blue,
    yellow
}
// * Use a function to print the color name
fn print_color(color: ColorNames){
    match color {
        ColorNames::red => println!("Red color"),
        ColorNames::green => println!("Green color"),
        ColorNames::blue => println!("Blue color"),
        ColorNames::yellow => println!("Yellow color")
    }
}
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn main() {
    let your_color = ColorNames::blue;
    print_color(your_color);
}
