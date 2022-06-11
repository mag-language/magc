use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Compiler {
    lexer:  Lexer,
    parser: Parser,
}