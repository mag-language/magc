use crate::compiler::{Compilelet, Compiler};
use crate::types::{
    CompilerError, CompilerResult, Expression, ExpressionKind, Pattern, ValuePattern,
};
use std::collections::HashSet;
use strontium::machine::instruction::{
    CalculationMethod, ComparisonMethod, Instruction, Interrupt, InterruptKind,
};
use strontium::machine::register::RegisterValue;

pub struct CallCompilelet;

impl Compilelet for CallCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut instructions = Vec::new();

        if let ExpressionKind::Call(call) = expression.kind {
            let method_name = call.name;
            let signature = call.signature.clone();

            match method_name.as_str() {
                "print" => {
                    let value_register = compiler.registers.allocate_register();

                    if let Some(pattern) = signature {
                        instructions.append(&mut self.compile_print_argument(
                            compiler,
                            pattern,
                            value_register.clone(),
                        )?);
                    } else {
                        instructions.push(Instruction::Load {
                            value: RegisterValue::Empty,
                            register: value_register.clone(),
                        });
                    }

                    instructions.push(Instruction::Interrupt {
                        interrupt: Interrupt {
                            address: value_register,
                            kind: InterruptKind::Print,
                        },
                    });

                    let destination_register =
                        target_register.unwrap_or_else(|| compiler.registers.allocate_register());

                    instructions.push(Instruction::Load {
                        value: RegisterValue::Empty,
                        register: destination_register,
                    });
                }

                "sqrt" => {
                    if let Some(pattern) = signature {
                        let arg_register = compiler.registers.allocate_register();
                        instructions.append(&mut self.compile_print_argument(
                            compiler,
                            pattern,
                            arg_register.clone(),
                        )?);

                        let destination_register = target_register
                            .unwrap_or_else(|| compiler.registers.allocate_register());
                        instructions.push(Instruction::Calculate {
                            method: CalculationMethod::SQRT,
                            operand1: arg_register.clone(),
                            operand2: arg_register,
                            destination: destination_register.clone(),
                        });

                        if compiler.context.recursion_depth == 1 && compiler.context.repl_mode {
                            instructions.push(Instruction::Interrupt {
                                interrupt: Interrupt {
                                    address: destination_register,
                                    kind: InterruptKind::Print,
                                },
                            });
                        }
                    }
                }

                // Built-in arithmetic operators
                "+" | "-" | "*" | "/" | "^" | "%" | "~" => {
                    let method = match method_name.as_str() {
                        "+" => CalculationMethod::ADD,
                        "-" => CalculationMethod::SUBTRACT,
                        "*" => CalculationMethod::MULTIPLY,
                        "/" => CalculationMethod::DIVIDE,
                        "^" => CalculationMethod::POWER,
                        "%" => CalculationMethod::MODULO,
                        "~" => CalculationMethod::CONCAT,
                        _ => unreachable!(),
                    };

                    if let Some(Pattern::Pair(pair)) = signature {
                        let left_expr =
                            if let Pattern::Value(ValuePattern { expression }) = *pair.left {
                                *expression
                            } else {
                                unreachable!()
                            };

                        let right_expr =
                            if let Pattern::Value(ValuePattern { expression }) = *pair.right {
                                *expression
                            } else {
                                unreachable!()
                            };

                        let left_register = compiler.registers.allocate_register();
                        instructions.append(
                            &mut compiler
                                .compile_expression(left_expr, Some(left_register.clone()))?,
                        );

                        let right_register = compiler.registers.allocate_register();
                        instructions.append(
                            &mut compiler
                                .compile_expression(right_expr, Some(right_register.clone()))?,
                        );

                        let destination_register = target_register
                            .unwrap_or_else(|| compiler.registers.allocate_register());
                        instructions.push(Instruction::Calculate {
                            method,
                            operand1: left_register,
                            operand2: right_register,
                            destination: destination_register.clone(),
                        });

                        if compiler.context.recursion_depth == 1 && compiler.context.repl_mode {
                            instructions.push(Instruction::Interrupt {
                                interrupt: Interrupt {
                                    address: destination_register,
                                    kind: InterruptKind::Print,
                                },
                            });
                        }
                    }
                }

                // Built-in comparison operators
                "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                    let method = match method_name.as_str() {
                        "==" => ComparisonMethod::EQ,
                        "!=" => ComparisonMethod::NEQ,
                        "<" => ComparisonMethod::LT,
                        "<=" => ComparisonMethod::LTE,
                        ">" => ComparisonMethod::GT,
                        ">=" => ComparisonMethod::GTE,
                        _ => unreachable!(),
                    };

                    if let Some(Pattern::Pair(pair)) = signature {
                        let left_expr =
                            if let Pattern::Value(ValuePattern { expression }) = *pair.left {
                                *expression
                            } else {
                                unreachable!()
                            };

                        let right_expr =
                            if let Pattern::Value(ValuePattern { expression }) = *pair.right {
                                *expression
                            } else {
                                unreachable!()
                            };

                        let left_register = compiler.registers.allocate_register();
                        instructions.append(
                            &mut compiler
                                .compile_expression(left_expr, Some(left_register.clone()))?,
                        );

                        let right_register = compiler.registers.allocate_register();
                        instructions.append(
                            &mut compiler
                                .compile_expression(right_expr, Some(right_register.clone()))?,
                        );

                        let destination_register = target_register
                            .unwrap_or_else(|| compiler.registers.allocate_register());
                        instructions.push(Instruction::Compare {
                            method,
                            operand1: left_register,
                            operand2: right_register,
                            destination: destination_register.clone(),
                        });

                        if compiler.context.recursion_depth == 1 && compiler.context.repl_mode {
                            instructions.push(Instruction::Interrupt {
                                interrupt: Interrupt {
                                    address: destination_register,
                                    kind: InterruptKind::Print,
                                },
                            });
                        }
                    }
                }

                // Any other method calls (user-defined multimethods)
                _ => {
                    // Verify the multimethod exists
                    if !compiler.multimethods.contains_key(&method_name) {
                        return Err(CompilerError::MethodNotFound(method_name.clone()));
                    }

                    // Compile the argument expression into the 'arg' register
                    // The argument is what will be matched against patterns at runtime
                    clear_call_argument_registers(compiler, &method_name, &mut instructions);
                    if let Some(call_sig) = signature {
                        match call_sig {
                            Pattern::Value(ValuePattern { expression }) => {
                                instructions.append(
                                    &mut compiler
                                        .compile_expression(*expression, Some("arg".to_string()))?,
                                );
                            }
                            Pattern::Record(_) | Pattern::Pair(_) => {
                                compile_record_call_arg(compiler, &call_sig, &mut instructions)?;
                            }
                            _ => {
                                return Err(CompilerError::Generic(
                                    "Only value and record patterns supported in calls currently".to_string(),
                                ));
                            }
                        }
                    }

                    instructions.push(Instruction::CallShim {
                        method_name: method_name.clone(),
                    });

                    // Copy the return value to the target register
                    let destination_register =
                        target_register.unwrap_or_else(|| compiler.registers.allocate_register());

                    instructions.push(Instruction::Copy {
                        source: "ret".to_string(),
                        destination: destination_register.clone(),
                    });

                    // Print result at top level
                    if compiler.context.recursion_depth == 1 && compiler.context.repl_mode {
                        instructions.push(Instruction::Interrupt {
                            interrupt: Interrupt {
                                address: destination_register,
                                kind: InterruptKind::Print,
                            },
                        });
                    }
                }
            }
        }

        Ok(instructions)
    }
}

