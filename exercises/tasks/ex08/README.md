# ex08: Lifetimes and slices

Return the longer input by Unicode scalar count. Ties return the first input. Borrow the result without allocating; give both inputs and output one explicit lifetime.

## Interface

```rust
pub fn longer<'a>(left: &'a str, right: &'a str) -> &'a str
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(longer("a", "rust"), "rust"); assert_eq!(longer("🦀", "ab"), "ab");
assert_eq!(longer("ab", "cd"), "ab"); assert_eq!(longer("", ""), "");
```

Run `cargo test -p exercises --test ex08`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex08). Optional answer: [reference](../../../../solutions/exercises/ex08.rs). Earlier exercises teach concepts but are not code dependencies.

