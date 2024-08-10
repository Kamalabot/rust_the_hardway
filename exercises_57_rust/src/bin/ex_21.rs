#[allow(dead_code)]
#[allow(unused_variables)]

use std::io::*;
use std::collections::HashMap;

fn main() {
    println!("Provide a input between 1 to 12");
    let mut get_month = String::new();

    stdin().read_line(&mut get_month).unwrap();
    let get_month: usize = get_month.trim().parse().unwrap();

    match get_month {
        1 => println!("January"),
        2 => println!("February"),
        3 => println!("March"),
        4 => println!("April"),
        5 => println!("May"),
        6 => println!("June"),
        7 => println!("July"),
        8 => println!("August"),
        9 => println!("September"),
        10 => println!("October"),
        11 => println!("November"),
        12 => println!("December"),
        _ => panic!("Number between 1 and 12 only...")
    }

    // we can implement the above using the vec also
    let mut months:Vec<String>  = Vec::new();
    
    months.push("Jan".to_owned());
    months.push("Feb".to_owned());
    months.push("Mar".to_owned());
    months.push("Apr".to_owned());
    months.push("May".to_owned());
    months.push("Jun".to_owned());
    months.push("Jul".to_owned());
    months.push("Aug".to_owned());
    months.push("Sep".to_owned());
    months.push("Oct".to_owned());
    months.push("Nov".to_owned());
    months.push("Dec".to_owned());
    
    println!("Using the decremented index: {}", get_month - 1);

    println!("Printing the month corresponding to {:?}: {:?}", get_month - 1,
        months.get(get_month - 1).unwrap());

    println!("Lets pull in HashMap also...");

    let mut month_map: HashMap<usize, String> =  HashMap::new();
    month_map.insert(1, "Jan".to_owned());
    month_map.insert(2, "Feb".to_owned());
    month_map.insert(3, "Mar".to_owned());
    month_map.insert(4, "Apr".to_owned());
    month_map.insert(5, "May".to_owned());
    month_map.insert(6, "Jun".to_owned());
    month_map.insert(7, "Jul".to_owned());
    month_map.insert(8, "Aug".to_owned());
    month_map.insert(9, "Sep".to_owned());
    month_map.insert(10, "Oct".to_owned());
    month_map.insert(11, "Nov".to_owned());
    month_map.insert(12, "Dec".to_owned());

    println!("Extracting the value from the HashMap...");

    let gotmonth = month_map.get(&get_month); 

    match gotmonth {
        Some(val) => println!("Got the month: {}", val),
        _ => println!("THere is error")
    };
}
