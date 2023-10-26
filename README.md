![mag banner](https://world-of-music.at/downloads/bird-banner.png)

# Introduction

[`magc`](https://github.com/mag-language/magc) is a compiler library which translates Mag code into a series of executable instructions for the Strontium machine.

More specifically, the `Compiler` struct found in this library combines the `Lexer` and `Parser` modules along with its own code generation code into a pipeline. This processing pipeline then produces a sequence of bytecode instructions representing the semantics of the Mag source string and finally executes it on an instance of the `strontium` VM.

Please refer to the `mag-lang` crate to find code examples for the Mag language.

# How far along are we?

The current implementation has a fairly complete implementation of the lexer and parser stages of the compiler, so building an AST from a source string works quite well. The code generation modules are very new though, so the function set available in the REPL is limited for now. Simple arithmetic operators with two operators work already, such as `+`, `-`, `*` and `/`. More is in the works. Don't nest infix expressions for now. There are still many rough edges to this project.

## Credits

Mag is based on the Magpie language by [Robert Nystrom](http://stuffwithstuff.com/), who is a language engineer at Google with [a blog and a lot of amazing ideas](http://journal.stuffwithstuff.com/category/magpie/). His various blog posts are what started and inspired this project, and I plan on continuing his legacy even if the original codebase ceases further development.

However, since there are a few syntactical differences to the original Magpie language, the two languages are *source-incompatible* and thus have different names. In particular, Bob's implementation substitutes the dot commonly used for calling methods on objects with a space (usually a meaningless character), which I find rather unintuitive, especially for new programmers.