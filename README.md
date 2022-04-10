# wordle

[![Build](https://github.com/jackwillis/wordle/actions/workflows/build.yml/badge.svg)](https://github.com/jackwillis/wordle/actions/workflows/build.yml)
[![Code coverage](https://codecov.io/gh/jackwillis/wordle/branch/main/graph/badge.svg?token=2Y9FF6ZM9Y)](https://codecov.io/gh/jackwillis/wordle)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Word guessing game.

## Building this project

### Installing Rust language

Use official installer, [rustup](https://rustup.rs/).

If using Visual Studio Code, install the
[rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer),
and uninstall the [official Rust one](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) if you have it.
Rust-analyzer is better and active and it's being phased in to replace the other.

### Using Cargo

Rust's command-line tool, Cargo, manages the build.
Some useful commands:

* `cargo test`  
runs unit tests
* `cargo doc --open`  
opens documentation in browser
* `cargo run -p wordle`  
starts minimalist command-line game
* `cargo run -p wordle-gui`  
starts cross-platform desktop gui game
* `cargo build --release`  
builds release binaries in "./target/release"

See: [The Cargo Book](https://doc.rust-lang.org/cargo/index.html),
"[Build Commands](https://doc.rust-lang.org/cargo/commands/build-commands.html)."

### Linux dependencies

Ubuntu: `sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

Fedora: `sudo dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel`

### Continuous integration

Run the "install-git-hooks" script before using Git.
See also GitHub workflows.
