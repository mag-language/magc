use crate::types::{Expression, CompilerResult};
use strontium::machine::instruction::Instruction;

use super::Compiler;

mod call;
mod literal;
mod method;
mod value_pattern;

pub use self::call::*;
pub use self::literal::*;
pub use self::method::*;
pub use self::value_pattern::*;

pub trait Compilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>>;
}