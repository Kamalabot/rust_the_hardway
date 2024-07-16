#[allow(dead_code)]
#[allow(unused_variables)]
// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use an enum for the box color
enum BoxColor{
    Red,
    Green,
    Blue,
    Brown
    }

impl BoxColor{
   fn print(&self){
    match self {
        BoxColor::Red => println!("Red printed by Impl"),
        BoxColor::Green => println!("Green printed by Impl"),
        BoxColor::Blue => println!("Blue printed by Impl"),
        BoxColor::Brown => println!("Brown printed by Impl"),
    }
   } 
}
// * Use a struct to encapsulate the box characteristics
struct Box {
    dims: (i32, i32, i32),
    weight: f64,
    color: BoxColor, 
}
// * Implement functionality on the box struct
// to create a new box
impl Box {
    fn create_box(weight: f64, color: BoxColor, dims: (i32, i32, i32)) -> Self {
        Self {
            dims,
            weight,
            color,
        }
    }
    fn disp_char(&self) {
        println!("Dimension of box: {} {} {}",
                self.dims.0,
                self.dims.1,
                self.dims.2);

        let boxcol = &self.color;
        
        let printcol = match boxcol {
           BoxColor::Red => "Red", 
           BoxColor::Green => "Green", 
           BoxColor::Blue => "Blue", 
           BoxColor::Brown => "Brown", 
        };
        println!("Box color is {}", printcol);
        self.color.print(); // calls the impl of enum boxcolor
        println!("Box Weight is: {}", &self.weight)
    }
}
// * Implement functionality on the box 
// struct to print the characteristics

fn main() {
    let box1 = Box::create_box(57.65, 
                                BoxColor::Green,
                            (65, 25, 25));
    box1.disp_char();
}
