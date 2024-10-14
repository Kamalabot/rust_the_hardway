use std::io::stdin;
fn main() {
    println!("Exercise 28: Validating Inputs");
    let mut fname = String::new();
    let mut lname = String::new();
    let mut zip = String::new();
    let mut empid = String::new();

    println!("Provide your first name");
    stdin().read_line(&mut fname).unwrap();

    println!("Provide your last name");
    stdin().read_line(&mut lname).unwrap();

    println!("Provide the zip code: ");
    stdin().read_line(&mut zip).unwrap();

    println!("Provide your empid");
    stdin().read_line(&mut empid).unwrap();

    println!("The fname: {fname}, last name: {lname}");
    println!("The zip: {zip}, empid: {empid}");
    // Need to chek the above inputs by calling
    // the cheker functiona
    chek_name(lname.trim());
    chek_name(fname.trim());
    chek_zip(zip.trim());
    chek_empid(empid.trim());
}

fn chek_name(pname: &str) {
    match pname {
        _ if pname.len() < 2 => println!("give a name that is more than 2 chars"),
        _ if pname.is_empty() => println!("The name cannot be empty"),
        _ => println!("Name is accepted"),
    }
}

fn chek_zip(zipcode: &str) {
    match zipcode {
        _ if zipcode.is_empty() => println!("give a proper code"),
        _ if zipcode.chars().all(|s| s.is_alphabetic()) => {
            println!("The zip code has to be number")
        }
        _ => println!("zipcode is accepted"),
    }
}

fn chek_empid(empid: &str) {
    if empid.contains("-") {
        let mut sp_emp = empid.split("-");
        let p1 = sp_emp.next().unwrap();
        let p2 = sp_emp.next().unwrap();
        if p1.chars().all(|s| s.is_alphabetic())
            && p2.chars().all(|s| s.is_numeric())
            && p1.len() == 2
            && p2.len() == 4
        {
            println!("Emp ID is acceptable");
        } else {
            println!("Emp ID is not accepted, check the conditions")
        }
    } else {
        println!("Emp ID is not accepted. AA-1234 should be the format")
    }
}
