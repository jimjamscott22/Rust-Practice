# ex13: Vectors

Return unique values in their original order, without changing the input. A simple linear membership search is sufficient.

## Interface

```rust
pub fn unique(values: &[i32]) -> Vec<i32>
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(unique(&[3, 1, 3, 2, 1]), vec![3, 1, 2]);
assert_eq!(unique(&[]), Vec::<i32>::new()); assert_eq!(unique(&[-1, -1, 0]), vec![-1, 0]);
```

Run `cargo test -p exercises --test ex13`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex13). Optional answer: [reference](../../../../solutions/exercises/ex13.rs). Earlier exercises teach concepts but are not code dependencies.

