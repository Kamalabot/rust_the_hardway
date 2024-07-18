#[allow(dead_code)]

enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1 = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(13);
    v1.push(23);

    println!("Getting raw vectors: {:?}", v1);

    let v2 = vec!['a', 'b', 'c', 'd'];
    let char_2 = &v2[1];

    println!("The 2nd char in vector: {}", char_2);

    // there has to be better way in getting at the values
    // let mut item3: i32; // following match pattern is different...
    match v1.get(3){
        Some(item3) => println!("The value is {}", item3),
        None => println!("There is no such index")
    }
    for i in &mut v1{
        // need to use the &mut
        *i += 5;
    } 
    for v in v1{
        println!("Value in v1 is: {:?}", v);
    }
    // working with spreadsheet cells
    let a_row_insheet = vec![
        SpreadSheetCell::Int(5),
        SpreadSheetCell::Float(56.5),
        SpreadSheetCell::Text(String::from("Halo 5")),
    ];

    match &a_row_insheet[1] {
        SpreadSheetCell::Int(val) => println!("The value of Int is: {}", val),
        SpreadSheetCell::Float(val) => println!("The value of Float is: {}", val),
        _ => println!("No integer value")
    }
    
}
