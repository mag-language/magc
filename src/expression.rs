use crate::token::{Literal};

use std::collections::BTreeMap;

type VariablePatternName = Option<String>;
type VariablePatternType = Option<String>;

pub enum Expression<'a> {
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    Pattern(Pattern<'a>),
}

pub enum Pattern<'a> {
    Value(&'a Pattern<'a>),
    Tuple(Vec<Pattern<'a>>),
    Record(BTreeMap<String, Expression<'a>>),
    Variable(VariablePatternName, VariablePatternType),
}