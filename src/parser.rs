/// A parser which turns a linear token stream into a tree of Mag expressions.
pub struct Parser {
    position: usize,
    source: Vec<Token>,
}