use super::Compilelet;
use crate::compiler::Compiler;
use crate::types::{CompilerResult, Expression, ExpressionKind, Pattern, PairPattern};
use strontium::machine::instruction::Instruction;

pub struct RecordCompilelet;

impl Compilelet for RecordCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        _target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        compiler.context.record_fields.clear();
        let mut instructions = vec![];

        if let ExpressionKind::Pattern(pattern) = expression.kind {
            compile_record_pattern(&pattern, compiler, &mut instructions)?;
        }

        Ok(instructions)
    }
}

/// Walk a record subject pattern, compiling each field's value into a register
/// and storing field_name → register in compiler.context.record_fields.
fn compile_record_pattern(
    pattern: &Pattern,
    compiler: &mut Compiler,
    instructions: &mut Vec<Instruction>,
) -> CompilerResult<()> {
    match pattern {
        Pattern::Record(rp) => {
            compile_record_field(&rp.name, &rp.value, compiler, instructions)?;
        }
        Pattern::Pair(PairPattern { left, right }) => {
            compile_record_pattern(left, compiler, instructions)?;
            compile_record_pattern(right, compiler, instructions)?;
        }
        _ => {}
    }
    Ok(())
}

fn compile_record_field(
    name: &str,
    value: &Pattern,
    compiler: &mut Compiler,
    instructions: &mut Vec<Instruction>,
) -> CompilerResult<()> {
    match value {
        Pattern::Value(vp) => {
            let reg = compiler.registers.allocate_register();
            let mut field_instr =
                compiler.compile_expression(*vp.expression.clone(), Some(reg.clone()))?;
            instructions.append(&mut field_instr);
            compiler.context.record_fields.insert(name.to_string(), reg);
        }
        Pattern::Variable(vp) => {
            if let Some(var_name) = &vp.name {
                let reg = if let Some(src) =
                    compiler.context.register_bindings.get(var_name).cloned()
                {
                    src
                } else if compiler.context.local_variables.contains(var_name) {
                    let r = compiler.registers.allocate_register();
                    instructions.push(Instruction::LoadLocal {
                        name: var_name.clone(),
                        register: r.clone(),
                    });
                    r
                } else {
                    var_name.clone()
                };
                compiler.context.record_fields.insert(name.to_string(), reg);
            }
        }
        Pattern::Tuple(tp) => {
            // Compile each tuple element into an indexed sub-field: "fieldname.0", "fieldname.1", ...
            // Also store a non-Empty marker under "fieldname" so `nothing` checks work correctly.
            let elements = flatten_pair(&tp.child);
            let mut first_reg: Option<String> = None;
            for (i, elem) in elements.iter().enumerate() {
                let key = format!("{}.{}", name, i);
                if let Pattern::Value(vp) = elem {
                    let reg = compiler.registers.allocate_register();
                    let mut field_instr =
                        compiler.compile_expression(*vp.expression.clone(), Some(reg.clone()))?;
                    instructions.append(&mut field_instr);
                    if first_reg.is_none() {
                        first_reg = Some(reg.clone());
                    }
                    compiler.context.record_fields.insert(key, reg);
                }
            }
            // Point the field name at the first element as a non-Empty marker.
            if let Some(marker) = first_reg {
                compiler.context.record_fields.insert(name.to_string(), marker);
            }
        }
        _ => {}
    }
    Ok(())
}

/// Flatten a nested PairPattern into a flat list of element patterns.
pub fn flatten_pair(pattern: &Pattern) -> Vec<&Pattern> {
    match pattern {
        Pattern::Pair(pp) => {
            let mut v = flatten_pair(&pp.left);
            v.extend(flatten_pair(&pp.right));
            v
        }
        _ => vec![pattern],
    }
}
