use colored::Colorize;
fn main() {
    println!("{}", "Code introduces List like DS".green());
    let ls1 = [1, 2, 3, 5];
    for l in ls1{
        println!("{}", l.to_string().red());
    }
    let ls2 = [7, 9, 6, 5];
    let ls3 = [52..100];
    let ls5 = [ls1, ls2].concat();
    println!("Extracting data from Range...");
    for r in ls3{
        println!("{:?}", r); // [52..100]
    }
    for lr in ls5{
        println!("{}", lr);
    }

    // slicing on the list
    let slice = &ls2[..2];
    println!("{:?}", slice);
}
