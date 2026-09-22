# ex14: Word counts

Split on whitespace and count exact, case-sensitive words. Keep punctuation. Return owned String keys.

## Interface

```rust
pub fn word_counts(text: &str) -> std::collections::HashMap<String, usize>
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
let c = word_counts("Rust Rust rust!"); assert_eq!(c.get("Rust"), Some(&2)); assert_eq!(c.get("rust!"), Some(&1)); assert_eq!(c.len(), 2);
assert!(word_counts(" \t\n").is_empty()); assert_eq!(word_counts("猫\n猫").get("猫"), Some(&2));
```

Run `cargo test -p exercises --test ex14`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex14). Optional answer: [reference](../../../../solutions/exercises/ex14.rs). Earlier exercises teach concepts but are not code dependencies.

