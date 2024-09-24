#![allow(unused_imports)]
#![allow(warnings)]

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.2.2")]
#[command(about = "Practice 2")]
#[command(long_about = None)]
struct Cli {
    /// this is the first string arg
    carg1: String,
    /// this is second i32 arg
    carg2: i32,
    /// this is first option
    #[arg(short)]
    fopt: String,
    /// this is Second option
    #[arg(long)]
    sopt: Option<String>,
    /// following is subcommand
    #[command(subcommand)]
    cmd: SubCmd,
}

#[derive(Subcommand)]
enum SubCmd {
    Choice1,
    Choice2,
    Choice3(NestStruct),
}

#[derive(Args, Debug)]
struct NestStruct {
    data: i32,
}

fn main() {
    let cli = Cli::parse();
    println!("First CL Arg: {}", cli.carg1);
    println!("Second CL Arg: {}", cli.carg2);
    println!("First Option: {}", cli.fopt);
    println!("Second Option: {:?}", cli.sopt.as_deref());
    match cli.cmd {
        SubCmd::Choice1 => println!("This is choice 1"),
        SubCmd::Choice2 => println!("This is choice 2"),
        SubCmd::Choice3(stt) => println!("struct data: {:?}", stt),
    }
}
