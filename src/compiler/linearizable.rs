//! Pattern linearization: turning patterns into bytecode that tests a value.
//!
//! A multimethod is linearized by ordering its variants by precedence and testing each
//! variant's signature in that order (see `Multimethod::linearize`); a `match` tests its
//! arms in source order. Both rely on [`Linearizable`], implemented for every pattern type.
//!
//! Values are stored in registers below a root: scalars in the root itself, record fields
//! in `root.<name>` and tuple elements in `root.<index>`, at any depth. `<path>.__present`
//! marks a field or element as present, and `<path>.__structure` marks a record or tuple.

use std::collections::BTreeSet;

use super::compilelets::flatten_pair;
use super::Compiler;
use crate::parser::Parser;
use crate::types::{
    CompilerError, CompilerResult, Expression, ExpressionKind, Literal, PairPattern, Pattern,
    RecordPattern, TokenKind, TuplePattern, ValuePattern, VariablePattern,
};
use strontium::machine::instruction::{ComparisonMethod, Instruction};
use strontium::machine::register::{RegisterType, RegisterValue};

/// A pattern that can be tested against a value stored below a register root.
pub trait Linearizable {
    /// How specific this pattern is. Higher wins: values 3, typed variables 2,
    /// untyped variables 1, record fields 10 plus their value; tuples and pairs add up.
    fn get_precedence(&self) -> usize;

    /// Canonical form of this pattern that ignores variable names, e.g. `n Int` and
    /// `m Int` are both `Int`. Used to detect duplicate method signatures. Fails for
    /// expression patterns like `1 + 2`, since value patterns in signatures must be literals.
    fn signature(&self, parser: &Parser) -> CompilerResult<String>;

    /// Emit bytecode that jumps to `on_fail` unless the value stored at `root` matches.
    fn linearize(
        &self,
        root: &str,
        on_fail: usize,
        compiler: &mut Compiler,
    ) -> CompilerResult<Vec<Instruction>>;

    /// Variables bound by this pattern, paired with the register holding each value.
    fn bindings(&self, root: &str) -> Vec<(String, String)>;

    /// Registers read by [`Linearizable::linearize`]; reset them before storing a value.
    fn registers(&self, root: &str) -> Vec<String>;
}

impl Linearizable for Pattern {
    fn get_precedence(&self) -> usize {
        match self {
            Pattern::Record(pattern) => pattern.get_precedence(),
            Pattern::Tuple(pattern) => pattern.get_precedence(),
            Pattern::Value(pattern) => pattern.get_precedence(),
            Pattern::Variable(pattern) => pattern.get_precedence(),
            Pattern::Pair(pattern) => pattern.get_precedence(),
        }
    }

    fn signature(&self, parser: &Parser) -> CompilerResult<String> {
        match self {
            Pattern::Record(pattern) => pattern.signature(parser),
            Pattern::Tuple(pattern) => pattern.signature(parser),
            Pattern::Value(pattern) => pattern.signature(parser),
            Pattern::Variable(pattern) => pattern.signature(parser),
            Pattern::Pair(pattern) => pattern.signature(parser),
        }
    }

    fn linearize(
        &self,
        root: &str,
        on_fail: usize,
        compiler: &mut Compiler,
    ) -> CompilerResult<Vec<Instruction>> {
        match self {
            Pattern::Record(pattern) => pattern.linearize(root, on_fail, compiler),
            Pattern::Tuple(pattern) => pattern.linearize(root, on_fail, compiler),
            Pattern::Value(pattern) => pattern.linearize(root, on_fail, compiler),
            Pattern::Variable(pattern) => pattern.linearize(root, on_fail, compiler),
            Pattern::Pair(pattern) => pattern.linearize(root, on_fail, compiler),
        }
    }

    fn bindings(&self, root: &str) -> Vec<(String, String)> {
        match self {
            Pattern::Record(pattern) => pattern.bindings(root),
            Pattern::Tuple(pattern) => pattern.bindings(root),
            Pattern::Value(pattern) => pattern.bindings(root),
            Pattern::Variable(pattern) => pattern.bindings(root),
            Pattern::Pair(pattern) => pattern.bindings(root),
        }
    }

