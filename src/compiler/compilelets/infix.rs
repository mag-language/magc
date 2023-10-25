use strontium::machine::instruction::Instruction;
use crate::types::{
    Call,
    CompilerResult,
    Expression,
    ExpressionKind,
    Pattern,
    PairPattern,
    ValuePattern,
    TokenKind,
};
use crate::compiler::Compiler;
use super::Compilelet;

pub struct InfixCompilelet;

impl Compilelet for InfixCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        if let ExpressionKind::Infix(infix) = expression.kind {
            // Convert the infix expression to a method call
            let method_name = get_method_name(&infix.operator.kind);
            let call_expression = Expression {
                kind: ExpressionKind::Call(Call {
                    name: method_name,
                    signature: Some(Pattern::Pair(PairPattern {
                        left: Box::new(Pattern::Value(ValuePattern {
                            expression: Box::new(*infix.left),
                        })),
                        right: Box::new(Pattern::Value(ValuePattern {
                            expression: Box::new(*infix.right),
                        })),
                    })),
                }),
                start_pos: expression.start_pos,
                end_pos: expression.end_pos,
            };

            // Compile the method call
            compiler.compile_expression(call_expression, target_register)
        } else {
            unreachable!()
        }
    }
}

fn get_method_name(operator: &TokenKind) -> String {
    match operator {
        TokenKind::Plus => "+".to_string(),
        TokenKind::Minus => "-".to_string(),
        TokenKind::Star => "*".to_string(),
        TokenKind::Slash => "/".to_string(),
        // ... other operators ...
        _ => unimplemented!(),
    }
}
