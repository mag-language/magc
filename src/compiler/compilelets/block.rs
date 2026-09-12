use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind};
use strontium::machine::instruction::Instruction;

pub struct BlockCompilelet;

impl Compilelet for BlockCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let ExpressionKind::Block(block) = expression.kind else {
            return Ok(vec![]);
        };

        let mut instructions = vec![];
        let children: Vec<_> = block.children.into_iter().collect();
        let last = children.len().saturating_sub(1);

        for (i, child) in children.into_iter().enumerate() {
            let reg = if i == last {
                target_register.clone()
            } else {
                None
            };
            instructions.extend(compiler.compile_expression(child, reg)?);
        }

        Ok(instructions)
    }
}