    fn registers(&self, root: &str) -> Vec<String> {
        match self {
            Pattern::Record(pattern) => pattern.registers(root),
            Pattern::Tuple(pattern) => pattern.registers(root),
            Pattern::Value(pattern) => pattern.registers(root),
            Pattern::Variable(pattern) => pattern.registers(root),
            Pattern::Pair(pattern) => pattern.registers(root),
        }
    }
}

impl Linearizable for ValuePattern {
    fn get_precedence(&self) -> usize {
        3
    }

    fn signature(&self, parser: &Parser) -> CompilerResult<String> {
        literal_text(&self.expression, parser)
    }

    fn linearize(
        &self,
        root: &str,
        on_fail: usize,
        compiler: &mut Compiler,
    ) -> CompilerResult<Vec<Instruction>> {
        if is_nothing(&self.expression) {
            // A record or tuple root holds `nothing` too, so rule those out first.
            let mut instructions = expect_value(
                &structure(root),
                RegisterValue::Boolean(false),
                on_fail,
                compiler,
            );
            instructions.extend(expect_value(root, RegisterValue::Empty, on_fail, compiler));
            return Ok(instructions);
        }

        let value_register = compiler.registers.allocate_register();
        let mut instructions = compiler
            .compile_expression((*self.expression).clone(), Some(value_register.clone()))?;
        let condition = compiler.registers.allocate_register();
        instructions.push(Instruction::Compare {
            method: ComparisonMethod::EQ,
            operand1: root.to_string(),
            operand2: value_register,
            destination: condition.clone(),
        });
        instructions.push(jump_unless(condition, on_fail));
        Ok(instructions)
    }

    fn bindings(&self, _root: &str) -> Vec<(String, String)> {
        vec![]
    }

    fn registers(&self, root: &str) -> Vec<String> {
        let mut registers = vec![root.to_string()];
        if is_nothing(&self.expression) {
            registers.push(structure(root));
        }
        registers
    }
}

impl Linearizable for VariablePattern {
    fn get_precedence(&self) -> usize {
        match self.checked_type() {
            Some(_) => 2,
            None => 1,
        }
    }

    fn signature(&self, _parser: &Parser) -> CompilerResult<String> {
        Ok(self.type_id.clone().unwrap_or_else(|| "_".to_string()))
    }

    fn linearize(
        &self,
        root: &str,
        on_fail: usize,
        compiler: &mut Compiler,
    ) -> CompilerResult<Vec<Instruction>> {
        Ok(match self.checked_type() {
            Some(register_type) => expect_type(root, register_type, on_fail, compiler),
            None => vec![],
        })
    }

    fn bindings(&self, root: &str) -> Vec<(String, String)> {
        match &self.name {
            Some(name) => vec![(name.clone(), root.to_string())],
            None => vec![],
        }
    }

    fn registers(&self, root: &str) -> Vec<String> {
        match self.checked_type() {
            Some(RegisterType::Empty) => vec![root.to_string(), structure(root)],
            Some(_) => vec![root.to_string()],
            None => vec![],
        }
    }
}

impl VariablePattern {
    /// The runtime type this variable's annotation checks, if Mag knows the type name.
    fn checked_type(&self) -> Option<RegisterType> {
        match self.type_id.as_deref()? {
            "Int" => Some(RegisterType::Int64),
            "Float" => Some(RegisterType::Float64),
            "String" => Some(RegisterType::String),
            "Bool" | "Boolean" => Some(RegisterType::Boolean),
            "Nothing" => Some(RegisterType::Empty),
            _ => None,
        }
    }
}

impl Linearizable for RecordPattern {
    fn get_precedence(&self) -> usize {
        10 + self.value.get_precedence()
    }

    fn signature(&self, parser: &Parser) -> CompilerResult<String> {
        Ok(format!("{}: {}", self.name, self.value.signature(parser)?))
    }

