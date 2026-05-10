use strontium::machine::register::{RegisterType, RegisterValue};

/// A pattern for multimethod dispatch — either a specific value, a type, or a wildcard.
#[derive(Debug, Clone, PartialEq)]
pub enum DispatchPattern {
    Value(RegisterValue),
    Type(RegisterType),
    Any,
}

impl DispatchPattern {
    pub fn precedence(&self) -> usize {
        match self {
            DispatchPattern::Value(_) => 3,
            DispatchPattern::Type(_) => 2,
            DispatchPattern::Any => 1,
        }
    }
}
