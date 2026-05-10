use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind};
use strontium::machine::instruction::Instruction;

pub struct ReturnCompilelet;

impl Compilelet for ReturnCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        _target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let ExpressionKind::Return(ret_expr) = expression.kind else {
            return Ok(vec![]);
        };

        let mut instructions = vec![];

        let val_reg = compiler.registers.allocate_register();
        instructions.extend(compiler.compile_expression(*ret_expr.value, Some(val_reg.clone()))?);

        instructions.push(Instruction::Copy {
            source: val_reg,
            destination: "ret".to_string(),
        });

        instructions.push(Instruction::Return);

        Ok(instructions)
    }
}
