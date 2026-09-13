use super::linearizable::Linearizable;
use super::Compiler;
use crate::types::{CompilerError, CompilerResult, Method};
use strontium::machine::instruction::Instruction;

/// The register holding the argument of a multimethod call.
pub const ARGUMENT_REGISTER: &str = "arg";

/// One method definition of a multimethod, linearized for dispatch.
#[derive(Debug, Clone)]
pub struct Variant {
    /// Unique identifier of the compiled body, e.g. `fib(0)`.
    pub id: String,
    /// The method definition.
    pub method: Method,
    /// Canonical signature, see [`Linearizable::signature`].
    pub signature: String,
    /// Dispatch precedence; higher wins, see [`Linearizable::get_precedence`].
    pub precedence: usize,
    /// Bytecode that jumps to `on_fail` unless the argument matches this variant.
    pub checks: Vec<Instruction>,
    /// The label jumped to when the argument does not match.
    pub on_fail: usize,
    /// Argument registers read by `checks`.
    pub registers: Vec<String>,
}

impl Variant {
    /// Linearize a method definition.
    ///
    /// This must happen when the method is defined: literal values are read from the
    /// parser's source, which is replaced on every `compile` call (e.g. each REPL line).
    pub fn new(method: Method, compiler: &mut Compiler) -> CompilerResult<Self> {
        let (signature, precedence, registers) = match &method.signature {
            Some(pattern) => (
                pattern.signature(&compiler.parser)?,
                pattern.get_precedence(),
                pattern.registers(ARGUMENT_REGISTER),
            ),
            None => ("_".to_string(), 1, vec![]),
        };

        let on_fail = compiler.alloc_label();
        let checks = match &method.signature {
            Some(pattern) => pattern.linearize(ARGUMENT_REGISTER, on_fail, compiler)?,
            None => vec![],
        };

        Ok(Self {
            id: format!("{}({})", method.name, signature),
            method,
            signature,
            precedence,
            checks,
            on_fail,
            registers,
        })
    }
}

/// A collection of methods with different function signatures which share a common name.
#[derive(Debug, Clone)]
pub struct Multimethod {
    /// The name of this multimethod.
    pub name: String,
    /// The linearized method definitions, in definition order.
    pub variants: Vec<Variant>,
}

impl Multimethod {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            variants: vec![],
        }
    }

    /// Add a variant, rejecting a signature that is already defined.
    pub fn add_variant(&mut self, variant: Variant) -> CompilerResult<()> {
        if self
            .variants
            .iter()
            .any(|existing| existing.signature == variant.signature)
        {
            return Err(CompilerError::DuplicateMethodSignature {
                method_name: variant.method.name,
                signature: variant.method.signature,
            });
        }
        self.variants.push(variant);
        Ok(())
    }

    /// The variants in dispatch order: higher precedence first, equal precedence in
    /// definition order.
    pub fn linearize(&self) -> Vec<&Variant> {
        let mut variants: Vec<&Variant> = self.variants.iter().collect();
        // `sort_by` is stable, which keeps definition order for equal precedence.
        variants.sort_by(|a, b| b.precedence.cmp(&a.precedence));
        variants
    }

    /// Every argument register read by any variant, including the argument register itself.
    pub fn registers(&self) -> Vec<String> {
        let mut registers = vec![ARGUMENT_REGISTER.to_string()];
        for variant in &self.variants {
            registers.extend(variant.registers.iter().cloned());
        }
        registers
    }
}
