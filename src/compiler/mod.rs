use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::type_system::Typed;
use crate::types::{CompilerResult, Expression, ExpressionKind, Literal, Pattern, VariablePattern};
use crate::CompilerError;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::dispatch::DispatchPattern;
use strontium::machine::instruction::{ComparisonMethod, Instruction};
use strontium::machine::register::{RegisterType, RegisterValue, Registers};

pub type Environment<T> = HashMap<String, T>;

/// Represents a compiled method body ready for linking.
#[derive(Debug, Clone)]
pub struct CompiledMethod {
    /// Unique identifier for this method variant (name + signature hash).
    pub id: String,
    /// The name of the multimethod this belongs to.
    pub method_name: String,
    /// The dispatch pattern for shim generation.
    pub pattern: DispatchPattern,
    /// The compiled bytecode for this method body.
    pub instructions: Vec<Instruction>,
    /// Names of pattern variables that need to be bound at call time.
    pub parameter_names: Vec<String>,
}

mod compilelets;
mod errors;
mod multimethod;
mod type_system;

pub use self::errors::ErrorReporter;
pub use self::multimethod::Multimethod;
pub use self::type_system::TypeSystem;
pub use compilelets::{
    CallCompilelet, Compilelet, ConditionalCompilelet, LiteralCompilelet, MatchCompilelet,
    MethodCompilelet, ReturnCompilelet, ValuePatternCompilelet, VarCompilelet,
    VariablePatternCompilelet,
};

pub struct CompilationContext {
    pub recursion_depth: usize,
    /// Names of pattern variables in the current method scope.
    /// Used to compile variable references as LoadLocal.
    pub local_variables: HashSet<String>,
    /// Names of top-level variables bound with `var`.
    /// Stored in named registers (persist across REPL iterations).
    pub global_variables: HashSet<String>,
    /// Match arm bindings: variable name → register name.
    /// Checked before local/global; scoped to the current arm body.
    pub register_bindings: HashMap<String, String>,
    /// Counter for generating unique label IDs for conditional branches.
    pub next_label_id: usize,
    /// True when running inside the REPL; enables auto-print of top-level expressions.
    pub repl_mode: bool,
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
    multimethods: Environment<Multimethod>,
    /// Stores compiled method bodies indexed by their unique ID.
    pub compiled_methods: HashMap<String, CompiledMethod>,
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
        compilelets.insert("Nothing".to_string(), &LiteralCompilelet as &dyn Compilelet);
        compilelets.insert(
            "ValuePattern".to_string(),
            &ValuePatternCompilelet as &dyn Compilelet,
        );
        compilelets.insert(
            "VariablePattern".to_string(),
            &VariablePatternCompilelet as &dyn Compilelet,
        );
        compilelets.insert(
            "ConditionalExpression".to_string(),
            &ConditionalCompilelet as &dyn Compilelet,
        );
        compilelets.insert(
            "VarExpression".to_string(),
            &VarCompilelet as &dyn Compilelet,
        );
        compilelets.insert(
            "ReturnExpression".to_string(),
            &ReturnCompilelet as &dyn Compilelet,
        );
        compilelets.insert(
            "MatchExpression".to_string(),
            &MatchCompilelet as &dyn Compilelet,
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
                global_variables: HashSet::new(),
                register_bindings: HashMap::new(),
                next_label_id: 0,
                repl_mode: false,
            },
            multimethods: HashMap::new(),
            compiled_methods: HashMap::new(),
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
            Pattern::Record(record) => Self::extract_variable_names(&record.value),
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