    fn linearize(
        &self,
        root: &str,
        on_fail: usize,
        compiler: &mut Compiler,
    ) -> CompilerResult<Vec<Instruction>> {
        let path = member(root, &self.name);
        let mut instructions =
            expect_value(&presence(&path), RegisterValue::Boolean(true), on_fail, compiler);
        instructions.extend(self.value.linearize(&path, on_fail, compiler)?);
        Ok(instructions)
    }

    fn bindings(&self, root: &str) -> Vec<(String, String)> {
        self.value.bindings(&member(root, &self.name))
    }

    fn registers(&self, root: &str) -> Vec<String> {
        let path = member(root, &self.name);
        let mut registers = vec![presence(&path)];
        registers.extend(self.value.registers(&path));
        registers
    }
}

impl Linearizable for TuplePattern {
    fn get_precedence(&self) -> usize {
        flatten_pair(&self.child)
            .iter()
            .map(|element| element.get_precedence())
            .sum()
    }

    fn signature(&self, parser: &Parser) -> CompilerResult<String> {
        let elements = flatten_pair(&self.child)
            .iter()
            .map(|element| element.signature(parser))
            .collect::<CompilerResult<Vec<_>>>()?;
        Ok(format!("({})", elements.join(", ")))
    }

    fn linearize(
        &self,
        root: &str,
        on_fail: usize,
        compiler: &mut Compiler,
    ) -> CompilerResult<Vec<Instruction>> {
        let elements = flatten_pair(&self.child);
        let record_group = check_record_group(&elements)?;
        let mut instructions = vec![];

        for (index, element) in elements.iter().enumerate() {
            if record_group {
                instructions.extend(element.linearize(root, on_fail, compiler)?);
            } else {
                let path = member(root, &index.to_string());
                instructions.extend(expect_value(
                    &presence(&path),
                    RegisterValue::Boolean(true),
                    on_fail,
                    compiler,
                ));
                instructions.extend(element.linearize(&path, on_fail, compiler)?);
            }
        }

        Ok(instructions)
    }

    fn bindings(&self, root: &str) -> Vec<(String, String)> {
        let elements = flatten_pair(&self.child);
        let record_group = is_record_group(&elements);
        elements
            .iter()
            .enumerate()
            .flat_map(|(index, element)| {
                if record_group {
                    element.bindings(root)
                } else {
                    element.bindings(&member(root, &index.to_string()))
                }
            })
            .collect()
    }

    fn registers(&self, root: &str) -> Vec<String> {
        let elements = flatten_pair(&self.child);
        let record_group = is_record_group(&elements);
        let mut registers = vec![];

        for (index, element) in elements.iter().enumerate() {
            if record_group {
                registers.extend(element.registers(root));
            } else {
                let path = member(root, &index.to_string());
                registers.push(presence(&path));
                registers.extend(element.registers(&path));
            }
        }

        registers
    }
}

impl Linearizable for PairPattern {
    fn get_precedence(&self) -> usize {
        self.left.get_precedence() + self.right.get_precedence()
    }

    fn signature(&self, parser: &Parser) -> CompilerResult<String> {
        Ok(format!(
            "{}, {}",
            self.left.signature(parser)?,
            self.right.signature(parser)?
        ))
    }

    fn linearize(
        &self,
        root: &str,
        on_fail: usize,
        compiler: &mut Compiler,
    ) -> CompilerResult<Vec<Instruction>> {
        let mut instructions = self.left.linearize(root, on_fail, compiler)?;
        instructions.extend(self.right.linearize(root, on_fail, compiler)?);
        Ok(instructions)
    }

    fn bindings(&self, root: &str) -> Vec<(String, String)> {
        let mut bindings = self.left.bindings(root);
        bindings.extend(self.right.bindings(root));
        bindings
    }

    fn registers(&self, root: &str) -> Vec<String> {
        let mut registers = self.left.registers(root);
        registers.extend(self.right.registers(root));
        registers
    }
}

