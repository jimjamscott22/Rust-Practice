# Project overview

Rust Practice Lab is a Rust 2024 Cargo workspace with two standard-library-only packages:
exercises (20 isolated learning targets plus a solved example) and capstone (a task-list CLI).

Preserve the learning experience: starter files intentionally contain todo!(), while complete
answers live under solutions/ outside normal build targets. Never replace all starters with
solutions during maintenance. Verify answers in a temporary workspace using the original tests.

Each exercise owns its starter, tests, and instructions. Keep capstone model, command parsing,
storage, and executable entry point in separate modules. Use portable Cargo commands and no
third-party dependencies. Keep docs/ for curriculum, hints, plans, and implementation summaries.
After repository changes, update docs/implementation-summary.md.

Tests for unfinished learner targets are expected to fail at runtime. Compilation, ex00, and
reference-solution tests must pass. Check formatting for starters and reference files.

