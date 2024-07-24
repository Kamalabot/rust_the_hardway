#[allow(unused_imports)]

use std::io::stdin;

fn madlib(noun: String, verb: String, adverb: String, adjective: String) -> String {
    let l_noun = noun.trim();
    let l_verb = verb.trim();
    let l_adverb = adverb.trim();
    let l_adjective = adjective.trim();
    format!("Do you {l_verb} your {l_adjective} {l_noun} {l_adverb}")
}

fn main() {
    // define the container variables
    let mut noun = String::new();
    let mut verb = String::new();
    let mut adverb = String::new();
    let mut adjective = String::new();

    // get the inputs for the user
    println!("Provide a Noun: ");
    stdin().read_line(&mut noun).unwrap();
    println!("Provide a Verb: ");
    stdin().read_line(&mut verb).unwrap();
    println!("Provide a Adjective: ");
    stdin().read_line(&mut adjective).unwrap();
    println!("Provide a Adverb : ");
    stdin().read_line(&mut adverb).unwrap();
    // Call the madlib function
    let your_string = madlib(noun, verb, adverb, adjective);
    // print the final string
    println!("{}", your_string);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_madlib(){
        let noun = String::from("dog");
        let verb = String::from("walk");
        let adverb = String::from("quickly");
        let adjective = String::from("blue");
        assert_eq!(madlib(noun, verb, adverb, adjective),
                    "Do you walk your blue dog quickly")
    }
}
