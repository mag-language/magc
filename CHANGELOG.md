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

- Extend doc comments with code examples and describe the internal workings of the compiler in greater detail.

### Fixed

- Re-enable doc references to `Lexer` and `Parser` structs by publicly exporting them in `lib.rs`.

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