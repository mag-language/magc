pub type CompilerResult<R> = Result<R, CompilerError>;

pub enum CompilerError {
    LexerError(LexerError),
    ParserError(ParserError),
}

pub enum LexerError {

}

pub enum ParserError {

}