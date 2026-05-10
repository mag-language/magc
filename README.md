![mag banner](https://world-of-music.at/downloads/bird-banner.png)

# Introduction

[`magc`](https://github.com/mag-language/magc) is a compiler library that translates Mag source code into Strontium bytecode. It combines a lexer, a Pratt parser, and a modular code generation stage into a single pipeline.

## What works today

- **Arithmetic and comparison**: `+`, `-`, `*`, `/`, `^`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Multimethods**: define multiple implementations of the same method with different signatures; the runtime dispatches based on argument value or type
- **Type-based dispatch**: `def foo(n Int)` and `def foo(n String)` coexist correctly — type annotations on parameters generate type-matching dispatch patterns
- **Conditionals**: `if <cond> then <expr> else <expr> end`
- **Variables**: `var x = <expr>` at the top level (stored in named registers) or inside method bodies (stored in stack-frame locals)
- **Return**: `return <expr>` exits a method early
- **Literals**: integers, floats, strings, booleans

## Architecture

The compiler uses a compilelet pattern: a `HashMap<String, &dyn Compilelet>` routes each expression type to the piece of code that generates bytecode for it. Adding support for a new expression kind means adding a new `Compilelet` implementation and registering it — no large match arms to touch.

Method bodies are compiled separately from the main program, then linked together in `link_bytecode`. Conditional branches use compile-time pseudo-instructions (`LabelTarget`, `JumpToLabel`, `JumpCToLabel`) that are resolved to absolute byte addresses during linking.

## Credits

Mag is based on the Magpie language by [Robert Nystrom](http://stuffwithstuff.com/). His [blog posts](http://journal.stuffwithstuff.com/category/magpie/) are the foundational inspiration for this project.
