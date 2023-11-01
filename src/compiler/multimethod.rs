use crate::types::Method;
use crate::parser::Parser;
use crate::types::{CompilerResult, CompilerError, Pattern};

use super::Compiler;

/// A collection of methods with different function signatures which share a common name.
pub struct Multimethod {
    /// The name of this multimethod.
    pub name: String,
    /// Contains pairs of method signatures and bodies.
    pub methods: Vec<Method>,
}

impl Multimethod {
    pub fn new(name: &str) -> Self {
        Self {
            name:    String::from(name),
            methods: vec![],
        }
    }

    pub fn linearize(&self, parser: &mut Parser, pattern: Option<Pattern>) -> CompilerResult<Method> {
        let mut matching_methods = vec![];

        for method in &self.methods {
            match (pattern.clone(), method.signature.clone()) {
                (None, None) => matching_methods.push((method, 0)),
                (Some(p), Some(s)) => {
                    if s.matches_with(parser, p.clone()) {
                        matching_methods.push((method, p.get_precedence()));
                    }
                },
                (Some(..), None) | (None, Some(..)) => {},
            }
        }

        // Sort the resulting method signatures by their pattern's precedence.
        matching_methods.sort_by(|m1, m2| m1.1.cmp(&m2.1));
        let (linearized_method, precedence) = matching_methods[0];

        Ok(linearized_method.clone())
    }

    pub fn add_method(&mut self, method: Method) -> CompilerResult<()> {
        // Break out early if the method already exists.
        for m in &self.methods {
            if m.signature == method.signature {
                return Err(CompilerError::DuplicateMethodSignature {
                    signature: method.signature
                });
            }
        }
        self.methods.push(method);

        Ok(())
    }
}