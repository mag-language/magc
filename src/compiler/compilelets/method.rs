use super::Compilelet;
use crate::compiler::{CompiledMethod, Compiler, Multimethod};
use crate::types::{CompilerResult, Expression, ExpressionKind};
use strontium::machine::instruction::Instruction;

// Implement a compilelet which defines a method within the compiler using a Block as the body.
pub struct MethodCompilelet;

impl Compilelet for MethodCompilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        _target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        match expression.kind.clone() {
            ExpressionKind::Method(method) => {
                let method_id = Compiler::generate_method_id(&method.name, &method.signature);

                // Register with multimethod dispatch table
                if let Some(multimethod) = compiler.multimethods.get_mut(&method.name) {
                    multimethod.add_method(&compiler.parser, method.clone())?;
                } else {
                    let mut m = Multimethod::new(&method.name);
                    m.add_method(&compiler.parser, method.clone())?;
                    compiler.multimethods.insert(method.name.clone(), m);
                }

                // Extract parameter names from the signature
                let parameter_names = if let Some(ref sig) = method.signature {
                    Compiler::extract_variable_names(sig)
                } else {
                    vec![]
                };

                // Convert the signature to a dispatch pattern for runtime matching
                let dispatch_pattern =
                    Compiler::pattern_to_dispatch_pattern(&method.signature, &compiler.parser);

                // Store a placeholder entry BEFORE compiling the body
                // This allows recursive methods to reference themselves
                compiler.compiled_methods.insert(
                    method_id.clone(),
                    CompiledMethod {
                        id: method_id.clone(),
                        method_name: method.name.clone(),
                        pattern: dispatch_pattern,
                        instructions: vec![], // Placeholder - will be filled in
                        parameter_names: parameter_names.clone(),
                    },
                );

                // Set up local variable scope for compiling the method body
                let old_locals = compiler.context.local_variables.clone();
                compiler.context.local_variables = parameter_names.iter().cloned().collect();

                // Build method preamble: copy argument from 'arg' register to local variables
                let mut body_instructions = vec![];
                for param_name in &parameter_names {
                    body_instructions.push(Instruction::StoreLocal {
                        name: param_name.clone(),
                        register: "arg".to_string(),
                    });
                }

                // Compile the method body with result going to 'ret' register
                body_instructions.append(
                    &mut compiler
                        .compile_expression(*method.body.clone(), Some("ret".to_string()))?,
                );

                // Add RETURN instruction at end of method
                body_instructions.push(Instruction::Return);

                // Restore previous scope
                compiler.context.local_variables = old_locals;

                // Update the compiled method with actual instructions
                if let Some(compiled) = compiler.compiled_methods.get_mut(&method_id) {
                    compiled.instructions = body_instructions;
                }
            }

            _ => (),
        }

        Ok(vec![])
    }
}
