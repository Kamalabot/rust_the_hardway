#[allow(dead_code)]
#[allow(unused_imports)]

use std::io;

#[derive(Debug)]
enum MenuChoice{
    MainMenu,
    SubMenu,
    Quit,
}

// fn get_sound(name: &str) -> Result<MenuChoice, String>{
    // if name == "alert"{
        // Ok(SoundData::new("alert"));
    // } else {
        // Err("File not found");
    // } 
// }

fn get_choice(name: &str) -> Result<MenuChoice, String>{
    println!("Entering Choice is {}", &name);
    // this match returns either Ok or Err
    // which needs to be stored
    match name {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "submenu" => Ok(MenuChoice::SubMenu),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Its screwed up...".to_owned()),
    }
}
fn print_choice(choice: &MenuChoice) {
    println!("Choice = {:?}", choice);
}

fn pick_choice(input: &str)-> Result<(), String>{
    let choice = get_choice(input)?;
    // ? automaticall does a match on the returned menu 
    // choice, and sends the error to the called function
    print_choice(&choice);
    Ok(())
}
fn main(){
    // let mut in_str = String::new();
    // println!("Provide the menu you want to go: ");
    // io::stdin()
        // .read_line(&mut in_str)
        // .unwrap();
    // let out = get_choice(&in_str);
    // Following is replaced with pick_choice
    // let out: Result<MenuChoice, _>= get_choice("mainmenu");
    // match out {
        // // the analyzer asked to use ref
        // Ok(ref in_choice) => print_choice(&in_choice),
        // Err(ref e) => println!("Error: {:?}", e)
    // }
    // println!("{:?}", out);
    // let _ = pick_choice("start");
    let _ = pick_choice("mainmenu");
}