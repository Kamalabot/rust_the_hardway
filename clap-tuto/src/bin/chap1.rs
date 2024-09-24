use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about,long_about=None)]
struct Cli {
    /// optional name to operate
    #[arg(long, value_name = "name me")]
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, value_name = "FILE")]
    config: Option<PathBuf>,
    // adding the #[arg()] makes it an option
    // that requires a flag
    /// Turn debugging info on
    debug: u8,
    // if nothing then it is CLI arg
    //
    /// Turn Another debug on
    another_debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Does testing things
    Test {
        /// list the values
        // chap1 0 0 test -l
        #[arg(short, long)]
        list: bool,

        #[arg(last = true)]
        items: Vec<String>,
    },
    // target/debug/chap1 0 1 test -l -- 1 2 3
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("value of name : {name}");
    }
    if let Some(config_path) = cli.config.as_deref() {
        println!("value of name : {}", config_path.display());
    }
    match cli.debug {
        0 => println!("Debug is off"),
        1 => println!("Debug is 1"),
        2 => println!("Debug is 2"),
        _ => println!("Debug is off the roof"),
    }
    match cli.another_debug {
        0 => println!("Here goes 0"),
        _ => println!("There is nothing"),
    }
    match &cli.command {
        Some(Commands::Test { list, items }) => {
            if *list {
                println!("Printing test list...{:?}", *items);
            } else {
                println!("Not printing...");
            }
        }
        None => {}
    }
}
