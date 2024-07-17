#[allow(unused_imports)]
#[allow(dead_code)]
// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult{
    name: String,
    age: i32
}
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// impl Adult {
    // fn new(name: String, age: i32) -> Self{
        // Self {
            // name,
            // age
        // }
    // }
// }

impl Adult {
    fn new(age: i32, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                name: name.to_owned(),
                age,
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

fn new(name: String, age: i32) -> Result<Adult, String>{
    if age > 21{
        let adult = Adult{
            name,
            age,
        };
        Ok(adult)
    } else {
        Err("Age has to above 21".to_owned())
    }
}


fn main() {
    // * Instantiate two `Adult` structures:
    //   * One should be aged under 21
    //   * One should be 21 or over
    let ad1 = new("adult1".to_owned(), 22);
    let ad2 = new("adult2".to_owned(), 20);
    let ch2 = Adult::new(25, "logan");
    println!("{:?}", ad1);
    println!("{:?}", ad2);
    println!("{:?}", ch2);
    // * Use `match` to print out a message for each `Adult`:
    //   * For the Ok variant, print any message you want
    //   * For the Err variant, print out the error message
    match ad1 {
        Ok(ref in_ad) => println!("This worked: {:?}", in_ad),
        Err(ref e) => println!("The Error: {}", e),
    }
    match ad2 {
        Ok(ref in_ad) => println!("This worked: {:?}", in_ad),
        Err(ref e) => println!("The Error: {}", e),
    }

}
