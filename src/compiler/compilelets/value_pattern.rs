use std::string::String;

use strontium::machine::instruction::Instruction;

use crate::types::{
    CompilerResult,
    Expression,
    ExpressionKind,
};
use crate::compiler::Compiler;

use super::Compilelet;

/// A compilelet for method calls.
pub struct ValuePatternCompilelet;

impl Compilelet for ValuePatternCompilelet {
    fn compile(
        &self,
        compiler:   &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut instructions = vec![];

        if let ExpressionKind::Pattern(pattern) = expression.kind {
            let value_pattern = pattern.expect_value().unwrap();
            instructions.append(&mut compiler.compile_expression(
                *value_pattern.expression,
                target_register,
            )?);
        }

        Ok(instructions)
    }
}