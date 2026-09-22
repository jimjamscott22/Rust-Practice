# Implementation summary

Created a Rust 2024, standard-library-only Cargo workspace for programmers new to Rust.

- 20 independent exercises, each with instructions, starter code, and normal/edge-case tests.
- A completed example, progressive hints, manual curriculum checklist, and detached solutions.
- A five-milestone task-list CLI with separate model, argument parsing, storage, and entry point.
- Capstone tests cover persistence across processes, Unicode titles, stable IDs, malformed data,
  invalid UTF-8, argument errors, missing files, and filesystem errors.
- README onboarding, capstone instructions, troubleshooting, and project conventions.

## Verification (2026-09-21)

Tested on Windows with rustc 1.97.1 and Cargo 1.97.1.

- `cargo fmt --all -- --check`: passed; detached solutions also formatted with rustfmt.
- `cargo test --workspace --no-run`: all learner targets compile.
- `cargo test -p exercises --test ex00`: completed example passes.
- `cargo test -p exercises --test ex01`: fails at its named TODO as intended.
- All 20 exercise starters fail at their TODOs; all five unfinished milestones fail.
- Isolation verified in the temporary workspace: ex01 passes even with invalid Rust syntax
  deliberately inserted in ex20, then restored.
- Reference solutions substituted only in a temporary workspace: all **50 tests pass**
  (41 exercise/example tests and 9 capstone tests), with no ignored tests.

The learner workspace retains its TODOs. Whole-workspace test failures are intentional until
the course is completed. Tests check behavior; instructions additionally require techniques
such as borrowing without cloning, loops, and iterator chains.

## Maintenance verification

To check answers without changing learner work, create a temporary directory and copy only
Cargo.toml, Cargo.lock, exercises/, and capstone/ into it. Replace each copied exercise's
starter.rs with its matching solutions/exercises/exNN.rs, and replace the copied capstone
src/*.rs files with solutions/capstone/*.rs (retain lib.rs). Run `cargo test --workspace`
inside that temporary copy. Do not copy answers over the learner workspace.

This project does not include concurrency, external packages, a custom runner, or automatic
progress tracking. Capstone storage is for a single local writer. The current folder was
not a Git repository, so no commit was created.
