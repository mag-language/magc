![mag banner](https://world-of-music.at/downloads/bird-banner.png)

# Introduction

[`magc`](https://github.com/mag-language/magc) is a compiler library which translates Mag source code into a series of bytecode instructions for the [`strontium`](https://github.com/mag-language/magc) machine.

This is what a simple (and very inefficient) fibonacci function looks like in Mag:

```python
def fib(0) 0
def fib(1) 1
def fib(n Int) fib(n - 2) + fib(n - 1)
```

The three definitions generate a selection of methods with the same name, but different arguments.

## Credits

Mag is based on the Magpie language by [Robert Nystrom](http://stuffwithstuff.com/), who is a language engineer at Google with [a blog and a lot of amazing ideas](http://journal.stuffwithstuff.com/category/magpie/). His various blog posts are what started and inspired this project, and I plan on continuing his legacy even if the original codebase ceases further development.

However, since there are a few syntactical differences to the original Magpie language, the two languages are *source-incompatible* and thus have different names. In particular, Bob's implementation substitutes the dot commonly used for calling methods on objects with a space (usually a meaningless character), which I find rather unintuitive, especially for new programmers.