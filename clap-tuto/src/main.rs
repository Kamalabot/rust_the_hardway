#[allow(dead_code)]

use clap::{Parser, Subcommand};

// remember to use cargo add --features=derive


#[derive(Parser)]
#[command(name="cli-app")]
#[command(author="Name: Rustogator")]
#[command(version="1.0")]
#[command(about="Intro CLI app", long_about=None)]
struct Cli {
    #[command(Subcommand)]
    command: Commands,
}

enum Commands{
    /// Add Two Numbers
    Add {
        /// First Number
        a: i32,
        /// Second Number
        b: i32
    },
    /// Subtract Two Numbers
    Subtract {
        /// First Number
        a: i32,
        /// Second Number
        b: i32
    },
    /// Multiply Two Numbers
    Multiply {
        /// First Number
        a: i32,
        /// Second Number
        b: i32
    },
    /// Divide two numbers
    Divide {
        /// First Number
        a: i32,
        /// Second Number
        b: i32
    },
    /// Display the configs
    Config {
        /// set config level as verbose or quiet
        #[Arg(short, long, default_value_t= String::from("verbose"))]
        level: String,
    }

}

fn main() {
    println!("Hello, world!");

}
