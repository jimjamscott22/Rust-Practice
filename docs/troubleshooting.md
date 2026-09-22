# Troubleshooting

- **cargo not found:** install Rust from the official rustup installer, then reopen your terminal.
- **Linker missing on Windows:** follow the Rust installer guidance for Visual Studio C++ Build Tools and the Windows SDK. Rust compilation needs a native linker.
- **Edition 2024 unsupported:** use Rust 1.85 or newer; this project was developed with 1.97.1.
- **not yet implemented:** expected for unfinished tasks. Replace the named TODO in the starter.
- **assertion failed:** left is the actual result, right is expected. Read the task's examples and boundaries.
- **Other exercises fail:** select one target with --test exNN rather than running the whole workspace.
- **Compiler error after an edit:** this is part of practice. Read the first error; changing tests or public signatures usually hides the issue.
- **Borrow checker complaint:** identify the owner, each borrow, and when each reference is last used. Avoid adding clone automatically.
- **Solutions are not compiled:** intentional. They are optional references, outside Cargo targets.
- **Capstone panics in an earlier milestone:** later stages depend on earlier stages. Finish them in order.
- **Title split into multiple arguments:** quote it: cargo run -p capstone -- add "Learn Rust".
- **Data file location:** relative paths use the terminal's current directory. Use --file before the verb for another location.
- **Formatting:** cargo fmt --all formats learner code. It does not format the detached reference files.

