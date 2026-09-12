use crate::dispatch::DispatchPattern;
use crate::parser::Parser;
use crate::types::Method;
use crate::types::{CompilerError, CompilerResult, Pattern};

/// A collection of methods with different function signatures which share a common name.
#[derive(Debug, Clone)]
pub struct Multimethod {
    /// The name of this multimethod.
    pub name: String,
    /// Contains pairs of method signatures and bodies.
    pub methods: Vec<Method>,
    /// Dispatch patterns of `methods`, in the same order.
    pub dispatch_patterns: Vec<DispatchPattern>,
}

impl Multimethod {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            methods: vec![],
            dispatch_patterns: vec![],
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
                }
                (Some(..), None) | (None, Some(..)) => {}
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

    /// Register a method with its dispatch pattern.
    ///
    /// The dispatch pattern must be computed when the method is defined: patterns
    /// refer to source positions, and the parser's source is replaced on every
    /// `compile` call (e.g. each REPL line), so it cannot be recomputed later.
    pub fn add_method(
        &mut self,
        method: Method,
        dispatch_pattern: DispatchPattern,
    ) -> CompilerResult<()> {
        if self.dispatch_patterns.contains(&dispatch_pattern) {
            return Err(CompilerError::DuplicateMethodSignature {
                method_name: method.name,
                signature: method.signature,
            });
        }
        self.methods.push(method);
        self.dispatch_patterns.push(dispatch_pattern);
        Ok(())
    }
}
