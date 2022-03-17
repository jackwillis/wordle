# wordle

[![Build](https://github.com/jackwillis/wordle/actions/workflows/build.yml/badge.svg)](https://github.com/jackwillis/wordle/actions/workflows/rust.yml)
[![Code coverage](https://github.com/jackwillis/wordle/actions/workflows/build.yml/badge.svg)](https://github.com/jackwillis/wordle/actions/workflows/coverage.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Command-line wordle clone in Rust.

## Rules and functionality

* The goal of the game is to guess the secret word.
* The secret word is a random five-letter-long English word.
* You may make up to six guesses.
* You may guess any five-character sequence.
* After you guess, the outcome of your guess will be shown.  
    * If the first letter guessed matches the first letter of the secret word, the first symbol of the outcome will be 'X', likewise for second, third, forth, and fifth.
    * 'O' means the letter is present in the word but not at that position.
    * '_' means the letter is not present in the word.
* Known good, bad, and unknown letters are shown to the side of the outcome.

## Building this project

Install Rust programming language using official installer, [rustup](https://rustup.rs/).

If using Visual Studio Code, install the
[rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer),
and uninstall the [official Rust one](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) if you have it.
Rust-analyzer is better and active and it's being phased in to replace the other.

All build tasks are handled with Rust's command-line tool, cargo,
which automagically handles dependencies and compilation.
The command [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) will run tests
and [`cargo run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html) will run the binary.
See: [The Cargo Book](https://doc.rust-lang.org/cargo/index.html),
"[Build Commands](https://doc.rust-lang.org/cargo/commands/build-commands.html)." 

See also [clippy](https://github.com/rust-lang/rust-clippy),
a linter which is used in the
[continuous integration](https://github.com/jackwillis/wordle/actions/workflows/rust.yml).
