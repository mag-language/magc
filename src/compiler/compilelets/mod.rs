use strontium::Instruction;
use crate::types::{Expression, CompilerResult};

use super::Compiler;

pub trait Compilelet {
    fn parse(&self, parser: &mut Compiler, expr: Expression) -> CompilerResult<Vec<Instruction>>;
}