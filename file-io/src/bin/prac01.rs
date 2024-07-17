#[allow(dead_code)]
#[allow(unused_imports)]

use std::{fs, io::Write, io::Read};

fn main(){
    let warm_w01 = "warm_write01.txt".to_owned();
    let warm_wd_01 = String::from("warm_write_direct01.txt");
    let mut file1 = fs::File::create(&warm_w01).unwrap();
    // using let _ to ignore the results
    let what = file1.write(b"Whats happening...??");
    println!("{:?}", what); // Returns Ok(20)
    // another way is to write directly
    fs::write(warm_wd_01, "This is a newly minted file")
        .unwrap();
    println!("Lets see.. how to open now");
    // have a place to store the 
    let mut filedata = String::new();
    let mut prac_file = fs::File::open(&warm_w01).unwrap();
    prac_file.read_to_string(&mut filedata).unwrap();
    filedata.split_whitespace().for_each(|word| println!("{:?}", word));
}