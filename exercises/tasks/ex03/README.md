# ex03: Classify a number

Return "negative", "zero", or "positive". Practice match with guards.

## Interface

```rust
pub fn classify(n: i32) -> &'static str
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(classify(-4), "negative"); assert_eq!(classify(5), "positive");
assert_eq!(classify(0), "zero"); assert_eq!(classify(i32::MIN), "negative");
```

Run `cargo test -p exercises --test ex03`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex03). Optional answer: [reference](../../../../solutions/exercises/ex03.rs). Earlier exercises teach concepts but are not code dependencies.

