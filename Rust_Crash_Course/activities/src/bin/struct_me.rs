#[allow(dead_code)]
#[allow(non_camel_case_types)]
// implement a simple struct
struct ShippingBox{
    depth: i32,
    width: i32,
    height: i32,
} 

fn main(){
    let box1 = ShippingBox{
        depth: 5,
        width: 7,
        height: 8
    };

    let box_h = box1.height;

    println!("The box dims are {} {} {}",
            box1.depth,
            box1.height,
            box1.width);

    print!("We can access the box_h also {}", box_h)
}