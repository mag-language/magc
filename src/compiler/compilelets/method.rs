use crate::types::{Expression, ExpressionKind, CompilerResult};
use strontium::machine::instruction::Instruction;
use crate::compiler::{Compiler, Multimethod};
use super::Compilelet;

// Implement a compilelet which defines a method within the compiler using a Block as the body.
pub struct MethodCompilelet;

impl Compilelet for MethodCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        match expression.kind.clone() {
            ExpressionKind::Method(method) => {
                let method_c = method.clone();
                let method_identifier = format!("{}__{}", &method_c.name, &method_c.signature.unwrap());
                println!("method_identifier: {}", method_identifier);
                if let Some(multimethod) = compiler.multimethods.get_mut(&method.name) {
                    multimethod.add_method(&compiler.parser, method.clone())?;
                } else {
                    let mut m = Multimethod::new(&method.name);
                    m.add_method(&compiler.parser, method.clone())?;
                    compiler.multimethods.insert(
                        method.name.clone(),
                        m,
                    );
                }

                let body = compiler.compile_expression(*method.body.clone(), target_register)?;
            },

            _ => (),
        }

        Ok(vec![])
    }
}