/// Emit bytecode that resets `registers`: flags to `false`, values to `nothing`.
pub fn emit_reset(registers: &[String]) -> Vec<Instruction> {
    let unique: BTreeSet<&String> = registers.iter().collect();
    unique
        .into_iter()
        .map(|register| Instruction::Load {
            value: if register.ends_with(".__present") || register.ends_with(".__structure") {
                RegisterValue::Boolean(false)
            } else {
                RegisterValue::Empty
            },
            register: register.clone(),
        })
        .collect()
}

/// Emit bytecode that stores the value of `expression` below `root`, after resetting `reset`.
///
/// Records and tuples are spread into member registers together with their presence and
/// structure flags. Every leaf value is computed into its own temporary register first and
/// only then copied into place, so calls inside the value cannot overwrite members that were
/// already written.
pub fn store_value(
    expression: &Expression,
    root: &str,
    reset: &[String],
    compiler: &mut Compiler,
) -> CompilerResult<Vec<Instruction>> {
    let mut layout = Layout::default();
    layout.expression(expression, root)?;

    let mut instructions = vec![];
    let mut copies = vec![];
    for (path, leaf) in layout.leaves {
        let temporary = compiler.registers.allocate_register();
        instructions.extend(compiler.compile_expression(leaf, Some(temporary.clone()))?);
        copies.push(Instruction::Copy {
            source: temporary,
            destination: path,
        });
    }

    instructions.extend(emit_reset(reset));
    for flag in layout.flags {
        instructions.push(Instruction::Load {
            value: RegisterValue::Boolean(true),
            register: flag,
        });
    }
    instructions.extend(copies);

    Ok(instructions)
}

/// The leaf values and flags of a value spread out below a root.
#[derive(Default)]
struct Layout {
    leaves: Vec<(String, Expression)>,
    flags: Vec<String>,
}

impl Layout {
    fn expression(&mut self, expression: &Expression, path: &str) -> CompilerResult<()> {
        match &expression.kind {
            ExpressionKind::Pattern(pattern) => self.pattern(pattern, path),
            _ => {
                self.leaves.push((path.to_string(), expression.clone()));
                Ok(())
            }
        }
    }

    fn pattern(&mut self, pattern: &Pattern, path: &str) -> CompilerResult<()> {
        match pattern {
            Pattern::Value(value) => self.expression(&value.expression, path),
            Pattern::Variable(_) => {
                // A variable in a value is a reference to the variable's value.
                self.leaves.push((
                    path.to_string(),
                    Expression {
                        kind: ExpressionKind::Pattern(pattern.clone()),
                        start_pos: 0,
                        end_pos: 0,
                    },
                ));
                Ok(())
            }
            Pattern::Record(record) => {
                let member_path = member(path, &record.name);
                self.flags.push(structure(path));
                self.flags.push(presence(&member_path));
                self.pattern(&record.value, &member_path)
            }
            Pattern::Tuple(tuple) => {
                let elements = flatten_pair(&tuple.child);
                let record_group = check_record_group(&elements)?;
                self.flags.push(structure(path));

                for (index, element) in elements.iter().enumerate() {
                    if record_group {
                        self.pattern(element, path)?;
                    } else {
                        let member_path = member(path, &index.to_string());
                        self.flags.push(presence(&member_path));
                        self.pattern(element, &member_path)?;
                    }
                }

                Ok(())
            }
            Pattern::Pair(pair) => {
                self.pattern(&pair.left, path)?;
                self.pattern(&pair.right, path)
            }
        }
    }
}

/// The register of a record field or tuple element below `path`.
fn member(path: &str, name: &str) -> String {
    format!("{}.{}", path, name)
}

/// The flag marking a record field or tuple element at `path` as present.
fn presence(path: &str) -> String {
    format!("{}.__present", path)
}

/// The flag marking the value at `path` as a record or tuple.
fn structure(path: &str) -> String {
    format!("{}.__structure", path)
}

