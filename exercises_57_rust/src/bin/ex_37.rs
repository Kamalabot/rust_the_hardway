use std::io::stdin;

fn main() {
    println!("Computing Statistics..");
    let mut num_list = Vec::new();
    // Going to ask for the names
    loop {
        let mut num1 = String::new();
        println!("Give the number: ");
        stdin().read_line(&mut num1).expect("Provide a num");
        let wnum = num1.trim().to_string();
        if wnum.is_empty() {
            if num_list.is_empty() {
                println!("the list is empty give atleast name");
                continue;
            }
            println!("I think you are ready for the game");
            break;
        }
        let num = wnum.parse::<i32>().expect("Require a number");
        num_list.push(num);
    }
    let sum_list: i32 = num_list.iter().sum();
    let mean = sum_list as usize / num_list.len();
    let max_list = num_list.iter().max().expect("No max value");
    let min_list = num_list.iter().min().expect("No min value");
    let sqr_diff: Vec<i32> = num_list.iter().map(|b| b - mean as i32).collect();
    let sqr_list: Vec<i32> = sqr_diff.iter().map(|x| x * x).collect();
    let sd_sum: i32 = sqr_list.iter().sum();
    let sd_mean = sd_sum / num_list.len() as i32;
    let sd = f64::sqrt(sd_mean as f64);
    println!("Sum: {sum_list}");
    println!("Mean: {mean}");
    println!("Max: {max_list}");
    println!("Min: {min_list}");
    println!("Sd: {sd}");
}
