use super::Compilelet;
use crate::compiler::Compiler;
use crate::compiler::compilelets::record::flatten_pair;
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

        // Compile the subject into its register(s).
        // For record subjects this also populates compiler.context.record_fields.
        let subject_reg = compiler.registers.allocate_register();
        instructions.extend(
            compiler.compile_expression(*match_expr.subject, Some(subject_reg.clone()))?,
        );

        // Capture the record fields populated during subject compilation.
        let record_fields = compiler.context.record_fields.clone();
        let is_record_match = !record_fields.is_empty();

        // Each arm gets (body_label, skip_label).
        let arm_labels: Vec<(usize, usize)> = match_expr
            .arms
            .iter()
            .map(|_| (compiler.alloc_label(), compiler.alloc_label()))
            .collect();

        // Emit dispatch chain.
        for (arm, (body_label, skip_label)) in match_expr.arms.iter().zip(arm_labels.iter()) {
            if is_record_match {
                compile_record_dispatch(
                    &arm.pattern,
                    &record_fields,
                    *body_label,
                    *skip_label,
                    compiler,
                    &mut instructions,
                )?;
            } else {
                let dp = pattern_to_dispatch_pattern(&arm.pattern, compiler);
                match dp {
                    DispatchPattern::Record(_) => {
                        instructions.push(Instruction::JumpToLabel { id: *body_label });
                    }
                    DispatchPattern::Any => {
                        instructions.push(Instruction::JumpToLabel { id: *body_label });
                    }
                    DispatchPattern::Value(val) => {
                        let val_reg = compiler.registers.allocate_register();
                        let cmp_reg = compiler.registers.allocate_register();
                        instructions
                            .push(Instruction::Load { value: val, register: val_reg.clone() });
                        instructions.push(Instruction::Compare {
                            method: ComparisonMethod::EQ,
                            operand1: subject_reg.clone(),
                            operand2: val_reg,
                            destination: cmp_reg.clone(),
                        });
                        instructions.push(Instruction::JumpCToLabel {
                            id: *skip_label,
                            conditional_address: cmp_reg,
                        });
                        instructions.push(Instruction::JumpToLabel { id: *body_label });
                        instructions.push(Instruction::LabelTarget { id: *skip_label });
                    }
                    DispatchPattern::Type(rt) => {
                        let type_reg = compiler.registers.allocate_register();
                        let expected_reg = compiler.registers.allocate_register();
                        let cmp_reg = compiler.registers.allocate_register();
                        instructions.push(Instruction::LoadType {
                            source: subject_reg.clone(),
                            destination: type_reg.clone(),
                        });
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
                        instructions.push(Instruction::JumpCToLabel {
                            id: *skip_label,
                            conditional_address: cmp_reg,
                        });
                        instructions.push(Instruction::JumpToLabel { id: *body_label });
                        instructions.push(Instruction::LabelTarget { id: *skip_label });
                    }
                }
            }
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
        for (arm, (body_label, _)) in match_expr.arms.iter().zip(arm_labels.iter()) {
            instructions.push(Instruction::LabelTarget { id: *body_label });

            // Bind variables from the arm pattern.
            if is_record_match {
                let bindings =
                    collect_record_bindings(&arm.pattern, &record_fields, compiler);
                for (var_name, src_reg) in &bindings {
                    let bind_reg = compiler.registers.allocate_register();
                    instructions.push(Instruction::Copy {
                        source: src_reg.clone(),
                        destination: bind_reg.clone(),
                    });
                    compiler.context.register_bindings.insert(var_name.clone(), bind_reg);
                }

                instructions.extend(
                    compiler.compile_expression(*arm.body.clone(), Some(dest.clone()))?,
                );

                for (var_name, _) in &bindings {
                    compiler.context.register_bindings.remove(var_name);
                }
            } else {
                let binding = arm_binding(&arm.pattern);
                if let Some(name) = &binding {
                    let bind_reg = compiler.registers.allocate_register();
                    instructions.push(Instruction::Copy {
                        source: subject_reg.clone(),
                        destination: bind_reg.clone(),
                    });
                    compiler.context.register_bindings.insert(name.clone(), bind_reg);
                }

                instructions.extend(
                    compiler.compile_expression(*arm.body.clone(), Some(dest.clone()))?,
                );

                if let Some(name) = &binding {
                    compiler.context.register_bindings.remove(name);
                }
            }

            instructions.push(Instruction::JumpToLabel { id: end_label });
        }

        instructions.push(Instruction::LabelTarget { id: end_label });
        compiler.context.record_fields.clear();

        Ok(instructions)
    }
}

/// Emit a dispatch chain for a record pattern case arm.
/// Jumps to skip_label if any required field doesn't match; falls through to body_label on match.
fn compile_record_dispatch(
    pattern: &Pattern,
    record_fields: &std::collections::HashMap<String, String>,
    body_label: usize,
    skip_label: usize,
    compiler: &mut Compiler,
    instructions: &mut Vec<Instruction>,
) -> CompilerResult<()> {
    let any = emit_field_checks(pattern, record_fields, skip_label, compiler, instructions)?;

    if any {
        // All checks passed — jump to body.
        instructions.push(Instruction::JumpToLabel { id: body_label });
        instructions.push(Instruction::LabelTarget { id: skip_label });
    } else {
        // No checks emitted (all Any) — unconditional jump to body.
        instructions.push(Instruction::JumpToLabel { id: body_label });
    }
    Ok(())
}

