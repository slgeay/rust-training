# Rust Training

## Setup

<details>
<summary>Installation</summary>

### Rust

- [Install Rust](https://www.rust-lang.org/tools/install)

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

</details>

## Material

<details>
<summary>Course, exercices, docs</summary>

### Course
- [Slides](https://listochkin.ngrok-free.app/slides/) / ([source](https://github.com/ferrous-systems/rust-training))
- [Exercises](https://github.com/ferrous-systems/rust-exercises)
- [Legacy slides](https://ferrous-systems.github.io/teaching-material/) / ([source](https://github.com/ferrous-systems/teaching-material))

### Tools
- [Rust Playground](https://play.rust-lang.org/)

### Exercices
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Quiz](https://dtolnay.github.io/rust-quiz) / ([source](https://github.com/dtolnay/rust-quiz))

### Docs
- [The Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [The Rust Book - Brown University Edition](https://rust-book.cs.brown.edu)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Cheatsheet](https://cheats.rs/)
- [The Little Book of Rust Books](https://lborb.github.io/book/)
- [Lib.rs](https://lib.rs/)


</details>

## Crates

<details>
<summary>Useful libraries</summary>

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

</details>

## Exercices

<details open>
<summary>Day 1</summary>

- [FizzBuzz](src/fizzbuzz.rs)
- [Narcissistic Number Check](src/narcissistic_number_check.rs)
- [Rust Latin](src/rustlatin.rs)
- [Rustlings](https://github.com/slgeay/rustlings/tree/main/exercises) (Exercices 00 to 11, Quizzes 1 and 2)

</details>

<details open>
<summary>Day 2</summary>

- [Calculator](src/calculator.rs)

</details>

<details open>
<summary>Day 3</summary>

- [TCP Server](src/tcp_server.rs)
- [Shapes](src/shapes.rs)

</details>

<details open>
<summary>Day 4</summary>

- [Serde Lifetimes](src/serde_lifetimes.rs)

</details>
