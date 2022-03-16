# wordle

[![Continuous integration](https://github.com/jackwillis/wordle/actions/workflows/rust.yml/badge.svg)](https://github.com/jackwillis/wordle/actions/workflows/rust.yml)

command-line wordle clone in Rust

published under [MIT license](LICENSE)

## Building this project

Install Rust programming language using official installer, [rustup](https://rustup.rs/).

If using Visual Studio Code, install the
[rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer),
and uninstall the [official Rust one](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) if you have it.
Rust-analyzer is better and active and it's being phased in to replace the other.

All build tasks are handled with Rust's tool, cargo.
The command `cargo run` will fetch dependencies, build the package, and run the binary.
See: [The Cargo Book](https://doc.rust-lang.org/cargo/index.html),
"[Build Commands](https://doc.rust-lang.org/cargo/commands/build-commands.html)." 
