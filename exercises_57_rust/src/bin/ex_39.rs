fn main() {
    println!("Provide a series of numbers in a single line");
    let mut instr = String::new();
    use std::io::stdin;
    stdin().read_line(&mut instr).unwrap();
    let cln_str = instr.trim();
    // convert to vectors
    let vec_str: Vec<&str> = cln_str.split(" ").collect();
    // convert to numbers
    let vec_i32: Vec<i32> = vec_str.iter().map(|v| v.parse::<i32>().unwrap()).collect();
    println!("{:?}", vec_i32);
    // lets run filter
    let vec_filter: Vec<i32> = vec_i32.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("Filtered data: {:?}", vec_filter);
}
