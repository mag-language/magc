use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind};
use strontium::machine::instruction::Instruction;
use strontium::machine::register::RegisterValue;

pub struct ConditionalCompilelet;

impl Compilelet for ConditionalCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let ExpressionKind::Conditional(cond) = expression.kind else {
            return Ok(vec![]);
        };

        let mut instructions = vec![];

        let cond_reg = compiler.registers.allocate_register();
        instructions.extend(compiler.compile_expression(*cond.condition, Some(cond_reg.clone()))?);

        let else_label = compiler.alloc_label();
        let end_label = compiler.alloc_label();

        instructions.push(Instruction::JumpCToLabel {
            id: else_label,
            conditional_address: cond_reg,
        });

        let dest = target_register.unwrap_or_else(|| compiler.registers.allocate_register());
        instructions.extend(compiler.compile_expression(*cond.then_arm, Some(dest.clone()))?);

        instructions.push(Instruction::JumpToLabel { id: end_label });

        instructions.push(Instruction::LabelTarget { id: else_label });
        match cond.else_arm {
            Some(else_expr) => {
                instructions.extend(compiler.compile_expression(*else_expr, Some(dest.clone()))?);
            }
            None => {
                instructions.push(Instruction::Load {
                    value: RegisterValue::Empty,
                    register: dest.clone(),
                });
            }
        }
        instructions.push(Instruction::LabelTarget { id: end_label });

        Ok(instructions)
    }
}
