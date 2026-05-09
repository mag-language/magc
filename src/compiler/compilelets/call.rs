use crate::compiler::{Compilelet, Compiler};
use crate::types::{
    CompilerError, CompilerResult, Expression, ExpressionKind, Pattern, ValuePattern,
};
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

                // Built-in arithmetic operators
                "+" | "-" | "*" | "/" | "^" | "%" => {
                    let method = match method_name.as_str() {
                        "+" => CalculationMethod::ADD,
                        "-" => CalculationMethod::SUBTRACT,
                        "*" => CalculationMethod::MULTIPLY,
                        "/" => CalculationMethod::DIVIDE,
                        "^" => CalculationMethod::POWER,
                        "%" => CalculationMethod::MODULO,
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

                        if compiler.context.recursion_depth == 1 {
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

                        if compiler.context.recursion_depth == 1 {
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
                    if let Some(call_sig) = signature {
                        // Extract the value from the pattern and compile it
                        match call_sig {
                            Pattern::Value(ValuePattern { expression }) => {
                                instructions.append(
                                    &mut compiler
                                        .compile_expression(*expression, Some("arg".to_string()))?,
                                );
                            }
                            _ => {
                                // For other patterns, try to compile them directly
                                // This handles things like tuple arguments
                                return Err(CompilerError::Generic(
                                    "Only value patterns supported in calls currently".to_string(),
                                ));
                            }
                        }
                    }

                    // Generate DISPATCH instruction - runtime will match arg against patterns
                    instructions.push(Instruction::Dispatch {
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
                    if compiler.context.recursion_depth == 1 {
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
