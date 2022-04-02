use crate::token::{Literal, Type};

type VariablePatternName = Option<String>;
type VariablePatternType = Option<String>;

pub enum Expression<'a> {
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    Pattern(Pattern<'a>),
}

pub enum Pattern<'a> {
    Value(&'a Pattern),
    Tuple(Vec<Pattern>),
    Record(BTreeMap<String, Expression>),
    Variable(VariablePatternName, VariablePatternType),
}