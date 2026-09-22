# Rust Practice Lab 🦀

Learn Rust by completing **20 small exercises**, then build a task-list CLI in five milestones. Designed for programmers new to Rust.

## Start here

You need Rust and Cargo with support for the **2024 edition** (Rust 1.85 or newer). Check with `rustc --version` and `cargo --version`. If needed, follow the official [Rust installation guide](https://www.rust-lang.org/tools/install). This project has no external dependencies.

Open a terminal in this folder:

```text
cargo test -p exercises --test ex00
cargo test -p exercises --test ex01
```

The completed ex00 example passes. **ex01 initially fails on purpose**, with a message naming its unfinished task. Read [ex01 instructions](exercises/tasks/ex01/README.md), replace the `todo!()` in its `starter.rs`, and rerun until both tests pass.

1. Read the task and its tests.
2. Implement only the starter code; retain public signatures and tests.
3. Run that task's command. Compiler errors are feedback; failed assertions show actual versus expected values.
4. Try [progressive hints](docs/hints.md) if stuck.
5. Compare the [optional reference answer](solutions/README.md) after solving.
6. Mark your progress in the [curriculum checklist](docs/curriculum.md).

Each exercise is independent. `cargo test -p exercises --test ex08` runs exercise 08 without compiling other exercise targets. Avoid `cargo test --workspace` until everything is solved: it deliberately encounters unfinished tasks.

## Course map

| Tasks | Concepts |
| --- | --- |
| 01–04 | Formatting, numbers, conditions, loops |
| 05–08 | Ownership, borrowing, mutable references, lifetimes |
| 09–12 | Structs, methods, enums, Option |
| 13–16 | Vec, HashMap, Result, error propagation |
| 17–20 | Iterators, traits, generics, records |
| Capstone 1–5 | Build and persist a task-list CLI |

Start with the [worked example](exercises/tasks/ex00/README.md). After exercise 20, open the [capstone guide](capstone/README.md).

## Useful commands

```text
cargo test -p exercises --test ex01
cargo test -p capstone --test milestone1
cargo test --workspace --no-run
cargo fmt --all -- --check
```

The no-run command checks compilation of all starters without executing TODOs. Use `cargo fmt --all` to format your code. Reference answers are outside normal Cargo targets.

This is a learning project: tasks intentionally remain unfinished, and no test failures are hidden or ignored. There is no progress database, watcher, or custom runner.

See [troubleshooting](docs/troubleshooting.md) and [implementation notes](docs/implementation-summary.md).

