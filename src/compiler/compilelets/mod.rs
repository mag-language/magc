use crate::types::{Expression, CompilerResult};
use strontium::machine::instruction::Instruction;

use super::Compiler;

mod call;
mod infix;
mod literal;
mod value_pattern;

pub use self::call::CallCompilelet;
pub use self::infix::InfixCompilelet;
pub use self::literal::LiteralCompilelet;
pub use self::value_pattern::ValuePatternCompilelet;

pub trait Compilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>>;
}