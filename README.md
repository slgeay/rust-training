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
- [Slides](https://rust-training.ferrous-systems.com/) / ([source](https://github.com/ferrous-systems/rust-training))
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
- [Blessed.rs - Unofficial guide to the Rust ecosystem](https://blessed.rs/crates)


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

<details open>
<summary>Day 5</summary>

- [TCP Server](src/tcp_server.rs) (improved)

</details>

<details open>
<summary>Day 6</summary>

- [Async Await](src/async_await.rs)

</details>


## Further Reading

### Learning Rust

- Our Rust training slides are often best at presenting information in a very condensed form!
  - [New slides](https://github.com/ferrous-systems/rust-training/blob/main/training-slides/src/SUMMARY.md)
  - [Older slides](https://ferrous-systems.github.io/teaching-material/index.html)
- Start with [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) - quick snippets of code with some explanations.
- [Rust Book - Brown University version](https://rust-book.cs.brown.edu) with some chapters reworked, added quizzes and memory diagrams. Very long, so I suggest reading specific chapters when you want to learn more about different topics.
- [Rustonomicon](https://doc.rust-lang.org/nomicon/) Still not sure about the topic and need to go deeper? Has great chapters on Rust memory management, unsafe, internal data representation, etc.
- [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) goes deeper into Rust memory management. Starts with `clone`, moves to reference-counted pointers, and eventually showcases the use of `unsafe` in Rust for implementing a series of data structures.

### API design and patterns

- [Tour of Rust's Standard Library Traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md) - *Super useful resource! It will help you read Rust code, understand Rust standard library code, and eventually write a more idiomatic Rust.*
- [Error Handling In Rust - A Deep Dive](https://www.lpalmieri.com/posts/error-handling-rust/) - describes Rust approach to errors and covers the use of popular error handling libraries.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - is a set of rules that Rust Standard Library follows, and every third-party API is expected to follow.
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html) lists a set of idioms with examples of their use in Rust.
- [Effective Rust](https://www.lurklurk.org/effective-rust/) teaches some useful habits when writing Rust. I especially like [the conversion methods diagram for `Result` and `Option` types in Item 3](https://www.lurklurk.org/effective-rust/transform.html)
- [Patterns with Rust Types](https://www.shuttle.rs/blog/2022/07/28/patterns-with-rust-types) - a quick article showing how several Rust type system concepts work together.

### Rust projects organization

- Our slides on Cargo:
  - [Cargo commands](https://github.com/ferrous-systems/rust-training/blob/main/training-slides/src/using-cargo.md)
  - [How dependency resolution works](https://github.com/ferrous-systems/rust-training/blob/main/training-slides/src/dependency-management.md)
  - [Cargo Workspaces](https://github.com/ferrous-systems/rust-training/blob/main/training-slides/src/cargo-workspaces.md) - **your internal projects should be workspaces 99% of the time**
  - [Rust build times](https://github.com/ferrous-systems/rust-training/blob/main/training-slides/src/rust-build-time.md)
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
  - [Xtask Pattern for project tasks](https://github.com/matklad/cargo-xtask)
  - [Large Rust workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)

### Rust FFI

- [Rustonomicon has a very good chapter on Rust FFI.](https://doc.rust-lang.org/nomicon/ffi.html) It covers C-to-Rust and Rust-to-C function calls, callbacks, data representation, opaque types, accessing static / global data, and stack unwinding.

### Async Rust and Multithreading

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) has a few sections covering async Rust in general.
- [Async Rust Today](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo.html) - written as stories with each one talking about a specific problem or difficulty and offering a set of solutions.
- [Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/) talks about using channels to work with resources.
- [mini-redis](https://github.com/tokio-rs/mini-redis) example - a very well-commented piece of code that showcases some real-world scenarios: async data sharing, work cancellation, graceful shutdown, etc. Many libraries and web frameworks take care of these tasks for you but this example is still useful to learn from.
- [Rust Atomics and Locks](https://marabos.nl/atomics/) - at this point the best book about low-leven concurrency in general, not only in Rust. Describes things like locking, compare-and-swap, memory ordering, etc. on a hardware level, OS level and finally language level. If certain multithreading concepts make you feel lost this would be an excellent book.

### Videos

- 📽 [Ergonomic APIs for hard problems - Raph Levien](https://www.youtube.com/watch?v=Phk0C-kLlho) - a Keynote talk about some aspects of great API design. Raph is heavily involved in Rust GUI scene, but his advice is very much applicable to non-GUI projects, too.
- 📽 [Logan Smith has a good playlist](https://www.youtube.com/playlist?list=PLhjB8nmMLotIG0ik1RXjl0lfZcg9oxhMg) talking about various aspects of Rust API design
- 📽 [Best lifetime explanation video](https://www.youtube.com/watch?v=gRAVZv7V91Q)
- 📽 [Jon Gjengset](https://www.youtube.com/@jonhoo) makes exploration videos about various Rust features

### Code

Crates.io website is a somewhat small web application, but it handles large amounts of traffic and covers common Server-side use-cases:

- Authenticating with third-party services
- Authorization and access control
- File uploads and downloads
- Background jobs
- Error reporting, logging, monitoring, gathering metrics

Axum, SQLx, PostgreSQL, Heroku, AWS, Fastly

### Books

- 📖 [Zero To Production In Rust](https://www.zero2prod.com/) the book walks you through building a typical web API backend and showcases how different aspects of it can be done in Rust. A large portion of the book is available as [a series of blog posts on author's blog](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/) (scroll down to ToC).
- 📖 [Rust for Rustaceans](https://rust-for-rustaceans.com) is an intermediate-level book. It's very dense, contains almost no examples and instead talks about various aspects of the language in detail. Think "Rustonomicon+". Do not rush to read it right away, instead go back to it once you've done some Rust work and you feel the need to understand language fundamentals deeper.
- 📖 [Asynchronous Programming in Rust](https://www.packtpub.com/product/asynchronous-programming-in-rust/9781805128137) book by Carl Fredrik Samson explains in detail how different portions of Async Rust work. Doesn't go into patterns of Rust async programming, unfortunately.

## Arcane knowledge

- [Rusts Axum style magic function params example](https://github.com/alexpusch/rust-magic-function-params) another article that showcases the power of Rust's static resolution of generics
- [Rust RFC book](https://rust-lang.github.io/rfcs/). Every language feature is added to a language through an RFC process. Find a relevant RFC, and not only it will have explanations, but also it will link to few GitHub issue threads where people debated the RFC and its implementation.
- [Ferrocene Language Specification](https://spec.ferrocene.dev/index.html) - a WIP reference about Rust-the-language. Doesn't cover standard library yet.
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/getting-started.html) talks about compiler internals, including how certain features are implemented
