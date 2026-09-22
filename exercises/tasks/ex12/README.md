# ex12: Optional values

Find the first even integer and return a copy in Some, or None if absent. Zero and negative even values count.

## Interface

```rust
pub fn first_even(values: &[i32]) -> Option<i32>
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(first_even(&[3, -4, 8]), Some(-4)); assert_eq!(first_even(&[0]), Some(0));
assert_eq!(first_even(&[]), None); assert_eq!(first_even(&[1, 3]), None);
```

Run `cargo test -p exercises --test ex12`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex12). Optional answer: [reference](../../../../solutions/exercises/ex12.rs). Earlier exercises teach concepts but are not code dependencies.

