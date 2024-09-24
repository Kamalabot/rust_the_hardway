#![allow(unused_imports)]

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about= None)]
struct Cli {
    /// this is for getting the name
    name: Option<String>,
    /// this is cli for debug
    debug: Option<u8>,
    ///arg for test
    #[arg(short, long)]
    length: Option<String>,

    #[arg(short)]
    /// Provides opt 2 that is optional
    tpt_two: Option<String>,

    #[arg(short)]
    /// Provides opt 1
    opt_one: String,

    #[command(subcommand)]
    command: MoreCmds,
}

#[derive(Subcommand)]
enum MoreCmds {
    ///Gives One
    One,
    ///Gives Two
    Two,
}
// prac1 -l 1 me 0 one
fn main() {
    let cli = Cli::parse();
    println!("Printing name: {:?}", cli.name.as_deref());
    // as_deref is used to convert the string to more
    // efficient, barrowed string
    println!("Printing debug: {:?}", cli.debug);
    // check if length is there and then print
    match cli.length {
        Some(l) => println!("There is length: {}", l),
        None => println!("No length given..."),
    }
    match cli.command {
        MoreCmds::One => println!("Print me One"),
        MoreCmds::Two => println!("Print me Two"),
    }
}
