use std::io::stdin;

fn main() {
    println!("Employee List...");

    #[allow(warnings)]
    let mut emp_list = vec!["Jhonnyy", "Walker", "Katham", "Gumando"];
    println!("Employees in the list are");
    emp_list.sort();
    print_emp(&emp_list);
    let mut rempl = String::new();

    println!("Which employee to be ejected? ");
    stdin().read_line(&mut rempl).unwrap();

    let chk_empl = rempl.trim();
    println!("Employee chosen: {chk_empl}");

    // let idx = emp_list.binary_search(&chk_empl).unwrap();
    let dix = emp_list.binary_search(&chk_empl);
    match dix {
        Ok(val) => {
            println!("Idx of {rempl} is {val}");
            let r_emp = emp_list.remove(val);
            println!("Removed {r_emp}, chosen it");
            emp_list.sort();
        }
        Err(e) => println!("Error: {e}"),
    }

    // if let Ok(idx) = emp_list.binary_search(&chk_empl) {
    //     println!("Idx of {rempl} is {idx}");
    //     let rm_emp = emp_list.remove(idx);
    //     println!("Removed {rm_emp}, check it");
    //     emp_list.sort();
    // } else {
    //     println!("Couldn't find the employee");
    // }

    print_emp(&emp_list);
}

fn print_emp(emp_vec: &[&str]) {
    for emp in emp_vec.iter() {
        println!("{emp}");
    }
}
