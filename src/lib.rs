//! A compiler library which provides various methods and data structures used to compile Mag programs 
//! for a set of back-end architectures, including Strontium bytecode.
//!
//! ## Data Structures
//!
//! * [`Token`]: The smallest possible entity in the language, like a number, an identifier, or a keyword.
//! * [`Expression`]: A syntactic entity that may be evaluated to determine its value.
//!
//! ## Compilation Pipeline
//!
//! Processing starts with the [`Lexer`] struct, which converts a UTF-8 source string to a linear sequence 
//! of [`Token`]s while annotating them with positioning data and their original text. This enables us to 
//! iterate easily over our source code in later stages of the pipeline.
//!
//! The resulting array of tokens is then fed into a [`Parser`] instance, which attempts to build a tree 
//! of expressions

pub mod token;
pub mod lexer;
pub mod expression;
pub mod parser;

pub use self::token::Token;
pub use self::lexer::Lexer;
pub use self::expression::Expression;

/*use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A path to a file that contains Mag code.
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.path)
}*/
