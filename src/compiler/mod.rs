use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::type_system::Typed;
use crate::types::{CompilerResult, Expression, ExpressionKind, Literal, Pattern, VariablePattern};
use crate::CompilerError;
use std::collections::HashMap;
use std::collections::HashSet;

use strontium::machine::instruction::{DispatchPattern, Instruction};
use strontium::machine::register::{RegisterValue, Registers};

pub type Environment<T> = HashMap<String, T>;

/// Represents a compiled method body ready for linking.
#[derive(Debug, Clone)]
pub struct CompiledMethod {
    /// Unique identifier for this method variant (name + signature hash).
    pub id: String,
    /// The name of the multimethod this belongs to.
    pub method_name: String,
    /// The dispatch pattern for runtime matching.
    pub pattern: DispatchPattern,
    /// The compiled bytecode for this method body.
    pub instructions: Vec<Instruction>,
    /// Names of pattern variables that need to be bound at call time.
    pub parameter_names: Vec<String>,
}

/// Tracks a pending CALL that needs address resolution during linking.
#[derive(Debug, Clone)]
pub struct PendingCall {
    /// Index in the instruction vector where the CALL is located.
    pub instruction_index: usize,
    /// The method ID this call targets.
    pub target_method_id: String,
}

/// Information about a method for dispatch registration.
#[derive(Debug, Clone)]
pub struct MethodRegistration {
    pub method_name: String,
    pub pattern: DispatchPattern,
    pub address: usize,
}

mod compilelets;
mod errors;
mod multimethod;
mod type_system;

pub use self::errors::ErrorReporter;
pub use self::multimethod::Multimethod;
pub use self::type_system::TypeSystem;
pub use compilelets::{
    CallCompilelet, Compilelet, LiteralCompilelet, MethodCompilelet, ValuePatternCompilelet,
    VariablePatternCompilelet,
};

pub struct CompilationContext {
    pub recursion_depth: usize,
    /// Names of pattern variables in the current method scope.
    /// Used to compile variable references as LoadLocal.
    pub local_variables: HashSet<String>,
    /// Tracks the total number of instructions emitted so far.
    /// Used to calculate CALL instruction indices for linking.
    pub instruction_count: usize,
}

pub struct Compiler {
    /// The global namespace for variables.
    _variables: Environment<Expression>,
    /// Keeps track of registers as they would be allocated in the Strontium machine.
    pub registers: Registers,
    /// Maps expression types to pieces of code able to compile that specific expression.
    compilelets: HashMap<String, &'static dyn Compilelet>,
    pub lexer: Lexer,
    pub parser: Parser,
    pub context: CompilationContext,
    /// Contains all method instances defined at runtime.
    ///
    /// The `Multimethod` type in this environment stores an arbitrary number of pairs
    /// of method signatures and bodies under a single name, provides methods to match
    /// its signatures with a given call signature and extracts any variables.
    multimethods: Environment<Multimethod>,
    /// Stores compiled method bodies indexed by their unique ID.
    /// The ID is formed from the method name and a hash of its signature.
    pub compiled_methods: HashMap<String, CompiledMethod>,
    /// Tracks CALL instructions that need address resolution during linking.
    pub pending_calls: Vec<PendingCall>,
    /// Method registration info for the VM's dispatch table, populated during linking.
    pub method_registrations: Vec<MethodRegistration>,
    /// A structure which keeps track of defined types.
    _types: TypeSystem,
    /// Reports errors to the user with helpful information.
    _errors: ErrorReporter,
}