/// Whether parenthesized elements are record fields, like `(x: 1, y: 2)`, rather than
/// positional tuple elements. Mixing both is an error.
fn check_record_group(elements: &[&Pattern]) -> CompilerResult<bool> {
    let records = elements
        .iter()
        .filter(|element| matches!(element, Pattern::Record(_)))
        .count();

    if records > 0 && records < elements.len() {
        Err(CompilerError::Generic(
            "cannot mix record fields and tuple elements in parentheses".to_string(),
        ))
    } else {
        Ok(records > 0)
    }
}

/// Like [`check_record_group`], treating mixed elements as a tuple.
fn is_record_group(elements: &[&Pattern]) -> bool {
    !elements.is_empty()
        && elements
            .iter()
            .all(|element| matches!(element, Pattern::Record(_)))
}

fn is_nothing(expression: &Expression) -> bool {
    matches!(expression.kind, ExpressionKind::Literal(Literal::Nothing))
}

/// The canonical text of a literal value pattern, or an error for an expression pattern.
fn literal_text(expression: &Expression, parser: &Parser) -> CompilerResult<String> {
    match &expression.kind {
        ExpressionKind::Literal(Literal::Int) => {
            let lexeme = parser.get_lexeme(expression.start_pos, expression.end_pos)?;
            Ok(lexeme.parse::<i64>().map(|n| n.to_string()).unwrap_or(lexeme))
        }
        ExpressionKind::Literal(Literal::Float) => {
            let lexeme = parser.get_lexeme(expression.start_pos, expression.end_pos)?;
            Ok(lexeme.parse::<f64>().map(|n| format!("{:?}", n)).unwrap_or(lexeme))
        }
        ExpressionKind::Literal(_) => {
            Ok(parser.get_lexeme(expression.start_pos, expression.end_pos)?)
        }
        ExpressionKind::Prefix(prefix)
            if prefix.operator.kind == TokenKind::Minus
                && matches!(
                    prefix.operand.kind,
                    ExpressionKind::Literal(Literal::Int | Literal::Float)
                ) =>
        {
            Ok(format!("-{}", literal_text(&prefix.operand, parser)?))
        }
        _ => Err(CompilerError::Generic(
            "value patterns in method signatures must be literals like `0`, `\"hi\"` or \
             `nothing`, not expression patterns"
                .to_string(),
        )),
    }
}

/// Jump to `on_fail` unless the register `condition` holds `true`.
fn jump_unless(condition: String, on_fail: usize) -> Instruction {
    Instruction::JumpCToLabel {
        id: on_fail,
        conditional_address: condition,
    }
}

/// Emit a check that `register` holds `expected`.
fn expect_value(
    register: &str,
    expected: RegisterValue,
    on_fail: usize,
    compiler: &mut Compiler,
) -> Vec<Instruction> {
    let expected_register = compiler.registers.allocate_register();
    let condition = compiler.registers.allocate_register();
    vec![
        Instruction::Load {
            value: expected,
            register: expected_register.clone(),
        },
        Instruction::Compare {
            method: ComparisonMethod::EQ,
            operand1: register.to_string(),
            operand2: expected_register,
            destination: condition.clone(),
        },
        jump_unless(condition, on_fail),
    ]
}

/// Emit a check that `register` holds a value of `register_type`.
fn expect_type(
    register: &str,
    register_type: RegisterType,
    on_fail: usize,
    compiler: &mut Compiler,
) -> Vec<Instruction> {
    let mut instructions = vec![];

    if register_type == RegisterType::Empty {
        // A record or tuple root holds `nothing` too, so rule those out first.
        instructions.extend(expect_value(
            &structure(register),
            RegisterValue::Boolean(false),
            on_fail,
            compiler,
        ));
    }

    let type_register = compiler.registers.allocate_register();
    instructions.push(Instruction::LoadType {
        source: register.to_string(),
        destination: type_register.clone(),
    });
    instructions.extend(expect_value(
        &type_register,
        RegisterValue::Int64(register_type as i64),
        on_fail,
        compiler,
    ));

    instructions
}
