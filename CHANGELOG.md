# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
    Add new changelog entries here.
    Each entry may be annotated with "Added", "Changed", "Removed", and "Fixed" titles.

    Example:

    ## [1.0.0] - May 16, 2022

    ### Added
    - New visual identity.

    ### Changed
    - Start using "changelog" over "change log" since it's the common usage.

    ### Removed
    - Section about "changelog" vs "CHANGELOG".

    ### Fixed
    - Fix typos in recent README changes.
    - Update outdated unreleased diff link.
-->

## [Unreleased]

### Removed

- `Multimethod::linearize`, `Pattern::linearize`, `Pattern::matches_with`, `Pattern::get_precedence`, and the `LinearizeResult` type alias. They were no longer called anywhere: multimethod dispatch and precedence are resolved at compile time via `DispatchPattern`.

## [0.9.0] - September 13, 2026

### Added

- `match`/`case` expressions with value, type, wildcard, and record patterns. Arm patterns can bind variables for use in the arm body.
- Record patterns in `match` subjects and arms, including tuple-valued fields (e.g. `match name: "Dan", pals: ("Sam", "Ed")`).
- Multimethod dispatch on named record fields via `DispatchPattern::Record`, so `def abs(num: x Int)` and `def abs(num: x Float)` coexist. Each field's presence is tracked in an `arg.<field>.__present` register so a call never matches a field left over from a previous call.
- A `nothing` literal and a `Nothing` type annotation. Booleans, strings, and `nothing` can now be used as dispatch values.
- Multi-line bodies for `def`, `if`, and `match` arms: a body starting on the line after `)` or `then` runs until `end`. Same-line bodies are still a single expression.
- One-liner `match` expressions without `end`: a `match` whose first `case` is on the same line as `match` ends at the line break (e.g. `match x case 1 then "one" else "other"`). A trailing `end` on that line is still accepted.
- Unary `+` and `-` via the new `PrefixCompilelet`.
- The `~` operator for string concatenation.
- Calling a multimethod with an argument that matches none of its variants now raises a runtime error naming the method and the argument.
- In `repl_mode`, `Compiler::compile` echoes the value of every top-level expression, including literals, `if`, `match`, and `nothing`. Method definitions, `var` bindings, and `print` calls are not echoed. This replaces the auto-printing previously scattered across individual compilelets, which skipped most expression kinds.
- `Lexer::reset`, `Parser::reset_tokens`, and `Compiler::is_complete` for detecting incomplete input.
- An integration test suite covering prefix operators, conditionals, `match`, multi-line definitions, and multimethod dispatch.

### Changed

- Multimethod dispatch now happens entirely at compile time: `link_bytecode` generates one dispatch shim per multimethod and patches every `CallShim` into a direct `Call`. `DispatchPattern` moved from `strontium` into `magc::dispatch`.
- `FieldPattern` is renamed to `RecordPattern` (`Pattern::Field` → `Pattern::Record`, `expect_field` → `expect_record`).
- `Compiler::compile` resets the lexer and parser before each call, so every REPL line is parsed on its own.
- `Multimethod::add_method` takes a precomputed `DispatchPattern` instead of a `&Parser`, and `Compiler::generate_method_id` derives method IDs from the dispatch pattern.
- The `else` branch of `match` is now optional (`MatchExpression::else_arm` is an `Option`). A `match` without `else` evaluates to `nothing` when no arm matches, like `if` without `else`.
- `Compiler::new` uses `env_logger::try_init`, so more than one compiler can be created per process.

### Fixed

- Defining value-pattern variants on separate REPL lines (e.g. `def fib(0) 0` followed by `def fib(1) 1`) no longer fails with "this method signature has already been defined". Such variants also no longer overwrite each other's compiled bodies.
- The parser now skips comment tokens instead of failing on them.
- `magc` builds on stable Rust again: the unused `#![feature(type_ascription)]` attribute, which required a nightly toolchain, was removed.

## [0.8.0] - May 10, 2026

### Added

