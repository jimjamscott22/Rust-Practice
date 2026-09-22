# ex09: Build a struct

Create a User with the supplied name and active set to true.

## Interface

```rust
pub fn new_user(name: String) -> User
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
let u = new_user("Ada".into()); assert_eq!(u.name, "Ada"); assert!(u.active);
assert_eq!(new_user(String::new()).name, "");
```

Run `cargo test -p exercises --test ex09`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex09). Optional answer: [reference](../../../../solutions/exercises/ex09.rs). Earlier exercises teach concepts but are not code dependencies.

