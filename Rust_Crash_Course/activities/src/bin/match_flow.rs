fn main(){
    let mut someval = true;
    // all the different values someval can have
    // must be accounted for
    match someval {
        true => println!("Nice true"),
        false => println!("hold on, its false..."),
        _ => println!("THis is something else..."),
    }

    someval = false;
    match someval {
        true => println!("Nice true"),
        false => println!("hold on, its false..."),
        _ => println!("THis is something else..."),
        // apart from true / false bool cannot have 
        // anything else, so raises a unreachable code
        // warning.
    }
    // someval = "newval"; rust doesn't allow for string
    let intval = 6;
    match intval{
        1 => println!("Yay its 1"),
        2 => println!("Yay its 2"),
        _ => println!("THis is something else..."),
    }

    let yourname = "Bill";
    match yourname {
        "Bill" => println!("Woah there Bill... Where have you been?"),
        "Dill" => println!("Dill... You again!!!"),
        "Jill" => println!("jill... Now what did you do to jack?"),
        _ => println!("Don't know you..."),
    }

}