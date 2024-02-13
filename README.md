# Rust Training

## Setup

### Rust

- [Install Rust](https://www.rust-lang.org/tools/install)
- [Install Rustup](https://rustup.rs/)

### Cargo

- [Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Cargo Commands](https://doc.rust-lang.org/cargo/commands/index.html)
- [Cargo Cheatsheet](https://cheats.rs/#cargo)

### VSCode Plugins

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Your 1 stop shop for all things Rust
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - inline error hints
- [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - extra features for `Cargo.toml` files
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - C/C++ debugger with Rust support

## Material

- [Rust Slides](https://github.com/ferrous-systems/rust-training) ([Online Version](https://listochkin.ngrok-free.app/slides/))
- [Rust Exercises](https://github.com/ferrous-systems/rust-exercises)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [The Rust Standard Library Docs](https://doc.rust-lang.org/std/index.html)
- [The Rust Book - Brown University Edition](https://rust-book.cs.brown.edu/title-page.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Playground](https://play.rust-lang.org/)

## Useful crates (libraries)

- [anyhow](https://docs.rs/anyhow/latest/anyhow/) - application-level error handling
- [thiserror](https://docs.rs/thiserror/latest/thiserror/) - error development for libraries
- [log](https://crates.io/crates/log) - classical logging
- [tracing](https://crates.io/crates/tracing) - span-oriented & structured logging, suitable for concurrency
- [divan](https://crates.io/crates/divan) - statistics-driven microbenchmarking
- [serde](https://crates.io/crates/serde) - serialize/deserialize data to JSON and other formats
- [rayon](https://github.com/rayon-rs/rayon) - easy data parallelism
- [crossbeam](https://github.com/crossbeam-rs/crossbeam) - advanced concurrency primitives
- [itertools](https://docs.rs/itertools/latest/itertools/index.html) - more functions for iteration
- [tokio](https://tokio.rs) - async runtime and related libraries for Rust

## Exercices

### Day 1

- [FizzBuzz](src/fizzbuzz.rs)
- [Narcissistic Number Check](src/narcissistic_number_check.rs)
- [Rust Latin](src/rustlatin.rs)
- Rustlings:
    - [00 Intro](rustlings/exercises/00_intro)
    - [01 Variables](rustlings/exercises/01_variables)
    - [02 Functions](rustlings/exercises/02_functions)
    - [03 If](rustlings/exercises/03_if)
    - [Quiz 1](rustlings/exercises/quiz1.rs)
    - [04 Primitive Types](rustlings/exercises/04_primitive_types)
    - [05 Vecs](rustlings/exercises/05_vecs)
    - [06 Move Semantics](rustlings/exercises/06_move_semantics)
    - [07 Structs](rustlings/exercises/07_structs)
    - [08 Enums](rustlings/exercises/08_enums)
    - [09 Strings](rustlings/exercises/09_strings)
