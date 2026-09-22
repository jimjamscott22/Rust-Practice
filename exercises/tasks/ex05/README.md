# ex05: Move and return

Take ownership of a String, append !, and return it. Mutate the original allocation rather than cloning.

## Interface

```rust
pub fn exclaim(mut text: String) -> String
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(exclaim(String::from("Rust")), "Rust!");
assert_eq!(exclaim(String::new()), "!");
```

Run `cargo test -p exercises --test ex05`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex05). Optional answer: [reference](../../../../solutions/exercises/ex05.rs). Earlier exercises teach concepts but are not code dependencies.

