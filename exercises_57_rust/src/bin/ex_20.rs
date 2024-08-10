#[allow(dead_code)]
use std::io::*;

enum States {
    EauClaire,
    Dunn
}
 
fn main() {
    /* work here */
    let mut state = String::new();
    println!("Provide the state your are residing: ");
    stdin().read_line(&mut state).unwrap();
    let state = state.trim().to_lowercase();

    if state == "eau claire" {
        let mut enum_state = States::EauClaire;
    } else {
        let mut enum_state = States::Dunn;
    }

    match enum_state {
        States::EauClaire => {
            println!("There is more to Eau Claire than it meets the eyes");
        },
        States::Dunn => {
            println!("Not much to say...");
        }
    }
}