- `if/then/else` conditional expressions, compiled via `ConditionalCompilelet` using a label-based pseudo-instruction system (`LabelTarget`, `JumpToLabel`, `JumpCToLabel`) that is resolved to absolute byte addresses in `link_bytecode`.
- `var` declarations — at the top level, values are stored in named VM registers (persisting across REPL iterations); inside method bodies, `StoreLocal`/`LoadLocal` is used instead.
- `return` expressions inside method bodies, emitting a `Copy` to the `ret` register followed by `Return`.
- `repl_mode` flag on `CompilationContext`: auto-printing of top-level expression results is now REPL-only and does not trigger when executing source files.
- `global_variables` set on `CompilationContext` to track top-level `var` bindings.
- Label allocation (`alloc_label`) and resolution (`resolve_labels`) for compiling control flow with forward references.
- Type-based dispatch: `pattern_to_dispatch_pattern` now maps type-annotated variable patterns (e.g. `n Int`) to `DispatchPattern::Type(RegisterType::*)`, allowing multiple methods with the same name but different argument types to coexist in the dispatch table.

### Fixed

- Parser bug: `VariablePatternParselet` was storing the variable name in `type_id` instead of the type token's lexeme, causing all type annotations to be silently ignored during compilation.
- Bare variable references at the top level of the REPL (e.g. `>>> x`) now auto-print their value.

## [0.7.0] - May 9, 2026

### Added

- Full multimethod compilation pipeline with proper bytecode linking — method bodies are compiled separately, addresses are resolved before execution, and registration metadata is emitted for the runtime.
- `CompiledMethod`, `PendingCall`, and `MethodRegistration` types to track method bodies, pending address fixups, and dispatch registrations through the compilation process.
- A new compilelet for multimethods.
- Desugaring for expressions contained in patterns.
- Improved error handling, printing the offending line to the console, along with conversion methods for wrapping errors.

### Changed

- The `ParserError` enum now resides in the `types` module.

### Fixed

- UTF-8 lexeme extraction: the parser now stores source as a `Vec<String>` of grapheme clusters (matching the lexer) instead of a raw `String`, fixing incorrect byte-index slicing of multi-byte characters that caused subsequent REPL lines to be parsed from the wrong offset.
- Recursive method calls were not compiled or linked correctly.
- A bug which would cause the parser to return an EOF error when parsing various expressions due to a problem with the `get_lexeme` method.

## [0.1.1] - October 27, 2023

### Added

- New sections in the crate documentation, providing code examples and more detailed information about the inner workings of the compiler.
- A new `desugar` method for the `ExpressionKind` type, which converts syntactic sugar constructs like infix expressions to their semantic counterparts just before compilation.
- The `log` and `env_logger` dependencies printing the compilation tree for easier debugging.
- The `CompilationContext` struct which keeps track of recursion depth in the compiler so that we can print the final output of a calculation without any other intermediates.

### Changed

- `pattern.expect_value()` now provides the inner expression directly instead of the value pattern.

### Fixed

- Re-enable doc references to `Lexer` and `Parser` structs by publicly exporting them in `lib.rs`.
- Nested infix expressions now work properly.

## [0.1.0] - October 26, 2023

### Added
- A first implementation of the `Compiler` struct converting infix expressions to Strontium instructions.
- Modular parser and compiler design by passing control to a `Parselet` or `Compilelet` trait implementors based on the given token or expression.
- A new member parselet which parses expressions like `person.favoriteColor`.
- A trait called `Typed` which defines an interface for anything that has a type.
- Various implementations of the former trait so we can retrieve the types of any `Expression`, `Literal` or `Pattern`.
- A new parser error called `NoMatch` which is returned if two patterns don't match.
- A new parser error called `UnexpectedType` which is returned if there is a type mismatch.
- Various implementations of the `linearize` method to enable destructuring pattern matching.
- Human-readable changelogs.

### Fixed
- Make sure boolean values are accounted for in the `parse_identifier_or_keyword` method.
- An error where types starting with `U` would not be tokenized correctly.