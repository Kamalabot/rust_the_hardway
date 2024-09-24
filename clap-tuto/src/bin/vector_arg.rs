use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// This is mandatory
    name: Vec<String>,
    #[arg(short)]
    /// this is a arg vector
    argname: Vec<String>,
    /// THis is optional
    #[arg(last = true)]
    optname: Option<Vec<String>>,
}
// vector_arg one two -a 5 6 -- one two
fn main() {
    let cli = Cli::parse();
    println!("The vector is {:?}", cli.name);
    println!("The vector is {:?}", cli.argname);
    println!("The option vector is {:?}", cli.optname.as_deref());
}
