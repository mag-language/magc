pub type CompilerResult<R> = Result<R, CompilerError>;

#[derive(Debug, Clone)]
pub enum CompilerError {
    Generic(String),
    LexerError(LexerError),
    ParserError(ParserError),
}

#[derive(Debug, Clone)]
pub enum LexerError {

}

#[derive(Debug, Clone)]
pub enum ParserError {
    /// An opening brace, bracket or parenthesis is missing its closing counterpart.
    UnclosedDelimiter,
    /// The file ended unexpectedly.
    UnexpectedEof,
    UnexpectedPattern {
        expected: String,
        found:    String,
    },
}