impl Compiler {
    pub fn new() -> Self {
        env_logger::init();
        let mut compilelets = HashMap::new();

        compilelets.insert(
            "CallExpression".to_string(),
            &CallCompilelet as &dyn Compilelet,
        );
        compilelets.insert(
            "MethodExpression".to_string(),
            &MethodCompilelet as &dyn Compilelet,
        );
        compilelets.insert("Float".to_string(), &LiteralCompilelet as &dyn Compilelet);
        compilelets.insert("Int".to_string(), &LiteralCompilelet as &dyn Compilelet);
        compilelets.insert("String".to_string(), &LiteralCompilelet as &dyn Compilelet);
        compilelets.insert("Boolean".to_string(), &LiteralCompilelet as &dyn Compilelet);
        compilelets.insert(
            "ValuePattern".to_string(),
            &ValuePatternCompilelet as &dyn Compilelet,
        );
        compilelets.insert(
            "VariablePattern".to_string(),
            &VariablePatternCompilelet as &dyn Compilelet,
        );

        Self {
            _variables: HashMap::new(),
            registers: Registers::new(),
            compilelets,
            lexer: Lexer::new(),
            parser: Parser::new(),
            context: CompilationContext {
                recursion_depth: 0,
                local_variables: HashSet::new(),
                instruction_count: 0,
            },
            multimethods: HashMap::new(),
            compiled_methods: HashMap::new(),
            pending_calls: vec![],
            method_registrations: vec![],
            _types: TypeSystem,
            _errors: ErrorReporter,
        }
    }

    /// Extract variable names from a pattern signature.
    pub fn extract_variable_names(pattern: &Pattern) -> Vec<String> {
        match pattern {
            Pattern::Variable(VariablePattern { name: Some(n), .. }) => vec![n.clone()],
            Pattern::Variable(VariablePattern { name: None, .. }) => vec![],
            Pattern::Pair(pair) => {
                let mut names = Self::extract_variable_names(&pair.left);
                names.extend(Self::extract_variable_names(&pair.right));
                names
            }
            Pattern::Tuple(tuple) => Self::extract_variable_names(&tuple.child),
            Pattern::Field(field) => Self::extract_variable_names(&field.value),
            Pattern::Value(_) => vec![],
        }
    }

    /// Generate a unique ID for a method variant based on its name and signature.
    pub fn generate_method_id(name: &str, signature: &Option<Pattern>) -> String {
        match signature {
            Some(pattern) => format!("{}_{:?}", name, pattern),
            None => name.to_string(),
        }
    }

    /// Convert a Pattern to a DispatchPattern for runtime matching.
    pub fn pattern_to_dispatch_pattern(
        pattern: &Option<Pattern>,
        parser: &Parser,
    ) -> DispatchPattern {
        match pattern {
            None => DispatchPattern::Any,
            Some(Pattern::Variable(_)) => DispatchPattern::Any,
            Some(Pattern::Value(value_pattern)) => {
                // Try to extract a literal value
                match &value_pattern.expression.kind {
                    ExpressionKind::Literal(Literal::Int) => {
                        // Get the actual integer value from the source
                        if let Ok(lexeme) = parser.get_lexeme(
                            value_pattern.expression.start_pos,
                            value_pattern.expression.end_pos,
                        ) {
                            if let Ok(n) = lexeme.parse::<i64>() {
                                return DispatchPattern::Value(RegisterValue::Int64(n));
                            }
                        }
                        DispatchPattern::Any
                    }
                    ExpressionKind::Literal(Literal::Float) => {
                        if let Ok(lexeme) = parser.get_lexeme(
                            value_pattern.expression.start_pos,
                            value_pattern.expression.end_pos,
                        ) {
                            if let Ok(n) = lexeme.parse::<f64>() {
                                return DispatchPattern::Value(RegisterValue::Float64(n));
                            }
                        }
                        DispatchPattern::Any
                    }
                    _ => DispatchPattern::Any,
                }
            }
            _ => DispatchPattern::Any,
        }
    }

    pub fn compile_expression(
        &mut self,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        // TODO: Add a limit to recursion depth
        self.context.recursion_depth += 1;

        let mut bytecode = vec![];
        let expression_type = expression.get_type().unwrap();

        if let Some(compilelet) = self.compilelets.get(&expression_type) {
            let mut compiled = compilelet.compile(self, expression, target_register)?;

            // Update instruction count for call tracking
            self.context.instruction_count += compiled.len();

            bytecode.append(&mut compiled);
            self.context.recursion_depth -= 1;

            Ok(bytecode)
        } else {
            self.context.recursion_depth -= 1;
            Err(CompilerError::Generic(format!(
                "No compilelet found for type {}",
                expression_type
            )))
        }
    }

