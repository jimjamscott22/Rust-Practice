# ex19: Generics

Return a borrowed reference to the smallest value, or None for an empty slice. Support any Ord type without requiring Clone or Copy.

## Interface

```rust
pub fn smallest<T: Ord>(values: &[T]) -> Option<&T>
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(smallest(&[3, -1, 5]), Some(&-1)); let words = [String::from("z"), String::from("a")]; assert_eq!(smallest(&words), Some(&words[1]));
assert_eq!(smallest::<i32>(&[]), None); assert_eq!(smallest(&[7]), Some(&7));
```

Run `cargo test -p exercises --test ex19`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex19). Optional answer: [reference](../../../../solutions/exercises/ex19.rs). Earlier exercises teach concepts but are not code dependencies.

