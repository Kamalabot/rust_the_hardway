use std::collections::HashMap;
use std::io::stdin;

fn main() {
    println!("Sorting Data using List of Maps");
    let mut map_list = Vec::new();
    let keys = ["FirstName", "LastName", "Position", "SeperationDate"];
    // create a routine to get user input
    for idx in 0..3 {
        println!("Collecting data for {} employee", idx + 1);
        let mut emp_map = HashMap::new();
        for key in keys {
            let mut in_data = String::new();
            println!("Provide {key} of the employee");
            stdin().read_line(&mut in_data).unwrap();
            // clean the data before inserting into the map
            let cln_data = in_data.trim().to_owned();
            emp_map.insert(key, cln_data);
        }
        println!("push data for {} employee", idx + 1);
        map_list.push(emp_map)
    }
    show_employees(&map_list);
    println!("The data is sorted based on the names");
    map_list.sort_by(|a, b| a.get("FirstName").unwrap().cmp(b.get("FirstName").unwrap()));
    show_employees(&map_list)
}

fn show_employees(map_list: &[HashMap<&str, String>]) {
    println!("Name      |Position   | Seperation Date     ");
    for map in map_list {
        let fullname = [
            map.get("FirstName").unwrap().to_owned(),
            map.get("LastName").unwrap().to_owned(),
        ]
        .concat();
        println!(
            "{}     |{}     |{}     ",
            fullname,
            map.get("Position").unwrap(),
            map.get("SeperationDate").unwrap()
        );
    }
}
