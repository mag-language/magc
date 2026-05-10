use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind};
use strontium::machine::instruction::Instruction;

pub struct VarCompilelet;

impl Compilelet for VarCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let ExpressionKind::Var(var_decl) = expression.kind else {
            return Ok(vec![]);
        };

        let mut instructions = vec![];
        let val_reg = compiler.registers.allocate_register();
        instructions.extend(compiler.compile_expression(*var_decl.value, Some(val_reg.clone()))?);

        if compiler.context.recursion_depth <= 1 {
            // Top-level: store in a named register so it persists across REPL iterations.
            compiler.context.global_variables.insert(var_decl.name.clone());
            instructions.push(Instruction::Copy {
                source: val_reg.clone(),
                destination: var_decl.name.clone(),
            });
        } else {
            // Inside a method body: use the call-stack frame.
            instructions.push(Instruction::StoreLocal {
                name: var_decl.name.clone(),
                register: val_reg.clone(),
            });
            compiler.context.local_variables.insert(var_decl.name.clone());
        }

        let dest = target_register.unwrap_or_else(|| compiler.registers.allocate_register());
        instructions.push(Instruction::Copy {
            source: val_reg,
            destination: dest,
        });

        Ok(instructions)
    }
}