    /// Convert a Pattern to a DispatchPattern for shim generation.
    pub fn pattern_to_dispatch_pattern(
        pattern: &Option<Pattern>,
        parser: &Parser,
    ) -> DispatchPattern {
        match pattern {
            None => DispatchPattern::Any,
            Some(Pattern::Variable(VariablePattern { type_id: Some(t), .. })) => {
                match t.as_str() {
                    "Int" => DispatchPattern::Type(RegisterType::Int64),
                    "Float" => DispatchPattern::Type(RegisterType::Float64),
                    "String" => DispatchPattern::Type(RegisterType::String),
                    "Bool" => DispatchPattern::Type(RegisterType::Boolean),
                    _ => DispatchPattern::Any,
                }
            }
            Some(Pattern::Variable(_)) => DispatchPattern::Any,
            Some(Pattern::Value(value_pattern)) => {
                match &value_pattern.expression.kind {
                    ExpressionKind::Literal(Literal::Int) => {
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
        self.context.recursion_depth += 1;

        let mut bytecode = vec![];
        let expression_type = expression.get_type().unwrap();

        if let Some(compilelet) = self.compilelets.get(&expression_type) {
            let mut compiled = compilelet.compile(self, expression, target_register)?;
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
        self.link_bytecode(main_bytecode)
    }

    /// Build the combined shim + body block for one method name.
    ///
    /// Returns a flat Vec<Instruction> containing:
    ///   [shim dispatch logic] [HALT if no match]
    ///   [LabelTarget body_0] [body_0 instructions]
    ///   [LabelTarget body_1] [body_1 instructions]
    ///   ...
    ///
    /// All label IDs are globally unique (via alloc_label). The caller resolves
    /// labels with the block's base offset so cross-shim-to-body jumps work.
    fn build_method_block(&mut self, variant_ids: &[String]) -> Vec<Instruction> {
        let mut instructions = vec![];
        let mut body_labels: Vec<usize> = vec![];

        for id in variant_ids {
            let body_label = self.alloc_label();
            body_labels.push(body_label);
            let pattern = self.compiled_methods[id].pattern.clone();

            match pattern {
                DispatchPattern::Any => {
                    instructions.push(Instruction::JumpToLabel { id: body_label });
                }
                DispatchPattern::Value(val) => {
                    let skip_label = self.alloc_label();
                    let val_reg = self.registers.allocate_register();
                    let cmp_reg = self.registers.allocate_register();
                    instructions.push(Instruction::Load {
                        value: val,
                        register: val_reg.clone(),
                    });
                    instructions.push(Instruction::Compare {
                        method: ComparisonMethod::EQ,
                        operand1: "arg".to_string(),
                        operand2: val_reg,
                        destination: cmp_reg.clone(),
                    });
                    instructions.push(Instruction::JumpCToLabel {
                        id: skip_label,
                        conditional_address: cmp_reg,
                    });
                    instructions.push(Instruction::JumpToLabel { id: body_label });
                    instructions.push(Instruction::LabelTarget { id: skip_label });
                }
                DispatchPattern::Type(rt) => {
                    let skip_label = self.alloc_label();
                    let type_reg = self.registers.allocate_register();
                    let expected_reg = self.registers.allocate_register();
                    let cmp_reg = self.registers.allocate_register();
                    instructions.push(Instruction::LoadType {
                        source: "arg".to_string(),
                        destination: type_reg.clone(),
                    });
                    instructions.push(Instruction::Load {
                        value: RegisterValue::Int64(rt as i64),
                        register: expected_reg.clone(),
                    });
                    instructions.push(Instruction::Compare {
                        method: ComparisonMethod::EQ,
                        operand1: type_reg,
                        operand2: expected_reg,
                        destination: cmp_reg.clone(),
                    });
                    instructions.push(Instruction::JumpCToLabel {
                        id: skip_label,
                        conditional_address: cmp_reg,
                    });
                    instructions.push(Instruction::JumpToLabel { id: body_label });
                    instructions.push(Instruction::LabelTarget { id: skip_label });
                }
            }
        }

        // No variant matched
        instructions.push(Instruction::Halt);

        // Append each body preceded by its label target
        for (i, id) in variant_ids.iter().enumerate() {
            instructions.push(Instruction::LabelTarget { id: body_labels[i] });
            let body = self.compiled_methods[id].instructions.clone();
            instructions.extend(body);
        }

        instructions
    }

    /// Replace CallShim { method_name } with Call { address } using the shim address table.
    fn patch_call_shims(
        instructions: Vec<Instruction>,
        shim_addresses: &HashMap<String, usize>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut result = Vec::with_capacity(instructions.len());
        for instr in instructions {
            match instr {
                Instruction::CallShim { method_name } => {
                    if let Some(&addr) = shim_addresses.get(&method_name) {
                        result.push(Instruction::Call { address: addr });
                    } else {
                        return Err(CompilerError::MethodNotFound(method_name));
                    }
                }
                other => result.push(other),
            }
        }
        Ok(result)
    }

    /// Link bytecode: generate dispatch shims, resolve labels, patch CallShim references.
    ///
    /// Final layout:
    ///   [JUMP to main_start]
    ///   [method_A shim + bodies]
    ///   [method_B shim + bodies]
    ///   ...
    ///   [main bytecode]
    ///   [HALT]
    fn link_bytecode(
        &mut self,
        main_bytecode: Vec<Instruction>,
    ) -> CompilerResult<Vec<Instruction>> {
        if self.compiled_methods.is_empty() {
            return Ok(self.resolve_labels(main_bytecode, 0));
        }

        // Group method IDs by name, sort each group highest precedence first.
        let mut by_name: HashMap<String, Vec<String>> = HashMap::new();
        for (id, method) in &self.compiled_methods {
            by_name.entry(method.method_name.clone()).or_default().push(id.clone());
        }
        for ids in by_name.values_mut() {
            ids.sort_by(|a, b| {
                let pa = self.compiled_methods[a].pattern.precedence();
                let pb = self.compiled_methods[b].pattern.precedence();
                pb.cmp(&pa)
            });
        }

        // Stable ordering of method names for deterministic output.
        let mut method_names: Vec<String> = by_name.keys().cloned().collect();
        method_names.sort();

        // Pass 1: calculate shim addresses by walking block sizes.
        let jump_size = self.instruction_size(&Instruction::Jump { destination: 0 });
        let mut current_offset = jump_size;
        let mut block_infos: Vec<(String, Vec<Instruction>, usize)> = vec![];
        let mut shim_addresses: HashMap<String, usize> = HashMap::new();

        for name in &method_names {
            let ids = by_name[name].clone();
            let combined = self.build_method_block(&ids);
            let base_offset = current_offset;
            shim_addresses.insert(name.clone(), base_offset);
            for instr in &combined {
                current_offset += self.instruction_size(instr);
            }
            block_infos.push((name.clone(), combined, base_offset));
        }

        let main_start = current_offset;

        // Pass 2: resolve labels and patch CallShim in each block.
        let mut linked = vec![Instruction::Jump { destination: main_start as u32 }];
        for (_, combined, base_offset) in block_infos {
            let resolved = self.resolve_labels(combined, base_offset);
            let patched = Self::patch_call_shims(resolved, &shim_addresses)?;
            linked.extend(patched);
        }

        // Resolve labels then patch CallShim in main bytecode.
        let resolved_main = self.resolve_labels(main_bytecode, main_start);
        let patched_main = Self::patch_call_shims(resolved_main, &shim_addresses)?;
        linked.extend(patched_main);

        Ok(linked)
    }

    /// Calculate the byte size of an instruction when encoded.
    fn instruction_size(&self, instr: &Instruction) -> usize {
        match instr {
            Instruction::LabelTarget { .. } => 0,
            Instruction::JumpToLabel { .. } => {
                self.instruction_size(&Instruction::Jump { destination: 0 })
            }
            Instruction::JumpCToLabel { conditional_address, .. } => {
                self.instruction_size(&Instruction::JumpC {
                    destination: 0,
                    conditional_address: conditional_address.clone(),
                })
            }
            Instruction::CallShim { .. } => {
                self.instruction_size(&Instruction::Call { address: 0 })
            }
            _ => {
                let bytes: Vec<u8> = instr.clone().into();
                bytes.len()
            }
        }
    }

    pub fn get_multimethod(&self, name: &str) -> Option<&Multimethod> {
        self.multimethods.get(name)
    }

    pub fn alloc_label(&mut self) -> usize {
        let id = self.context.next_label_id;
        self.context.next_label_id += 1;
        id
    }

    /// Resolve LabelTarget / JumpToLabel / JumpCToLabel pseudo-instructions
    /// into real Jump / JumpC instructions with absolute byte addresses.
    fn resolve_labels(
        &self,
        instructions: Vec<Instruction>,
        base_offset: usize,
    ) -> Vec<Instruction> {
        let mut label_offsets: HashMap<usize, usize> = HashMap::new();
        let mut byte = 0usize;
        for instr in &instructions {
            if let Instruction::LabelTarget { id } = instr {
                label_offsets.insert(*id, byte);
            }
            byte += self.instruction_size(instr);
        }

        instructions
            .into_iter()
            .filter_map(|instr| match instr {
                Instruction::LabelTarget { .. } => None,
                Instruction::JumpToLabel { id } => Some(Instruction::Jump {
                    destination: (base_offset + label_offsets[&id]) as u32,
                }),
                Instruction::JumpCToLabel { id, conditional_address } => Some(Instruction::JumpC {
                    destination: (base_offset + label_offsets[&id]) as u32,
                    conditional_address,
                }),
                other => Some(other),
            })
            .collect()
    }
}
