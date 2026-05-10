use super::Compilelet;
use crate::compiler::Compiler;
use crate::dispatch::DispatchPattern;
use crate::types::{CompilerResult, Expression, ExpressionKind, Pattern, VariablePattern};
use strontium::machine::instruction::{ComparisonMethod, Instruction};
use strontium::machine::register::{RegisterType, RegisterValue};

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

        // Compile the subject into a fresh register.
        let subject_reg = compiler.registers.allocate_register();
        instructions.extend(compiler.compile_expression(*match_expr.subject, Some(subject_reg.clone()))?);

        // Each arm gets a body label and a skip label.
        let arm_labels: Vec<(usize, usize)> = match_expr.arms.iter()
            .map(|_| (compiler.alloc_label(), compiler.alloc_label()))
            .collect();

        // Emit the dispatch chain (shim logic, identical to build_method_block).
        for (arm, (body_label, skip_label)) in match_expr.arms.iter().zip(arm_labels.iter()) {
            let dp = pattern_to_dispatch_pattern(&arm.pattern, compiler);
            match dp {
                DispatchPattern::Any => {
                    instructions.push(Instruction::JumpToLabel { id: *body_label });
                }
                DispatchPattern::Value(val) => {
                    let val_reg = compiler.registers.allocate_register();
                    let cmp_reg = compiler.registers.allocate_register();
                    instructions.push(Instruction::Load { value: val, register: val_reg.clone() });
                    instructions.push(Instruction::Compare {
                        method: ComparisonMethod::EQ,
                        operand1: subject_reg.clone(),
                        operand2: val_reg,
                        destination: cmp_reg.clone(),
                    });
                    instructions.push(Instruction::JumpCToLabel { id: *skip_label, conditional_address: cmp_reg });
                    instructions.push(Instruction::JumpToLabel { id: *body_label });
                    instructions.push(Instruction::LabelTarget { id: *skip_label });
                }
                DispatchPattern::Type(rt) => {
                    let type_reg = compiler.registers.allocate_register();
                    let expected_reg = compiler.registers.allocate_register();
                    let cmp_reg = compiler.registers.allocate_register();
                    instructions.push(Instruction::LoadType { source: subject_reg.clone(), destination: type_reg.clone() });
                    instructions.push(Instruction::Load {
                        value: RegisterValue::Int64(rt as i64),
                        register: expected_reg.clone(),
                    });
                    instructions.push(Instruction::Compare {
                        method: ComparisonMethod::EQ,
                        operand1: type_reg,
                        operand2: expected_reg,
                        destination: cmp_reg.clone(),
                    });
                    instructions.push(Instruction::JumpCToLabel { id: *skip_label, conditional_address: cmp_reg });
                    instructions.push(Instruction::JumpToLabel { id: *body_label });
                    instructions.push(Instruction::LabelTarget { id: *skip_label });
                }
            }
        }

        // Else arm (no match in case arms).
        instructions.extend(compiler.compile_expression(*match_expr.else_arm, Some(dest.clone()))?);
        instructions.push(Instruction::JumpToLabel { id: end_label });

        // Emit arm bodies.
        for (arm, (body_label, _)) in match_expr.arms.iter().zip(arm_labels.iter()) {
            instructions.push(Instruction::LabelTarget { id: *body_label });

            // Bind the variable name (if any) to the subject register.
            let binding = arm_binding(&arm.pattern);
            if let Some(name) = &binding {
                let bind_reg = compiler.registers.allocate_register();
                instructions.push(Instruction::Copy {
                    source: subject_reg.clone(),
                    destination: bind_reg.clone(),
                });
                compiler.context.register_bindings.insert(name.clone(), bind_reg);
            }

            instructions.extend(compiler.compile_expression(*arm.body.clone(), Some(dest.clone()))?);

            if let Some(name) = &binding {
                compiler.context.register_bindings.remove(name);
            }

            instructions.push(Instruction::JumpToLabel { id: end_label });
        }

        instructions.push(Instruction::LabelTarget { id: end_label });

        Ok(instructions)
    }
}

/// Extract the variable name to bind from a case arm pattern, if any.
fn arm_binding(pattern: &Pattern) -> Option<String> {
    match pattern {
        Pattern::Variable(VariablePattern { name: Some(n), .. }) if n != "_" => Some(n.clone()),
        _ => None,
    }
}

/// Convert a case arm Pattern to a DispatchPattern for shim generation.
fn pattern_to_dispatch_pattern(pattern: &Pattern, compiler: &Compiler) -> DispatchPattern {
    match pattern {
        Pattern::Variable(VariablePattern { type_id: Some(t), .. }) => match t.as_str() {
            "Int" => DispatchPattern::Type(RegisterType::Int64),
            "Float" => DispatchPattern::Type(RegisterType::Float64),
            "String" => DispatchPattern::Type(RegisterType::String),
            "Bool" => DispatchPattern::Type(RegisterType::Boolean),
            _ => DispatchPattern::Any,
        },
        Pattern::Variable(_) => DispatchPattern::Any,
        Pattern::Value(vp) => {
            match &vp.expression.kind {
                ExpressionKind::Literal(crate::types::Literal::Int) => {
                    if let Ok(lex) = compiler.parser.get_lexeme(vp.expression.start_pos, vp.expression.end_pos) {
                        if let Ok(n) = lex.parse::<i64>() {
                            return DispatchPattern::Value(RegisterValue::Int64(n));
                        }
                    }
                    DispatchPattern::Any
                }
                ExpressionKind::Literal(crate::types::Literal::Float) => {
                    if let Ok(lex) = compiler.parser.get_lexeme(vp.expression.start_pos, vp.expression.end_pos) {
                        if let Ok(n) = lex.parse::<f64>() {
                            return DispatchPattern::Value(RegisterValue::Float64(n));
                        }
                    }
                    DispatchPattern::Any
                }
                _ => DispatchPattern::Any,
            }
        }
        _ => DispatchPattern::Any,
    }
}
