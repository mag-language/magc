use strontium::machine::instruction::{Instruction, CalculationMethod, Interrupt, InterruptKind};
use crate::types::{CompilerResult, Expression, ExpressionKind, Pattern, ValuePattern};
use crate::compiler::{Compiler, Compilelet};

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
            let signature = call.signature.unwrap();

            let method = match method_name.as_str() {
                "+" => CalculationMethod::ADD,
                "-" => CalculationMethod::SUBTRACT,
                "*" => CalculationMethod::MULTIPLY,
                "/" => CalculationMethod::DIVIDE,
                "^" => CalculationMethod::POWER,
                "%" => CalculationMethod::MODULO,
                _ => unreachable!(),
            };

            if let Pattern::Pair(pair) = signature {
                let left_expr = if let Pattern::Value(ValuePattern { expression }) = *pair.left {
                    *expression
                } else {
                    unreachable!()
                };

                let right_expr = if let Pattern::Value(ValuePattern { expression }) = *pair.right {
                    *expression
                } else {
                    unreachable!()
                };

                let left_register = compiler.registers.allocate_register();
                instructions.append(&mut compiler.compile_expression(left_expr, Some(left_register.clone()))?);

                let right_register = compiler.registers.allocate_register();
                instructions.append(&mut compiler.compile_expression(right_expr, Some(right_register.clone()))?);

                let destination_register = target_register.unwrap_or_else(|| compiler.registers.allocate_register());
                instructions.push(Instruction::CALCULATE {
                    method,
                    operand1: left_register,
                    operand2: right_register,
                    destination: destination_register.clone(),
                });

                instructions.push(Instruction::INTERRUPT {
                    interrupt: Interrupt {
                        address: destination_register,
                        kind: InterruptKind::Print,
                    },
                });
            }
        }

        Ok(instructions)
    }
}
