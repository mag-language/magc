use strontium::machine::register::{RegisterType, RegisterValue};

/// A dispatch check against a specific register.
///
/// Record calls are still a single argument, but each named field is compiled
/// into a stable register like `arg.name` so dispatch can inspect it.
#[derive(Debug, Clone, PartialEq)]
pub struct DispatchFieldPattern {
    pub register: String,
    pub pattern: DispatchPattern,
}

/// A pattern for multimethod dispatch — either a specific value, a type, or a wildcard.
#[derive(Debug, Clone, PartialEq)]
pub enum DispatchPattern {
    Record(Vec<DispatchFieldPattern>),
    Value(RegisterValue),
    Type(RegisterType),
    Any,
}

impl DispatchPattern {
    pub fn precedence(&self) -> usize {
        match self {
            DispatchPattern::Record(fields) => {
                10 + fields
                    .iter()
                    .map(|field| field.pattern.precedence())
                    .sum::<usize>()
            }
            DispatchPattern::Value(_) => 3,
            DispatchPattern::Type(_) => 2,
            DispatchPattern::Any => 1,
        }
    }
}
