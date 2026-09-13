use super::Compilelet;
use crate::compiler::linearizable::Linearizable;
use crate::compiler::multimethod::{Variant, ARGUMENT_REGISTER};
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
                // Linearize the signature now, while the parser still holds this method's source.
                let variant = Variant::new(method.clone(), compiler)?;
                let method_id = variant.id.clone();

                // Parameters bound by the signature, with the argument register holding each value
                let parameter_bindings = method
                    .signature
                    .as_ref()
                    .map(|signature| signature.bindings(ARGUMENT_REGISTER))
                    .unwrap_or_default();
                let parameter_names: Vec<String> = parameter_bindings
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect();

                // Register with multimethod dispatch table
                compiler
                    .multimethods
                    .entry(method.name.clone())
                    .or_insert_with(|| Multimethod::new(&method.name))
                    .add_variant(variant)?;

                // Store a placeholder entry BEFORE compiling the body
                // This allows recursive methods to reference themselves
                compiler.compiled_methods.insert(
                    method_id.clone(),
                    CompiledMethod {
                        id: method_id.clone(),
                        method_name: method.name.clone(),
                        instructions: vec![], // Placeholder - will be filled in
                        parameter_names: parameter_names.clone(),
                    },
                );

                // Set up local variable scope for compiling the method body
                let old_locals = compiler.context.local_variables.clone();
                compiler.context.local_variables = parameter_names.iter().cloned().collect();

                // Build method preamble: copy each parameter from its argument register to a local
                let mut body_instructions = vec![];
                for (param_name, source_register) in &parameter_bindings {
                    body_instructions.push(Instruction::StoreLocal {
                        name: param_name.clone(),
                        register: source_register.clone(),
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
