// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
struct Person {
    // * The color and name should be stored as a String
    age: i32,
    name: String,
    color: String
}
// * The name and colors should be printed using a function
fn print_str(data: &str){
    println!("{}", data);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let mut person_vector = Vec::new();
    person_vector.push(Person{age: 6,
                             name: "Nic".to_owned(),
                            color: "Green".to_owned()});
    person_vector.push(Person{age: 12,
                             name: "kon".to_owned(),
                            color: "Blue".to_owned()});
    person_vector.push(Person{age: 5,
                             name: "Tbp".to_owned(),
                            color: "Yellow".to_owned()});
    // * Iterate through the vector using a for..in loop
    for person in person_vector{
        // * Use an if expression to determine which person's
        // info should be printed
        if person.age < 10 {
            print_str(&person.name);
            print_str(&person.color);
        }

    }
}
