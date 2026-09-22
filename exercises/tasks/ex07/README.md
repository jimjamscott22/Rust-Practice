# ex07: Mutable borrow

Append a supplied suffix to an existing String through a mutable reference.

## Interface

```rust
pub fn append(text: &mut String, suffix: &str)
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
let mut s = String::from("Rust"); append(&mut s, " rocks"); assert_eq!(s, "Rust rocks");
let mut s = String::new(); append(&mut s, "🦀"); append(&mut s, ""); assert_eq!(s, "🦀");
```

Run `cargo test -p exercises --test ex07`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex07). Optional answer: [reference](../../../../solutions/exercises/ex07.rs). Earlier exercises teach concepts but are not code dependencies.

