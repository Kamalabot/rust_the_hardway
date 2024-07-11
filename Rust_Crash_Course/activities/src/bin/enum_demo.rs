#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// fn which_way(go: Direction){
// }

fn main(){
    let moves = Direction::Up;
    // which_way(move);
    match moves {
        Direction::Down => println!("Down"),
        Direction::Up => println!("Up"),
        Direction::Right => println!("Right"),
        Direction::Left => println!("Left")
    }
    // code throws warnings about unused enums
}