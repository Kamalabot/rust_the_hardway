use std::fs;
use std::io::stdin;

fn main() {
    println!("Creating folder tree");
    let mut site_raw = String::new();
    let mut author_raw = String::new();
    println!("Provide the site name: ");
    stdin().read_line(&mut site_raw).unwrap();
    let site = site_raw.trim();
    println!("Provide the Author name: ");
    stdin().read_line(&mut author_raw).unwrap();
    let author = author_raw.trim();

    let mut choice = String::new();
    println!("Looking for JS folder, Y/N?");
    stdin().read_line(&mut choice).unwrap();
    let cln_choice = choice.trim();
    if cln_choice == "Y" {
        let js_path = format!("./{site}/js");
        fs::create_dir_all(js_path).unwrap();
    }
    choice.clear();
    println!("Looking for css folder, y/n");
    stdin().read_line(&mut choice).unwrap();
    let cln_choice = choice.trim();
    if cln_choice == "y" {
        let css_path = format!("./{site}/css");
        fs::create_dir_all(css_path).unwrap();
    }
    let author_tag = format!("<meta>{author}</meta>\n");
    let site_tag = format!("<title>{site}</title>\n");
    let tags = [site_tag, author_tag].concat();
    println!("Writing the index.html file");
    fs::write("index.html", tags).unwrap();
}
