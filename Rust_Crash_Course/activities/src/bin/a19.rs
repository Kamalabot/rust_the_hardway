use std::collections::HashMap;

// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
#[allow(dead_code)]

struct Stock{
    item: String,
    qty: i32
}

fn main() {
    // create HM for stock storage
    // * 5 Chairs
    // * 3 Beds
    // * 2 Tables
    // * 0 Couches
    let mut hm_stk = HashMap::new();
    hm_stk.insert("chairs",Stock{item:"Chairs".to_owned(),qty: 5});
    hm_stk.insert("beds",Stock{item:"Beds".to_owned(),qty: 3});
    hm_stk.insert("tables",Stock{item:"Tables".to_owned(),qty: 2});
    hm_stk.insert("couches",Stock{item:"Couches".to_owned(),qty: 0});
    let mut sum_tot = 0;
    for (itm, stk) in hm_stk.iter(){
        println!("Reviewing {}", itm);
        if stk.qty > 0{
            println!("{} number of Item: {} is present", stk.qty, stk.item);
        } else {
            println!("No stock  of Item: {}", stk.item);
        }
        sum_tot += stk.qty;
    }
    println!("Total qty of stock is {}", sum_tot);

}
