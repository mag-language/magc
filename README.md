![mag banner](https://world-of-music.at/downloads/bird-banner.png)

# Introduction

This repository contains a compiler which converts Mag source files to executable binaries for various hardware architectures. The main compilation target will be the Strontium VM, but the roadmap includes ahead-of-time compilation to fast and efficient binary code using the [`cranelift`](https://docs.rs/cranelift/latest/cranelift/) crate in the future.

## Credits

Mag is based on the Magpie language by [Robert Nystrom](http://stuffwithstuff.com/), who is a language engineer at Google with [a blog and a lot of amazing ideas](http://journal.stuffwithstuff.com/category/magpie/). His various blog posts are what started this repository, and I plan on continuing his legacy even if the original codebase ceases further development.

However, since there are a few syntactical differences to the original Magpie langauge, the two languages will be *source-incompatible*. In particular, Bob's implementation substitutes the dot commonly used for calling methods on objects with spaces, which I find rather unintuitive, especially for new programmers.