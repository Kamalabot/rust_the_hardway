use std::collections::HashMap;
use std::io::stdin;

fn main() {
    println!("Filtering Data using List of Maps");
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
        // build the fullname and insert that also
        let fname = [
            emp_map.get("FirstName").unwrap().to_owned(),
            emp_map.get("LastName").unwrap().to_owned(),
        ]
        .concat();
        emp_map.insert("fullname", fname);
        println!("push data for {} employee", idx + 1);
        map_list.push(emp_map)
    }
    show_employees(&map_list);
    println!("The data is filtered based on the fullname");
    let mut srch = String::new();
    println!("Provide your search string: ");
    stdin().read_line(&mut srch).unwrap();
    let search_str = srch.trim();
    let filter_list: Vec<HashMap<&str, String>> = map_list
        .into_iter()
        .filter(|x| x.get("fullname").unwrap().contains(search_str))
        .collect();
    show_employees(&filter_list)
}

fn show_employees(map_vec: &[HashMap<&str, String>]) {
    println!("Name      |Position   | Seperation Date     ");
    for map in map_vec {
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
