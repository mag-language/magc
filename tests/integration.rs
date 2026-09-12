use magc::compiler::Compiler;
use strontium::machine::register::RegisterValue;
use strontium::Strontium;

fn run(source: &str) -> Strontium {
    let mut compiler = Compiler::new();
    let instructions = compiler
        .compile(source.to_string())
        .expect("compilation failed");

    let mut machine = Strontium::new(false);
    for instr in instructions {
        machine.push_instruction(instr);
    }
    machine.execute_until_eof().expect("execution failed");
    machine
}

/// Compile each line separately on one compiler, like the REPL does, and run the
/// program linked by the last line.
fn run_lines(lines: &[&str]) -> Strontium {
    let mut compiler = Compiler::new();
    let mut instructions = vec![];
    for line in lines {
        instructions = compiler
            .compile(line.to_string())
            .unwrap_or_else(|e| panic!("compilation of `{}` failed: {}", line, e));
    }

    let mut machine = Strontium::new(false);
    for instr in instructions {
        machine.push_instruction(instr);
    }
    machine.execute_until_eof().expect("execution failed");
    machine
}

fn reg(machine: &Strontium, name: &str) -> RegisterValue {
    machine
        .registers
        .get(name)
        .unwrap_or_else(|| panic!("register '{}' not found", name))
        .clone()
}

// ---------------------------------------------------------------------------
// Prefix operators
// ---------------------------------------------------------------------------

#[test]
fn prefix_minus_int() {
    let m = run("var result = -7");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(-7));
}

#[test]
fn prefix_minus_float() {
    let m = run("var result = -7.5");
    assert_eq!(reg(&m, "result"), RegisterValue::Float64(-7.5));
}

// ---------------------------------------------------------------------------
// if / else
// ---------------------------------------------------------------------------

#[test]
fn if_oneliner_true_branch() {
    let m = run("var result = if 1 == 1 then 1 else 0");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(1));
}

#[test]
fn if_oneliner_false_branch() {
    let m = run("var result = if 1 == 2 then 1 else 0");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(0));
}

#[test]
fn if_multiline_true_branch() {
    let m = run("var result = if 3 > 2 then\n  1\nelse\n  0\nend");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(1));
}

#[test]
fn if_multiline_false_branch() {
    let m = run("var result = if 1 > 2 then\n  1\nelse\n  0\nend");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(0));
}

// ---------------------------------------------------------------------------
// match expressions
// ---------------------------------------------------------------------------