    pub fn compile(&mut self, source: String) -> CompilerResult<Vec<Instruction>> {
        self.lexer.add_text(source.clone());
        let tokens = self.lexer.parse();

        self.parser.add_tokens(source, tokens);
        let expressions = self.parser.parse()?;
        let mut main_bytecode = vec![];

        for mut expr in expressions {
            expr.desugar();
            main_bytecode.append(&mut self.compile_expression(expr, None)?);
        }

        main_bytecode.push(Instruction::Halt);

        // Link the bytecode: resolve CALL addresses
        let linked = self.link_bytecode(main_bytecode)?;

        Ok(linked)
    }

    /// Link bytecode by resolving method call addresses.
    ///
    /// Layout:
    /// [JUMP to main start]
    /// [method 1 body][RETURN]
    /// [method 2 body][RETURN]
    /// ...
    /// [main bytecode][HALT]
    fn link_bytecode(
        &mut self,
        main_bytecode: Vec<Instruction>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut linked = vec![];

        // If no methods defined, just return main bytecode
        if self.compiled_methods.is_empty() {
            return Ok(main_bytecode);
        }

        // Calculate method addresses (in bytes)
        // First instruction is JUMP to skip methods
        let jump_size = self.instruction_size(&Instruction::Jump { destination: 0 });
        let mut method_addresses: HashMap<String, usize> = HashMap::new();
        let mut current_offset = jump_size;

        // Calculate byte offset for each method and build registration info
        self.method_registrations.clear();
        for (method_id, compiled_method) in &self.compiled_methods {
            method_addresses.insert(method_id.clone(), current_offset);

            // Record this method for dispatch registration
            self.method_registrations.push(MethodRegistration {
                method_name: compiled_method.method_name.clone(),
                pattern: compiled_method.pattern.clone(),
                address: current_offset,
            });

            for instr in &compiled_method.instructions {
                current_offset += self.instruction_size(instr);
            }
        }

        // Main bytecode starts after all methods
        let main_start = current_offset;

        // Build final bytecode
        // 1. Jump to main
        linked.push(Instruction::Jump {
            destination: main_start as u32,
        });

        // 2. All method bodies
        for (_, compiled_method) in &self.compiled_methods {
            linked.extend(compiled_method.instructions.clone());
        }

        // 3. Main bytecode with patched CALL addresses
        for (i, instr) in main_bytecode.into_iter().enumerate() {
            match instr {
                Instruction::Call { address: 0 } => {
                    // Find the pending call for this index
                    let call_index = i;
                    if let Some(pending) = self
                        .pending_calls
                        .iter()
                        .find(|p| p.instruction_index == call_index)
                    {
                        // Look up the method's byte address
                        if let Some(&byte_addr) = method_addresses.get(&pending.target_method_id) {
                            linked.push(Instruction::Call { address: byte_addr });
                        } else {
                            // Method not found - keep placeholder for debugging
                            linked.push(Instruction::Call { address: 0 });
                        }
                    } else {
                        // No pending call record - keep placeholder
                        linked.push(Instruction::Call { address: 0 });
                    }
                }
                _ => linked.push(instr),
            }
        }

        Ok(linked)
    }

    /// Calculate the byte size of an instruction when encoded.
    fn instruction_size(&self, instr: &Instruction) -> usize {
        // Convert to bytes and measure length
        let bytes: Vec<u8> = instr.clone().into();
        bytes.len()
    }

    pub fn get_multimethod(&self, name: &str) -> Option<&Multimethod> {
        self.multimethods.get(name)
    }

    /// Record a pending call that needs address resolution.
    pub fn add_pending_call(&mut self, instruction_index: usize, target_method_id: String) {
        self.pending_calls.push(PendingCall {
            instruction_index,
            target_method_id,
        });
    }
}
