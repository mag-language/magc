//! ![mag banner](https://world-of-music.at/downloads/bird-banner.png)
//!
//! # What's this?
//!
//! [`magc`](https://crates.io/crates/magc) is a compiler library which provides various methods 
//! and data structures used to compile Mag programs into an abstract syntax tree, and finally
//! into a sequence of instructions for the [Strontium machine](https://crates.io/crates/strontium)
//! to actually perform the desired computation.
//!
//! A simple infix addition operation like `1 + 2` can be converted to a token sequence like this:
//!
//! ```rust
//! use magc::{Lexer, Parser};
//!
//! // Add some text to the lexer's input buffer and parse it into a sequence of tokens.
//! let tokens = Lexer::new()
//!     .add_text("1 + 2");
//!     .parse();
//!
//! assert_eq!(
//!     tokens,
//!     Ok(
//!        vec![
//!            Token {
//!                kind: Literal(
//!                    Int,
//!                ),
//!                start_pos: 0,
//!                end_pos: 1,
//!            },
//!            Token {
//!                kind: Plus,
//!                start_pos: 2,
//!                end_pos: 3,
//!            },
//!            Token {
//!                kind: Literal(
//!                    Int,
//!                ),
//!                start_pos: 4,
//!                end_pos: 5,
//!            },
//!        ]
//!     ),
//! );
//! ```
//!
//! ## Data Structures
//!
//! * [`Token`]: The smallest possible entity in the language, like a number, an identifier, or a keyword.
//! * [`Expression`]: A syntactic entity that may be evaluated to determine its value.
//! * [`Pattern`]: A pattern that can be matched with an [`Expression`] to enable complex flow control
//! and destructuring.
//!
//! ## Compilation Pipeline
//!
//! Processing starts with the [`Lexer`] struct, which converts a UTF-8 source string to a linear sequence 
//! of [`Token`]s while annotating them with positioning data and the original string. This enables us to 
//! iterate easily over our program in later stages of the pipeline and frees us from having to deal
//! with strings during parsing. Literals and similar token kinds don't actually contain their values,
//! instead the lexeme string is stored in the token. This enables us to lazily decode their values, so we
//! can store the structure in a HashMap without issues. `f64` does not implement `Hash`.
//!
//! The resulting array of tokens is fed into a [`Parser`] instance and then iteratively turned into a tree 
//! of expressions representing the structure of the original source string. In case the compiler encounters 
//! some form of invalid code, it will return an error which is then reported by the runtime.
//!
//! ## Credits
//!
//! Mag is based on the Magpie language by [Robert Nystrom](http://stuffwithstuff.com/), who is 
//! a language engineer at Google with [a blog and a lot of amazing ideas](http://journal.stuffwithstuff.com/category/magpie/).
//! His various blog posts are what started and inspired this project, and I plan on continuing his legacy even if the original codebase ceases further development.
//!
//! However, since there are a few syntactical differences to the original Magpie language, the two languages are *source-incompatible* and thus have different names. In particular, Bob's implementation substitutes the dot commonly used for calling methods on objects with a space (usually a meaningless character), which I find rather unintuitive, especially for new programmers.

#![feature(type_ascription)]

pub mod compiler;
pub mod helpers;
pub mod lexer;
pub mod parser;
pub mod types;
pub mod type_system;

pub use self::types::*;
pub use self::type_system::*;

pub use self::lexer::Lexer;
pub use self::parser::Parser;