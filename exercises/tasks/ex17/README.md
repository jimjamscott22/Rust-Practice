# ex17: Iterators

Return squares of the even inputs, preserving order. Use filter, map, and collect; convert to i64 before squaring.

## Interface

```rust
pub fn even_squares(values: &[i32]) -> Vec<i64>
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(even_squares(&[-4, 1, 2, 0]), vec![16, 4, 0]);
assert!(even_squares(&[]).is_empty()); assert_eq!(even_squares(&[i32::MIN]), vec![4_611_686_018_427_387_904]);
```

Run `cargo test -p exercises --test ex17`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex17). Optional answer: [reference](../../../../solutions/exercises/ex17.rs). Earlier exercises teach concepts but are not code dependencies.

