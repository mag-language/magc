use crate::types::Method;
use crate::parser::Parser;
use crate::types::{CompilerResult, CompilerError, Pattern};

/// A collection of methods with different function signatures which share a common name.
#[derive(Debug, Clone)]
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

    pub fn linearize(&self, parser: &Parser, pattern: Option<Pattern>) -> CompilerResult<Method> {
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

        if matching_methods.len() > 0 {
            let (linearized_method, _precedence) = matching_methods[0];

            Ok(linearized_method.clone())
        } else {
            Err(CompilerError::MethodSignatureNotFound {
                method_name: self.name.clone(),
                pattern,
            })
        }
    }

    pub fn add_method(&mut self, parser: &Parser, method: Method) -> CompilerResult<()> {
        // Break out early if the method already exists.
        for m in &self.methods {
            match [m.signature.clone(), method.signature.clone()] {
                [Some(p1), Some(p2)] => {
                    if p1.matches_with(parser, p2.clone()) {
                        return Err(CompilerError::DuplicateMethodSignature {
                            method_name: method.name,
                            signature: method.signature
                        });
                    }
                },
                // no match.
                [Some(_), None] | [None, Some(_)] => {},
                [None, None] => {
                    println!("MATCH!");

                    return Err(CompilerError::DuplicateMethodSignature {
                        method_name: method.name,
                        signature: method.signature
                    })
                },
            }
        }
        self.methods.push(method);

        Ok(())
    }
}