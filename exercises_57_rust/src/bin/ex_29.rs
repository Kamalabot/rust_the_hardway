use std::io::stdin;

fn main() {
    println!("Lets compute total of five numbers");
    let mut total = 0;
    let mut count = 0;
    loop {
        println!("Provide a number: ");
        // the string has to be created as many times the
        // loop proceeds
        let mut indata = String::new();
        stdin().read_line(&mut indata).unwrap();
        count += 1;
        println!("Provide indata is: {indata}");
        let data = indata.trim().parse::<i32>().unwrap();
        total += data;
        if count >= 5 {
            break;
        }
    }
    println!("The total is :{total}");
}