#[test]
fn match_value_pattern_hit() {
    let m = run("var result = match 7 case 7 then 1 else 0 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(1));
}

#[test]
fn match_value_pattern_miss() {
    let m = run("var result = match 3 case 7 then 1 else 0 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(0));
}

#[test]
fn match_type_pattern_int_hit() {
    let m = run("var result = match 42 case _ Int then 1 else 0 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(1));
}

#[test]
fn match_type_pattern_int_miss() {
    let m = run("var result = match 3.14 case _ Int then 1 else 0 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(0));
}

#[test]
fn match_type_pattern_float_hit() {
    let m = run("var result = match 3.14 case _ Float then 1 else 0 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(1));
}

#[test]
fn match_variable_binding() {
    let m = run("var result = match 7 case n then n else 0 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(7));
}

#[test]
fn match_variable_binding_used_in_expr() {
    let m = run("var result = match 7 case n then n + 1 else 0 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(8));
}

#[test]
fn match_multiline() {
    let source = "var result = match 5
  case 5 then 100
  else 0
end";
    let m = run(source);
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(100));
}

#[test]
fn match_without_else_hit() {
    let m = run("var result = match 7 case 7 then 1 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(1));
}

#[test]
fn match_without_else_miss_is_nothing() {
    let m = run("var result = match 3 case 7 then 1 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Empty);
}

// `boom(1)` has no matching variant, so executing the arm body fails at runtime.
const BOOM: &str = "def boom(0) 0\n";

#[test]
fn match_without_else_miss_skips_arm_body() {
    let m = run(&format!("{}var result = match 3 case 7 then boom(1) end", BOOM));
    assert_eq!(reg(&m, "result"), RegisterValue::Empty);
}

#[test]
#[should_panic(expected = "execution failed")]
fn match_without_else_hit_runs_arm_body() {
    // Control for the test above: on a match, the arm body does run.
    run(&format!("{}var result = match 7 case 7 then boom(1) end", BOOM));
}

#[test]
fn record_match_without_else_miss_is_nothing() {
    let m = run("var result = match name: \"Dan\" case name: \"Dave\" then 1 end");
    assert_eq!(reg(&m, "result"), RegisterValue::Empty);
}

// ---------------------------------------------------------------------------
// Multi-line def / multimethod dispatch
// ---------------------------------------------------------------------------

#[test]
fn def_oneliner_called() {
    let source = "def double(n Int) n + n\nvar result = double(5)";
    let m = run(source);
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(10));
}

#[test]
fn def_multiline_abs_positive() {
    let source = "def abs(n Int) if n < 0 then 0 - n else n\nvar result = abs(7)";
    let m = run(source);
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(7));
}

#[test]
fn def_multiline_abs_negative() {
    let source = "def abs(n Int) if n < 0 then 0 - n else n\nvar result = abs(-7)";
    let m = run(source);
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(7));
}

#[test]
fn multimethod_dispatch_by_value() {
    let source = "def label(n Int) 0\ndef label(0) 1\nvar result = label(0)";
    let m = run(source);
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(1));
}

#[test]
fn multimethod_value_variants_on_separate_compiles() {
    let m = run_lines(&[
        "def fib(0) 0",
        "def fib(1) 1",
        "def fib(n Int) fib(n - 1) + fib(n - 2)",
        "var result = fib(10)",
    ]);
    assert_eq!(reg(&m, "result"), RegisterValue::Int64(55));
}

#[test]
fn multimethod_duplicate_on_separate_compiles_is_rejected() {
    let mut compiler = Compiler::new();
    compiler.compile("def fib(0) 0".to_string()).unwrap();
    assert!(compiler.compile("def fib(0) 1".to_string()).is_err());
}

#[test]
fn multimethod_dispatch_by_type() {
    let source = "def describe(n Int) 1\ndef describe(n Float) 2\nvar r1 = describe(7)\nvar r2 = describe(3.14)";
    let m = run(source);
    assert_eq!(reg(&m, "r1"), RegisterValue::Int64(1));
    assert_eq!(reg(&m, "r2"), RegisterValue::Int64(2));
}

#[test]
fn multimethod_dispatch_by_named_record_field_type() {
    let source = "def abs(num: x Int) if x < 0 then 0 - x else x\n\
def abs(num: x Float) if x < 0.0 then 0.0 - x else x\n\
var r1 = abs(num: -7)\n\
var r2 = abs(num: -3.5)";
    let m = run(source);
    assert_eq!(reg(&m, "r1"), RegisterValue::Int64(7));
    assert_eq!(reg(&m, "r2"), RegisterValue::Float64(3.5));
}

#[test]
fn scalar_call_does_not_reuse_previous_named_record_field() {
    let source = "def choose(num: x Int) 1\n\
def choose(x Int) 2\n\
var r1 = choose(num: 7)\n\
var r2 = choose(7)";
    let m = run(source);
    assert_eq!(reg(&m, "r1"), RegisterValue::Int64(1));
    assert_eq!(reg(&m, "r2"), RegisterValue::Int64(2));
}

#[test]
fn named_record_field_can_dispatch_on_nothing() {
    let source = "def classify(num: nothing) 1\n\
def classify(num: x Int) 2\n\
var r1 = classify(num: nothing)\n\
var r2 = classify(num: 7)";
    let m = run(source);
    assert_eq!(reg(&m, "r1"), RegisterValue::Int64(1));
    assert_eq!(reg(&m, "r2"), RegisterValue::Int64(2));
}

// ---------------------------------------------------------------------------
// One-liner match without `end`
// ---------------------------------------------------------------------------

#[test]
fn match_oneliner_without_end() {
    let m = run("var hit = match 7 case 7 then 1\n\
var miss = match 3 case 7 then 1\n\
var with_else = match 3 case 7 then 1 else 0\n\
var second_arm = match 3 case 1 then 10 case 3 then 30");
    assert_eq!(reg(&m, "hit"), RegisterValue::Int64(1));
    assert_eq!(reg(&m, "miss"), RegisterValue::Empty);
    assert_eq!(reg(&m, "with_else"), RegisterValue::Int64(0));
    assert_eq!(reg(&m, "second_arm"), RegisterValue::Int64(30));
}

#[test]
fn match_oneliner_inside_multiline_def_leaves_end_to_def() {
    let source = "def f(x Int)\n  match x case 1 then 10 else 20\nend\nvar r1 = f(1)\nvar r2 = f(2)";
    let m = run(source);
    assert_eq!(reg(&m, "r1"), RegisterValue::Int64(10));
    assert_eq!(reg(&m, "r2"), RegisterValue::Int64(20));
}

// ---------------------------------------------------------------------------
// REPL echo
// ---------------------------------------------------------------------------

/// Compile in REPL mode, run, and return the values of all printed registers in order.
fn repl_output(source: &str) -> Vec<RegisterValue> {
    use strontium::machine::instruction::{Instruction, InterruptKind};

    let mut compiler = Compiler::new();
    compiler.context.repl_mode = true;
    let instructions = compiler
        .compile(source.to_string())
        .expect("compilation failed");

    let printed: Vec<String> = instructions
        .iter()
        .filter_map(|instr| match instr {
            Instruction::Interrupt { interrupt } if matches!(interrupt.kind, InterruptKind::Print) => {
                Some(interrupt.address.clone())
            }
            _ => None,
        })
        .collect();

    let mut machine = Strontium::new(false);
    for instr in instructions {
        machine.push_instruction(instr);
    }
    machine.execute_until_eof().expect("execution failed");

    printed.iter().map(|address| reg(&machine, address)).collect()
}

#[test]
fn repl_echoes_expressions() {
    assert_eq!(repl_output("2 + 3"), vec![RegisterValue::Int64(5)]);
    assert_eq!(repl_output("7"), vec![RegisterValue::Int64(7)]);
    assert_eq!(repl_output("-7"), vec![RegisterValue::Int64(-7)]);
    assert_eq!(repl_output("if true then 2 else 3"), vec![RegisterValue::Int64(2)]);
    assert_eq!(repl_output("match 7 case 7 then 1"), vec![RegisterValue::Int64(1)]);
}

#[test]
fn repl_echoes_nothing() {
    assert_eq!(repl_output("match 3 case 7 then 1"), vec![RegisterValue::Empty]);
    assert_eq!(repl_output("if false then 2"), vec![RegisterValue::Empty]);
    assert_eq!(repl_output("nothing"), vec![RegisterValue::Empty]);
}

#[test]
fn repl_echoes_variables_and_calls_once() {
    assert_eq!(repl_output("var x = 5\nx"), vec![RegisterValue::Int64(5)]);
    assert_eq!(
        repl_output("def fib(0) 0\ndef fib(1) 1\ndef fib(n Int) fib(n - 1) + fib(n - 2)\nfib(10)"),
        vec![RegisterValue::Int64(55)]
    );
}

#[test]
fn repl_does_not_echo_def_var_or_print() {
    assert_eq!(repl_output("def double(n Int) n + n"), vec![]);
    assert_eq!(repl_output("var x = 5"), vec![]);
    // `print` prints its argument once and its own result is not echoed.
    assert_eq!(
        repl_output("print(\"hi\")"),
        vec![RegisterValue::String("hi".to_string())]
    );
}
