# Wordle

[![Download for Windows](https://img.shields.io/badge/Download_for_Windows_10%2F11-0078D6?logo=windows&logoColor=white)](https://github.com/jackwillis/wordle/releases/latest)
[![Automated tests](https://github.com/jackwillis/wordle/actions/workflows/tests.yml/badge.svg)](https://github.com/jackwillis/wordle/actions/workflows/tests.yml)
[![Code coverage](https://codecov.io/gh/jackwillis/wordle/branch/main/graph/badge.svg?token=2Y9FF6ZM9Y)](https://codecov.io/gh/jackwillis/wordle)

Clone of the popular word guessing game.

## Building this project

### Installing Rust language

Use official installer, [rustup](https://rustup.rs/).

For Visual Studio Code, the
[rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
is recommended.

### Using Cargo

Rust's command-line tool, Cargo, manages the build.
Some useful commands:

* `cargo test`: runs unit tests
* `cargo run`: starts cross-platform desktop gui game
* `cargo run -p wordle`: starts minimalist command-line game
* `cargo build --release`: builds release binaries in "./target/"
* `cargo doc --open`: opens documentation in browser

See: [The Cargo Book](https://doc.rust-lang.org/cargo/index.html),
"[Build Commands](https://doc.rust-lang.org/cargo/commands/build-commands.html)."

### Linux build dependencies

* GCC
* GNU make
* cmake
* GNU libc headers
* libfreetype headers
* libfontconfig headers

Ubuntu/Debian: `sudo apt install build-essential cmake libfreetype-dev libfontconfig-dev`

### Continuous integration

Run the "install-git-hooks" script before making changes to this project.

See continuous integration tasks in ".github/workflows/".
