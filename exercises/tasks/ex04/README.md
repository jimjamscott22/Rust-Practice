# ex04: Accumulate

Sum integers from 1 through n using a loop. Return u64 so the sum fits for every u32 input. Do not use the arithmetic-series formula.

## Interface

```rust
pub fn sum_to(n: u32) -> u64
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(sum_to(5), 15); assert_eq!(sum_to(100_000), 5_000_050_000);
assert_eq!(sum_to(0), 0); assert_eq!(sum_to(1), 1);
```

Run `cargo test -p exercises --test ex04`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex04). Optional answer: [reference](../../../../solutions/exercises/ex04.rs). Earlier exercises teach concepts but are not code dependencies.

