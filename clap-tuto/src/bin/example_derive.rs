use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(name = "example-derive")]
#[command(author = "Revengineer")]
#[command(version = "1.0")]
#[command(about = "Example CLI App", long_about=None)]
struct Cli {
    #[arg(short = 'f')]
    eff: bool,
    // creating a struct element, to which the short name is given
    #[arg(short = 'p', value_name = "PEAR")]
    pea: Option<String>,
    // here we are giving option strings
    #[arg(last = true)]
    slop: Vec<String>,
    // here we are taking vector strings
    #[arg(short, long, default_value_t=String::from("verbose"))]
    level: String,
}

fn main() {
    let args = Cli::parse();

    // This is what will happen with `myprog -f -p=bob -- sloppy slop slop`...
    println!("-f used: {:?}", args.eff); // -f used: true
    println!("-p's value: {:?}", args.pea); // -p's value: Some("bob")
    println!("'slops' values: {:?}", args.slop); // 'slops' values: Some(["sloppy", "slop", "slop"])

    // Continued program logic goes here...
}
