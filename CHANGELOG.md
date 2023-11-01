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

## Unreleased

### Added

- A new compilelet for multimethods.
- Desugaring for expressions contained in patterns.
- Conversion methods for wrapping errors.

### Changed

- The `ParserError` enum now resides in the `types` module.

### Fixed

- A bug which would cause the parser to return an EOF error when parsing various expressions due to a problem with the `get_lexeme` method used to retrieve parts of the source string.

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