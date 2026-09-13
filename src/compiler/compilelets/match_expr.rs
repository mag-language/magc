use super::Compilelet;
use crate::compiler::linearizable::{store_value, Linearizable};
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind};
use strontium::machine::instruction::Instruction;
use strontium::machine::register::RegisterValue;

pub struct MatchCompilelet;

impl Compilelet for MatchCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let ExpressionKind::Match(match_expr) = expression.kind else {
            return Ok(vec![]);
        };

        let mut instructions = vec![];
        let dest = target_register.unwrap_or_else(|| compiler.registers.allocate_register());
        let end_label = compiler.alloc_label();

        // Store the subject below a fresh root, resetting every register an arm reads.
        let root = compiler.registers.allocate_register();
        let mut reset = vec![root.clone()];
        for arm in &match_expr.arms {
            reset.extend(arm.pattern.registers(&root));
        }
        instructions.extend(store_value(&match_expr.subject, &root, &reset, compiler)?);

        // Test the arms in source order; the first matching arm wins.
        let body_labels: Vec<usize> = match_expr
            .arms
            .iter()
            .map(|_| compiler.alloc_label())
            .collect();

        for (arm, body_label) in match_expr.arms.iter().zip(&body_labels) {
            let skip_label = compiler.alloc_label();
            instructions.extend(arm.pattern.linearize(&root, skip_label, compiler)?);
            instructions.push(Instruction::JumpToLabel { id: *body_label });
            instructions.push(Instruction::LabelTarget { id: skip_label });
        }

        // Else arm (no match). Without an else arm, the match evaluates to `nothing`,
        // like `if` without `else`. Either way, jump past the arm bodies.
        match match_expr.else_arm {
            Some(else_arm) => {
                instructions.extend(compiler.compile_expression(*else_arm, Some(dest.clone()))?);
            }
            None => {
                instructions.push(Instruction::Load {
                    value: RegisterValue::Empty,
                    register: dest.clone(),
                });
            }
        }
        instructions.push(Instruction::JumpToLabel { id: end_label });

        // Emit arm bodies.
        for (arm, body_label) in match_expr.arms.iter().zip(&body_labels) {
            instructions.push(Instruction::LabelTarget { id: *body_label });

            // Bind the arm's variables, copying each value so calls in the body cannot clobber it.
            let bindings = arm.pattern.bindings(&root);
            for (name, register) in &bindings {
                let bind_reg = compiler.registers.allocate_register();
                instructions.push(Instruction::Copy {
                    source: register.clone(),
                    destination: bind_reg.clone(),
                });
                compiler.context.register_bindings.insert(name.clone(), bind_reg);
            }

            instructions.extend(compiler.compile_expression(*arm.body.clone(), Some(dest.clone()))?);

            for (name, _) in &bindings {
                compiler.context.register_bindings.remove(name);
            }

            instructions.push(Instruction::JumpToLabel { id: end_label });
        }

        instructions.push(Instruction::LabelTarget { id: end_label });

        Ok(instructions)
    }
}
