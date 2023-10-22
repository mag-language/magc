use std::string::String;

use strontium::machine::instruction::Instruction;
use strontium::machine::register::RegisterValue::*;

use crate::types::{CompilerResult, Expression, ExpressionKind, Literal};
use crate::compiler::Compiler;

use super::Compilelet;

/// A compilelet for literal expressions like integers, floats, strings and booleans.
/// 
/// This will find the literal value in the source string using the `start_pos` and `end_pos` 
/// properties of the `Expression`, pull out the literal as a string from the source code,
/// parse it into a value and finally store it in a register using the `LOAD` instruction.
pub struct LiteralCompilelet;

impl Compilelet for LiteralCompilelet {
    fn compile(
        &self,
        compiler:   &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut instructions = vec![];

        if let ExpressionKind::Literal(literal) = expression.kind {
            let literal_string = compiler.lexer.get_literal_string(
                expression.start_pos,
                expression.end_pos,
            );
            let value = match literal {
                Literal::Int => Int64(literal_string.unwrap().parse::<i64>().unwrap()),
                Literal::Float => Float64(literal_string.unwrap().parse::<f64>().unwrap()),
                Literal::String => String(literal_string.unwrap()),
                Literal::Boolean => Boolean(literal_string.unwrap().parse::<bool>().unwrap()),
            };    

            // Define a LOAD instruction to find an empty register and load the value into it.
            instructions.push(Instruction::LOAD {
                value,
                register: target_register.unwrap_or(compiler.registers.allocate_register()),
            });
        }

        Ok(instructions)
    }
}