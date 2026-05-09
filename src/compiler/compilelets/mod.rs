use crate::types::{CompilerResult, Expression};
use strontium::machine::instruction::Instruction;

use super::Compiler;

mod call;
mod literal;
mod method;
mod value_pattern;
mod variable_pattern;

pub use self::call::*;
pub use self::literal::*;
pub use self::method::*;
pub use self::value_pattern::*;
pub use self::variable_pattern::*;

pub trait Compilelet {
    fn compile(
        &self,
        compiler: &mut Compiler,
        expression: Expression,
        target_register: Option<String>,
    ) -> CompilerResult<Vec<Instruction>>;
}
