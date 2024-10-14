#![allow(warnings)]

use std::fs::{self, read, File};
use std::io::stdin;
use std::io::{Read, Write};

fn main() {
    println!("Employee List...Reading from the file");

    let mut emp_data = String::new();
    let mut rdata = fs::File::open("emp_list.txt").unwrap();

    rdata.read_to_string(&mut emp_data).unwrap();
    let mut emp_list: Vec<&str> = emp_data.split("\n").collect();

    println!("{:?}", emp_list);

    // Reading file completed, now getting the emp_name
    let mut rempl = String::new();
    println!("Which employee to be ejected? ");
    stdin().read_line(&mut rempl).unwrap();
    let chk_empl = rempl.trim();
    println!("Employee chosen: {chk_empl}");

    // // let idx = emp_list.binary_search(&chk_empl).unwrap();
    // let dix = emp_list.binary_search(&chk_empl);
    // match dix {
    //     Ok(val) => {
    //         println!("Idx of {rempl} is {val}");
    //         let r_emp = emp_list.remove(val);
    //         println!("Removed {r_emp}, chosen it");
    //         emp_list.sort();
    //     }
    //     Err(e) => println!("Error: {e}"),
    // }

    if let Ok(idx) = emp_list.binary_search(&chk_empl) {
        println!("Idx of {rempl} is {idx}");
        let rm_emp = emp_list.remove(idx);
        println!("Removed {rm_emp}, check it");
        emp_list.sort();
    } else {
        println!("Couldn't find the employee");
    }

    print_emp(&emp_list);
    // Now to write it back to the file
    fs::write("emp_list.txt", emp_list.join("\n")).unwrap()
}

fn print_emp(emp_vec: &[&str]) {
    for emp in emp_vec.iter() {
        println!("{emp}");
    }
}
