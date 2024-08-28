// reviewed concepts of traits
// learnt how traits can be seperate objects
// yet they have to be implemented on an object
// to use the function inside
//

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn sum_coord(&self) -> i32 {
        self.x + self.y
    }
}

trait Summarize {
    fn get_words(&self) -> i32;
    // fn no_words() -> i32;
    // unable to access if &self is not declared
    fn reduce_length(&self) -> String;
}

impl Summarize for Point {
    fn get_words(&self) -> i32 {
        5
    }

    fn reduce_length(&self) -> String {
        "pass".to_string()
    }

}

fn print_pt_ref(point: &Point) {
    // ownership is the main
    println!("Point by its reference: {}, {}", point.x, point.y);
}


fn print_pt_val(point: Point) {
    // ownership is the main
    println!("Point by its reference: {}, {}", point.x, point.y);
}
fn main() {
    let pt1 = Point{x:5, y:5};
    println!("Point value is: {:?}", pt1);
    println!("{}", pt1.reduce_length());
    println!("Value of pt1 is: {}", pt1.sum_coord());
    println!("Using get_words: {:?}", pt1.get_words());

    print_pt_ref(&pt1);
    
    println!("Value of pt1 after calling by ref: {}", pt1.sum_coord());

    print_pt_val(pt1);
    // cannot use below
    // println!("Value of pt1 after calling by ref: {}", pt1.sum_coord());
}








