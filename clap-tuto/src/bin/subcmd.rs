use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    name: String,
}
// subcmd add -h
// subcmd add somename

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add(args) => println!("Given data: {}", args.name),
    }
}
