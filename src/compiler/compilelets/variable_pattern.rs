use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind, Pattern};
use strontium::machine::instruction::Instruction;

/// Compilelet for variable pattern references in method bodies.
/// When a variable like `n` is referenced in a method body, this emits LoadLocal.
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
                // Check if this variable is a local (pattern variable in scope)
                if compiler.context.local_variables.contains(&var_name) {
                    let dest_register =
                        target_register.unwrap_or_else(|| compiler.registers.allocate_register());

                    instructions.push(Instruction::LoadLocal {
                        name: var_name,
                        register: dest_register,
                    });
                }
                // TODO: Handle global variables or throw error for undefined variables
            }
        }

        Ok(instructions)
    }
}
