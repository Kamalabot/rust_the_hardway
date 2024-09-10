use std::collections::HashMap;
use std::io::stdin;

fn main() {
    println!("Enter Two strings and I will tell you if they are anagrams...");
    let mut str1 = String::new();
    let mut str2 = String::new();

    println!("Enter First String: ");
    stdin().read_line(&mut str1).unwrap();

    println!("Enter Second String: ");
    stdin().read_line(&mut str2).unwrap();

    println!("Your entered strings: \n {} \n {}", str1, str2);
    let check = check_anagram(str1, str2);
    println!("They are anagram: {}", check);
}

fn check_anagram(str1: String, str2: String) -> bool {
    let str1 = str1.strip_suffix('\n').unwrap().to_owned();
    let str2 = str2.strip_suffix('\n').unwrap().to_owned();

    // get the counter for str1 and str2
    let mut counter1: HashMap<char, i32> = HashMap::new();
    let mut counter2: HashMap<char, i32> = HashMap::new();

    // convert the string to vectors
    for char in str1.chars() {
        // println!("can enumerate {}", &char);
        if counter1.contains_key(&char) {
            let count = counter1.get(&char).unwrap();
            counter1.insert(char, count + 1);
        } else {
            counter1.insert(char, 1);
        }
    }
    for char in str2.chars() {
        // println!("can enumerate {}", &char);
        if counter2.contains_key(&char) {
            let count = counter2.get(&char).unwrap();
            counter2.insert(char, count + 1);
        } else {
            counter2.insert(char, 1);
        }
    }

    println!("Printing HashMap: {:?}", counter1);
    println!("Printing HashMap: {:?}", counter2);
    // for (idx, bts) in str2.bytes().enumerate() {
    // gettin the bytes of each char and adding them
    // will be an easy comparison
    // println!("Byte of {}, {}", str1.chars().nth(idx).unwrap(), bts)
    // }
    for item in counter1.keys() {
        if !counter2.contains_key(item) || (counter2.get(item) != counter1.get(item)) {
            // if any of the above happens then return false
            return false;
        }
    }
    // else return true
    true
}

#[cfg(test)]
mod tests {
    use crate::check_anagram;

    #[test]
    fn test_true_anag() {
        assert!(check_anagram("note\n".to_owned(), "tone\n".to_owned()))
    }

    #[test]
    fn test_false_anag() {
        assert!(check_anagram("note\n".to_owned(), "tone\n".to_owned()))
    }
}
