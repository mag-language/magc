pub mod token;
pub mod scanner;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A file containing Mag code.
    input: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.input)
}
