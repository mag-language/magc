//! A compiler library which provides various methods and data structures used to compile Mag programs 
//! for a set of back-end architectures, including Strontium bytecode. We'll be looking into ahead-of-time 
//! compilation in the future, possibly using [`cranelift`](https://docs.rs/cranelift/latest/cranelift/) 
//! or one of the various crates that provide bindings to LLVM.
//!
//! ## Data Structures
//!
//! * [`Token`]: The smallest possible entity in the language, like a number, an identifier, or a keyword.
//! * [`Expression`]: A syntactic entity that may be evaluated to determine its value.
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
//! This tree of expressions can then be executed on an interpreter or compiled to some binary format like
//! Strontium bytecode. There is no code generation or interpreter code yet.

pub mod lexer;
pub mod parser;
pub mod types;
pub mod type_system;

pub use self::types::*;