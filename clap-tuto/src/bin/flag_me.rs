use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Flagme {
    /// this is a very troublesome one
    #[arg(short, long,action= ArgAction::Count)]
    bugme: u8,
    // bugme: bool, // we can keep it as a flag or count
    // /// more subcommand
    // #[command(subcommand)]
    // cmd_struct: CmdEnum,
}

// derive Subcommand only supports Enum
// #[derive(Subcommand)]
// struct CmdStruct {
// enum CmdEnum {
//     ///Another Bug... Why Not
//     BugHim,
// }

// flag_me -b -b -b
fn main() {
    let cli = Flagme::parse();
    println!("Is there a bug:{}", cli.bugme);
    // match cli.cmd_struct {
    //     CmdEnum::BugHim => println!("Bughim"),
    // }
}