impl CallCompilelet {
    fn compile_print_argument(
        &self,
        compiler: &mut Compiler,
        pattern: Pattern,
        target_register: String,
    ) -> CompilerResult<Vec<Instruction>> {
        match pattern {
            Pattern::Value(ValuePattern { expression }) => {
                compiler.compile_expression(*expression, Some(target_register))
            }
            Pattern::Variable(variable) => compiler.compile_expression(
                Expression {
                    kind: ExpressionKind::Pattern(Pattern::Variable(variable)),
                    start_pos: 0,
                    end_pos: 0,
                },
                Some(target_register),
            ),
            _ => Err(CompilerError::Generic(
                "print only supports value and variable arguments".to_string(),
            )),
        }
    }
}

fn clear_call_argument_registers(
    compiler: &Compiler,
    method_name: &str,
    instructions: &mut Vec<Instruction>,
) {
    instructions.push(Instruction::Load {
        value: RegisterValue::Empty,
        register: "arg".to_string(),
    });

    let mut registers = HashSet::new();
    if let Some(multimethod) = compiler.get_multimethod(method_name) {
        for method in &multimethod.methods {
            if let Some(signature) = &method.signature {
                collect_record_call_registers(signature, "arg", &mut registers);
            }
        }
    }

    for register in registers {
        instructions.push(Instruction::Load {
            value: RegisterValue::Empty,
            register: register.clone(),
        });
        instructions.push(Instruction::Load {
            value: RegisterValue::Boolean(false),
            register: Compiler::presence_register(&register),
        });
    }
}

