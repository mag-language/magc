use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind, Pattern};
use strontium::machine::instruction::{Instruction, Interrupt, InterruptKind};

/// Compilelet for variable pattern references.
/// In method bodies, emits LoadLocal. At top level, copies from the named register.
pub struct VariablePatternCompilelet;

impl Compilelet for VariablePatternCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut instructions = Vec::new();

        if let ExpressionKind::Pattern(Pattern::Variable(var_pattern)) = expression.kind {
            if let Some(var_name) = var_pattern.name {
                let dest_register =
                    target_register.unwrap_or_else(|| compiler.registers.allocate_register());

                if compiler.context.local_variables.contains(&var_name) {
                    instructions.push(Instruction::LoadLocal {
                        name: var_name,
                        register: dest_register.clone(),
                    });
                } else if compiler.context.global_variables.contains(&var_name) {
                    instructions.push(Instruction::Copy {
                        source: var_name,
                        destination: dest_register.clone(),
                    });
                }

                if compiler.context.recursion_depth == 1 && compiler.context.repl_mode {
                    instructions.push(Instruction::Interrupt {
                        interrupt: Interrupt {
                            address: dest_register,
                            kind: InterruptKind::Print,
                        },
                    });
                }
            }
        }

        Ok(instructions)
    }
}
