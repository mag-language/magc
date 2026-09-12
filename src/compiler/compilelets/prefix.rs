use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind};
use strontium::machine::instruction::{CalculationMethod, Instruction};
use strontium::machine::register::RegisterValue;

pub struct PrefixCompilelet;

impl Compilelet for PrefixCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let ExpressionKind::Prefix(prefix) = expression.kind else {
            return Ok(vec![]);
        };

        use strontium::machine::instruction::Instruction::*;
        use crate::types::TokenKind;

        let dest = target_register.unwrap_or_else(|| compiler.registers.allocate_register());
        let mut instructions = vec![];

        match prefix.operator.kind {
            TokenKind::Plus => {
                instructions.extend(compiler.compile_expression(*prefix.operand, Some(dest))?);
            }
            TokenKind::Minus => {
                use crate::types::{ExpressionKind, Literal};
                let zero = match &prefix.operand.kind {
                    ExpressionKind::Literal(Literal::Float) => RegisterValue::Float64(0.0),
                    _ => RegisterValue::Int64(0),
                };
                let zero_reg = compiler.registers.allocate_register();
                let operand_reg = compiler.registers.allocate_register();
                instructions.push(Load {
                    value: zero,
                    register: zero_reg.clone(),
                });
                instructions.extend(
                    compiler.compile_expression(*prefix.operand, Some(operand_reg.clone()))?,
                );
                instructions.push(Calculate {
                    method: CalculationMethod::SUBTRACT,
                    operand1: zero_reg,
                    operand2: operand_reg,
                    destination: dest,
                });
            }
            _ => {
                return Err(crate::types::CompilerError::Generic(format!(
                    "unsupported prefix operator: {:?}",
                    prefix.operator.kind
                )));
            }
        }

        Ok(instructions)
    }
}
