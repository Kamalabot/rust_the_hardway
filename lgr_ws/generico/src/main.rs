#[allow(dead_code)]
use backtrace::Backtrace;

fn find_largest(in_vec: Vec<i32>) -> i32 {
    let mut largest = 0;

    for elem in in_vec {
        if elem > largest{
            largest = elem;
        }
    }
    largest
}

fn find_gen_lgst<T: PartialOrd + Copy>(in_vec: Vec<T>) -> T {
    // let mut largest = 0;
    let mut largest = in_vec[0];

    for elem in in_vec {
        if elem > largest{
            largest = elem;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}
impl <T, U> Point <T, U> {
    fn get_x(&self) -> &T{
        &self.x
    } 
}
impl <T, U> Point <T, U>{
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<U, V>{
        Point { x: self.y, y: other.x}
    }
}
fn main() {
    // generics are used when the inputs are 
    // going to be different types
    let v1 = vec![2, 5, 26, 76, 82];
    // move the below repeating code to function
    // let mut largest = 0;
    // for elem in v1 {
        // if elem > largest{
            // largest = elem;
        // }
    // }
    let loc_largest = find_largest(v1);
    println!("The largest is: {}", loc_largest);
    
    let v2 = vec![28, 57, 86, 23, 7, 74, 78];
    // let mut largest = 0;
    // for elem in v2 {
        // if elem > largest{
            // largest = elem;
        // }
    // }
    // let mut loc_largest2 = find_largest(v2);
    let loc_largest2 = find_gen_lgst(v2);
    println!("The largest is: {}", loc_largest2);

    let char_vec = vec!['a', 'b', 'd', 'p', 't'];
    let char_largest = find_gen_lgst(char_vec);
    // the above function can be used with multiple types
    println!("Largest Char is {}", char_largest);

    let p1 = Point{
        x: 5,
        y: 2.5
    };

    let p2 = Point {
        x: 'x',
        y: 'y'
    };

    let p3 = Point {
        x: 25.5,
        y: 5
    };
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);

    println!("{}", p1.get_x());

    let p3 = Point{
        x: "hello",
        y: 5
    }; 
    let new_p = p3.mix_up(p2);

    println!("{:#?}", new_p);

    let b1  = Backtrace::new();
    println!("{:?}", b1);
}
