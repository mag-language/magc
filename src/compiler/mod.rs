use crate::types::{Expression, CompilerResult};
use crate::type_system::Typed;
use crate::CompilerError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::HashMap;

use strontium::machine::instruction::Instruction;
use strontium::machine::register::Registers;

pub type Environment<T> = HashMap<String, T>;

mod compilelets;
mod errors;
mod type_system;
mod multimethod;

pub use self::compilelets::{
    Compilelet,
    CallCompilelet,
    InfixCompilelet,
    LiteralCompilelet,
    ValuePatternCompilelet,
};
pub use self::errors::ErrorReporter;
pub use self::multimethod::Multimethod;
pub use self::type_system::TypeSystem;

pub struct Compiler {
    /// The global namespace for variables.
    _variables:    Environment<Expression>,
    /// Keeps track of registers as they would be allocated in the Strontium machine.
    registers:    Registers,
    /// Maps expression types to pieces of code able to compile that specific expression.
    compilelets: HashMap<String, &'static dyn Compilelet>,
    lexer: Lexer,
    parser: Parser,
    /// Contains all method instances defined at runtime.
    ///
    /// The `Multimethod` type in this environment stores an arbitrary number of pairs
    /// of method signatures and bodies under a single name, provides methods to match
    /// its signatures with a given call signature and extracts any variables.
    _multimethods: Environment<Multimethod>,
    /// A structure which keeps track of defined types.
    _types:        TypeSystem,
    /// Reports errors to the user with helpful information.
    _errors:       ErrorReporter,
}

impl Compiler {
    pub fn new() -> Self {
        let mut compilelets = HashMap::new();

        compilelets.insert("InfixExpression".to_string(),&InfixCompilelet        as &dyn Compilelet);
        compilelets.insert("CallExpression".to_string(), &CallCompilelet         as &dyn Compilelet);
        compilelets.insert("Int".to_string(),            &LiteralCompilelet      as &dyn Compilelet);
        compilelets.insert("ValuePattern".to_string(),   &ValuePatternCompilelet as &dyn Compilelet);

        Self {
            _variables:    HashMap::new(),
            registers:     Registers::new(),
            compilelets,
            lexer:         Lexer::new(),
            parser:        Parser::new(),
            _multimethods: HashMap::new(),
            _types:        TypeSystem,
            _errors:       ErrorReporter,
        }
    }

    pub fn compile_expression(
        &mut self,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut bytecode = vec![];
        let expression_type = expression.get_type().unwrap();

        if let Some(compilelet) = self.compilelets.get(&expression_type) {
            let mut compiled = compilelet.compile(self, expression, target_register)?;

            bytecode.append(&mut compiled);

            Ok(bytecode)
        } else {
            Err(CompilerError::Generic(format!("No compilelet found for type {}", expression_type)))
        }
    }

    pub fn compile(&mut self, source: String) -> CompilerResult<Vec<Instruction>> {
        self.lexer.add_text(source.clone());
        self.parser.add_tokens(vec![source], self.lexer.parse());

        let expressions = self.parser.parse().unwrap();
        let mut bytecode = vec![];

        for expr in expressions {
            bytecode.append(&mut self.compile_expression(expr, None)?);
        }

        Ok(bytecode)
    }
}
