// there are two ways to write the file
#[allow(unused_imports)]
use std::{fs, io::Write, io::Read};
// we can use the io::Write trait for writing data to file
// we can use the io::Read trait for reading data from file
fn main() {
    // let mut file = fs::File::create("new_file.txt").unwrap();
    // file.write_all(b"Hey there, how are you?").unwrap();
    fs::write("write_direct.txt", "newly minted direct file")
        .unwrap();
    // unwrap() ignores the errors.
    // using the fs' own write 

    // Now that we have files, we can read the file
    let mut my_file = fs::File::open("new_file.txt").unwrap();
    let mut s = String::new();

    my_file.read_to_string(&mut s).unwrap();
    // split the string data with white space
    s.split_whitespace().for_each(|word| println!("{:?}", word));
}
