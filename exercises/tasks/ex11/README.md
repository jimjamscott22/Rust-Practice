# ex11: Enums

Return the number of seconds represented by a Wait variant. Minutes must be converted using u64 arithmetic.

## Interface

```rust
pub fn seconds(wait: Wait) -> u64
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(seconds(Wait::Seconds(7)), 7); assert_eq!(seconds(Wait::Minutes(2)), 120);
assert_eq!(seconds(Wait::Minutes(0)), 0); assert_eq!(seconds(Wait::Minutes(u32::MAX)), u64::from(u32::MAX) * 60);
```

Run `cargo test -p exercises --test ex11`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex11). Optional answer: [reference](../../../../solutions/exercises/ex11.rs). Earlier exercises teach concepts but are not code dependencies.

