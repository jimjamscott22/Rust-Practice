# ex06: Borrow text

Count Unicode scalar values in borrowed text; leave the caller's String usable. Do not count bytes.

## Interface

```rust
pub fn char_count(text: &str) -> usize
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
let text = String::from("café🦀"); assert_eq!(char_count(&text), 5); assert_eq!(text, "café🦀");
assert_eq!(char_count(""), 0);
```

Run `cargo test -p exercises --test ex06`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex06). Optional answer: [reference](../../../../solutions/exercises/ex06.rs). Earlier exercises teach concepts but are not code dependencies.

