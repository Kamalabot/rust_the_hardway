#[allow(dead_code)]
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
    Yellow,
}
// * Use a struct to encapsulate the box characteristics
struct ShippingBox{
    weight: f64,
    color: BoxColor,
    dims: (i32, i32, i32),
}
// * Implement functionality on the box struct to create a new box
impl ShippingBox{ 
    fn making_box(weight: f64, color: BoxColor, dims: (i32, i32, i32)) -> Self{
        Self{
            weight,
            color,
            dims,
        }
    }
    // Implement functionality on the box
    // struct to print the characteristics
    fn char_print(&self){
        println!("Weight is: {}", self.weight);
        let print_clr = match self.color {
            BoxColor::Red => "Red", 
            BoxColor::Green => "Green", 
            BoxColor::Blue => "Blue", 
            BoxColor::Yellow => "Yellow", 
            // _ => "Other",
        };
        println!("color is: {}", print_clr);
        println!("Dim height: {}", self.dims.0);
        println!("Dim width: {}", self.dims.1);
        println!("Dim breadth: {}", self.dims.2);
    }  
        
}

fn main() {
    let bx1 = ShippingBox::making_box(58.66,
                                BoxColor::Green,
                                (56, 25, 75));
    bx1.char_print();
}
