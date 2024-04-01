use crate::types::Expression;

/// An expression that evaluates to a value.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ValuePattern {
    pub expression: Box<Expression>,
}

impl ValuePattern {
    pub fn desugar(mut self) -> ValuePattern {
        self.expression.desugar();

        ValuePattern {
            expression: self.expression,
        }
    }
}

impl std::fmt::Display for ValuePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.expression)
    }
}