/// Recursively emit Compare+JumpC instructions for field patterns that require a value check.
/// Returns true if at least one check was emitted (i.e. the skip_label is reachable and needed).
fn emit_field_checks(
    pattern: &Pattern,
    record_fields: &std::collections::HashMap<String, String>,
    skip_label: usize,
    compiler: &mut Compiler,
    instructions: &mut Vec<Instruction>,
) -> CompilerResult<bool> {
    match pattern {
        Pattern::Record(rp) => {
            emit_record_field_check(&rp.name, &rp.value, record_fields, skip_label, compiler, instructions)
        }
        Pattern::Pair(pp) => {
            let l = emit_field_checks(&pp.left, record_fields, skip_label, compiler, instructions)?;
            let r = emit_field_checks(&pp.right, record_fields, skip_label, compiler, instructions)?;
            Ok(l || r)
        }
        _ => Ok(false),
    }
}

fn emit_record_field_check(
    field_name: &str,
    value_pattern: &Pattern,
    record_fields: &std::collections::HashMap<String, String>,
    skip_label: usize,
    compiler: &mut Compiler,
    instructions: &mut Vec<Instruction>,
) -> CompilerResult<bool> {
    match value_pattern {
        Pattern::Value(vp) => {
            if let Some(field_reg) = record_fields.get(field_name) {
                let val_reg = compiler.registers.allocate_register();
                let cmp_reg = compiler.registers.allocate_register();
                let mut val_instr =
                    compiler.compile_expression(*vp.expression.clone(), Some(val_reg.clone()))?;
                instructions.append(&mut val_instr);
                instructions.push(Instruction::Compare {
                    method: ComparisonMethod::EQ,
                    operand1: field_reg.clone(),
                    operand2: val_reg,
                    destination: cmp_reg.clone(),
                });
                // JumpC fires on Boolean(false) — skip when NOT equal
                instructions.push(Instruction::JumpCToLabel {
                    id: skip_label,
                    conditional_address: cmp_reg,
                });
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Pattern::Tuple(tp) => {
            // Match tuple elements by index: check "field.0", "field.1", ...
            let elements = flatten_pair(&tp.child);
            let mut any = false;
            for (i, elem) in elements.iter().enumerate() {
                let key = format!("{}.{}", field_name, i);
                any |= emit_record_field_check(&key, elem, record_fields, skip_label, compiler, instructions)?;
            }
            Ok(any)
        }
        // Variable or Any patterns always match — no check needed.
        _ => Ok(false),
    }
}

/// Walk a record case pattern and return (variable_name, source_register) pairs for binding.
fn collect_record_bindings(
    pattern: &Pattern,
    record_fields: &std::collections::HashMap<String, String>,
    _compiler: &Compiler,
) -> Vec<(String, String)> {
    let mut bindings = vec![];
    collect_bindings_inner(pattern, record_fields, &mut bindings);
    bindings
}

fn collect_bindings_inner(
    pattern: &Pattern,
    record_fields: &std::collections::HashMap<String, String>,
    out: &mut Vec<(String, String)>,
) {
    match pattern {
        Pattern::Record(rp) => {
            match rp.value.as_ref() {
                Pattern::Variable(VariablePattern { name: Some(n), .. }) if n != "_" => {
                    if let Some(reg) = record_fields.get(&rp.name) {
                        out.push((n.clone(), reg.clone()));
                    }
                }
                Pattern::Tuple(tp) => {
                    // Bind tuple elements by index: "field.0" → var_a, "field.1" → var_b
                    let elements = flatten_pair(&tp.child);
                    for (i, elem) in elements.iter().enumerate() {
                        let key = format!("{}.{}", rp.name, i);
                        if let Pattern::Variable(VariablePattern { name: Some(n), .. }) = elem {
                            if n != "_" {
                                if let Some(reg) = record_fields.get(&key) {
                                    out.push((n.clone(), reg.clone()));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Pattern::Pair(pp) => {
            collect_bindings_inner(&pp.left, record_fields, out);
            collect_bindings_inner(&pp.right, record_fields, out);
        }
        _ => {}
    }
}

/// Extract the variable name to bind from a simple (non-record) case arm pattern.
fn arm_binding(pattern: &Pattern) -> Option<String> {
    match pattern {
        Pattern::Variable(VariablePattern { name: Some(n), .. }) if n != "_" => Some(n.clone()),
        _ => None,
    }
}

/// Convert a non-record case arm Pattern to a DispatchPattern for shim generation.
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
        Pattern::Value(vp) => match &vp.expression.kind {
            ExpressionKind::Literal(crate::types::Literal::Int) => {
                if let Ok(lex) =
                    compiler.parser.get_lexeme(vp.expression.start_pos, vp.expression.end_pos)
                {
                    if let Ok(n) = lex.parse::<i64>() {
                        return DispatchPattern::Value(RegisterValue::Int64(n));
                    }
                }
                DispatchPattern::Any
            }
            ExpressionKind::Literal(crate::types::Literal::Float) => {
                if let Ok(lex) =
                    compiler.parser.get_lexeme(vp.expression.start_pos, vp.expression.end_pos)
                {
                    if let Ok(n) = lex.parse::<f64>() {
                        return DispatchPattern::Value(RegisterValue::Float64(n));
                    }
                }
                DispatchPattern::Any
            }
            _ => DispatchPattern::Any,
        },
        _ => DispatchPattern::Any,
    }
}
