use super::Compilelet;
use crate::compiler::linearizable::store_value;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, Pattern};
use strontium::machine::instruction::Instruction;

/// Compiles record and tuple values, like `name: "Dan", pals: ("Sam", "Ed")`, by storing
/// their members in registers below the target register.
pub struct RecordCompilelet;

impl Compilelet for RecordCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let root = target_register.unwrap_or_else(|| compiler.registers.allocate_register());
        store_value(&expression, &root, &[], compiler)
    }
}

/// Flatten a nested PairPattern into a flat list of element patterns.
pub fn flatten_pair(pattern: &Pattern) -> Vec<&Pattern> {
    match pattern {
        Pattern::Pair(pp) => {
            let mut v = flatten_pair(&pp.left);
            v.extend(flatten_pair(&pp.right));
            v
        }
        _ => vec![pattern],
    }
}
