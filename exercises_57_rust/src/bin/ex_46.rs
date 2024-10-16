use std::fs::{self, File};
use std::io::{stdin, Read};

fn main() {
    println!("Updating the file contents");
    let mut fname_rw = String::new();
    let mut rw_str = String::new();
    let mut fobj = File::open("utilize.txt").unwrap();
    fobj.read_to_string(&mut rw_str).unwrap();
    let cln_str = rw_str.trim();
    let updt_str = cln_str.replace("utilize", "use");
    println!("String updated. Provide new file name: ");
    stdin().read_line(&mut fname_rw).unwrap();
    let fname = fname_rw.trim();
    fs::write(fname, updt_str).unwrap();
}
