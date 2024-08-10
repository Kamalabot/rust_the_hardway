fn main() {
    let str1 = "test1".to_owned();
    if str1 == "test" {
        println!("The string is test1: {}", str1);
    } else {
        println!("The string is test");
    }

    let str2 = "test2";
    if str2 == "test2" {
        println!("with &str it is working: {}", str2);
    } else {
        println!("with &str else is also working: {}", str2);
    }

    match str2 {
        "test2" => println!("This is getting matched"),
        _ => println!("This one is not much matched")
    }

    let str3 = "test7".to_owned();

    match str3.as_str() {
        "test3" => println!("Does it go through match ??"),
        _ => println!("there seems to be a problem")
    }

    let str4 = "test4";

    match str4 != "test4" {
        true => println!("Yep, that is true"),
        false => println!("There is nothin left")
    }
}
