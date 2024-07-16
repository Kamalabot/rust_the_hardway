#[allow(dead_code)]

fn prnt_str(data: &str){
    println!("Printing a Slice: {}", data);
}

struct LineItem{
    item: String,
    qty: i32
}

fn main(){
    let owned_string = "this is owned".to_owned();
    let from_other = String::from("There");

    println!("Owned: {}", &owned_string);
    println!("from: {}", &from_other);

    prnt_str("Directly printing a slice alone...");
    prnt_str(&owned_string);
    prnt_str(&from_other);

    // example that shows struct requires owned string
    let item_vec = vec![
        LineItem{
            item: "item1".to_owned(),
            qty: 58,
        },
        LineItem{
            item: "item2".to_owned(),
            qty: 36,
        }
    ];

    for elm in item_vec{
        println!("String is {}", elm.item);
        println!("Qty is {}", elm.qty);
    }
}