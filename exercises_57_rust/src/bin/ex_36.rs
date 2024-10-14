use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() {
    println!("Lottery winner..");
    let mut win_list = Vec::new();
    // Going to ask for the names
    loop {
        let mut name1 = String::new();
        println!("Give the next name: ");
        stdin().read_line(&mut name1).unwrap();
        let wname = name1.trim().to_string();
        if wname.is_empty() {
            if win_list.is_empty() {
                println!("the list is empty give atleast name");
                continue;
            }
            println!("I think you are ready for the game");
            break;
        }
        win_list.push(wname);
    }
    let idx = thread_rng().gen_range(0..win_list.len());
    let sel_name = win_list.get(idx).unwrap();
    println!("Selected name is: {sel_name}");
}