fn collect_record_call_registers(pattern: &Pattern, prefix: &str, out: &mut HashSet<String>) {
    match pattern {
        Pattern::Record(record) => {
            let field_register = format!("{}.{}", prefix, record.name);
            out.insert(field_register.clone());
            if let Pattern::Tuple(tuple) = record.value.as_ref() {
                for (index, _) in crate::compiler::compilelets::flatten_pair(&tuple.child)
                    .iter()
                    .enumerate()
                {
                    out.insert(format!("{}.{}", field_register, index));
                }
            }
        }
        Pattern::Pair(pair) => {
            collect_record_call_registers(&pair.left, prefix, out);
            collect_record_call_registers(&pair.right, prefix, out);
        }
        _ => {}
    }
}

fn compile_record_call_arg(
    compiler: &mut Compiler,
    pattern: &Pattern,
    instructions: &mut Vec<Instruction>,
) -> CompilerResult<()> {
    match pattern {
        Pattern::Record(record) => {
            compile_record_call_field(
                compiler,
                &format!("arg.{}", record.name),
                &record.value,
                instructions,
            )?;
        }
        Pattern::Pair(pair) => {
            compile_record_call_arg(compiler, &pair.left, instructions)?;
            compile_record_call_arg(compiler, &pair.right, instructions)?;
        }
        _ => {}
    }

    Ok(())
}

fn compile_record_call_field(
    compiler: &mut Compiler,
    register: &str,
    pattern: &Pattern,
    instructions: &mut Vec<Instruction>,
) -> CompilerResult<()> {
    mark_present(register, instructions);

    match pattern {
        Pattern::Value(value) => {
            instructions.append(
                &mut compiler.compile_expression(*value.expression.clone(), Some(register.to_string()))?,
            );
        }
        Pattern::Variable(variable) => {
            instructions.append(&mut compiler.compile_expression(
                Expression {
                    kind: ExpressionKind::Pattern(Pattern::Variable(variable.clone())),
                    start_pos: 0,
                    end_pos: 0,
                },
                Some(register.to_string()),
            )?);
        }
        Pattern::Tuple(tuple) => {
            for (index, element) in crate::compiler::compilelets::flatten_pair(&tuple.child)
                .iter()
                .enumerate()
            {
                compile_record_call_field(
                    compiler,
                    &format!("{}.{}", register, index),
                    element,
                    instructions,
                )?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn mark_present(register: &str, instructions: &mut Vec<Instruction>) {
    instructions.push(Instruction::Load {
        value: RegisterValue::Boolean(true),
        register: Compiler::presence_register(register),
    });
}
