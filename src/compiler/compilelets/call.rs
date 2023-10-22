use std::string::String;

use strontium::machine::instruction::{Instruction, CalculationMethod};

use crate::types::{CompilerResult, Expression, ExpressionKind};
use crate::compiler::Compiler;

use super::Compilelet;

/// A compilelet for method calls.
pub struct CallCompilelet;

impl Compilelet for CallCompilelet {
    fn compile(
        &self,
        compiler:   &mut Compiler,
        expression: Expression,
        _target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut instructions = vec![];

        // Implement the call compilelet, compiling method calls to CALCULATE
            // instructions if a common operators like +, -, *, /, etc. are used as
            // the method name, and to CALL instructions otherwise.
        if let ExpressionKind::Call(call) = expression.kind {
            let method_name = call.name;
            let signature = call.signature.unwrap();

            // Compile the left and right expressions
            // let left_instructions = compiler.compile(*signature.left);
            // let right_instructions = compiler.compile(*signature.right);

            let instruction = match method_name.as_str() {
                "+" | "-" | "/" |  "*" | "^" | "%" => {
                    let method = match method_name.as_str() {
                        "+" => CalculationMethod::ADD,
                        "-" => CalculationMethod::SUBTRACT,
                        "*" => CalculationMethod::MULTIPLY,
                        "/" => CalculationMethod::DIVIDE,
                        "^" => CalculationMethod::POWER,
                        "%" => CalculationMethod::MODULO,
                        _ => unreachable!(),
                    };

                    let operands = signature.expect_pair().unwrap();

                    let left_register = compiler.registers.allocate_register();
                    let right_register = compiler.registers.allocate_register();
                    let destination = compiler.registers.allocate_register();

                    instructions.append(&mut compiler.compile_expression(
                        Expression {
                            kind: ExpressionKind::Pattern(*operands.left),
                            start_pos: expression.start_pos,
                            end_pos:   expression.end_pos,
                        },
                        Some(left_register.clone()),
                    )?);

                    instructions.append(&mut compiler.compile_expression(
                        Expression {
                            kind: ExpressionKind::Pattern(*operands.right),
                            start_pos: expression.start_pos,
                            end_pos:   expression.end_pos,
                        },
                        Some(right_register.clone()),
                    )?);

                    Instruction::CALCULATE {
                        method,
                        operand1:    left_register,
                        operand2:    right_register,
                        destination,
                    }
                }

                _ => {
                    // Define a CALL instruction to find an empty register and load the value into it.
                    Instruction::CALL {}
                }
            };

            instructions.push(instruction);
        }
        Ok(instructions)
    }
}