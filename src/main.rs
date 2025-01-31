use clap::{Args, Parser};
use run::{adder, getter, lister};

mod run;
mod traits;
mod types;

#[derive(Parser)]
enum Cli {
    Add(AddArgs),
    Get(GetArgs),
    List,
}

#[derive(Args)]
#[command(author, version, about, long_about = None)]
struct AddArgs {
    #[arg()]
    title: String,
    #[arg()]
    content: String,
}

#[derive(Args)]
#[command(author, version, about, long_about = None)]
struct GetArgs {
    #[arg()]
    id: i64,
}

fn main() {
    let cli = Cli::parse();
    match cli {
        Cli::Add(args) => adder(args).expect("Adding a Note failed"),
        Cli::Get(args) => getter(args).expect("Geting a note failed"),
        Cli::List => lister().expect("Listing the notes failed"),
    }
}
