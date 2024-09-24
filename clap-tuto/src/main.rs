use clap::{Parser, Subcommand};
#[allow(dead_code)]
use std::path::PathBuf;

// remember to use cargo add --features=derive

#[derive(Parser)]
#[command(name = "cli-app")]
#[command(author = "Name: Rustogator")]
#[command(version = "1.0")]
#[command(about="Intro CLI app", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add Two Numbers
    Add {
        /// First Number
        a: i32,
        /// Second Number
        b: i32,
    },
    /// Subtract Two Numbers
    Subtract {
        /// First Number
        a: i32,
        /// Second Number
        b: i32,
    },
    /// Multiply Two Numbers
    Multiply {
        /// First Number
        a: i32,
        /// Second Number
        b: i32,
    },
    /// Divide two numbers
    Divide {
        /// First Number
        a: i32,
        /// Second Number
        b: i32,
    },
    /// Display the configs
    Donfig {
        /// set config level as verbose or quiet
        #[arg(short, long, default_value_t= String::from("verbose"))]
        level: String,
    },
}

fn add(a: i32, b: i32) {
    println!("Adding {a} and {b} gives: {}", a + b)
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Commands::Add { a, b } => {
        //     println!("The result of adding {} and {} is {}", a, b, a + b);
        // }
        Commands::Add { a, b } => add(*a, *b),
        Commands::Subtract { a, b } => {
            println!("The result of subtract {} and {} is {}", a, b, a - b);
        }
        Commands::Multiply { a, b } => {
            println!("The result of multiplying {} and {} is {}", a, b, a * b);
        }
        Commands::Divide { a, b } => {
            println!("The result of dividing {} and {} is {}", a, b, a / b);
        }
        Commands::Donfig { level } => {
            println!("Config level set to {}", level)
        }
    }
}
