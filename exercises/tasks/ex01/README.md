# ex01: Greeting

Return `Hello, {name}!` without trimming the name. Empty names are allowed.

## Interface

```rust
pub fn greet(name: &str) -> String
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(greet("Ada"), "Hello, Ada!"); assert_eq!(greet("世界"), "Hello, 世界!");
assert_eq!(greet(""), "Hello, !"); assert_eq!(greet(" Ada "), "Hello,  Ada !");
```

Run `cargo test -p exercises --test ex01`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex01). Optional answer: [reference](../../../../solutions/exercises/ex01.rs). Earlier exercises teach concepts but are not code dependencies.

