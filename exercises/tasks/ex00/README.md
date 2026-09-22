# Worked example: double a number

Run `cargo test -p exercises --test ex00` from the workspace root. This example is already solved.

`double(value: i32) -> i64` borrows nothing: integers are Copy. Convert to i64 before multiplying so every i32 input fits. The last expression is returned without a semicolon.

Open starter.rs and tests.rs side by side. Change `* 2` to `* 3`, rerun, and read the assertion: left is actual, right is expected. Restore `* 2` afterward. Exercise 01 replaces the implementation with `todo!()`: that compiles but panics when called. Replace its body, not the tests.

