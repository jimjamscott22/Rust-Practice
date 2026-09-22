# ex18: Traits

Implement Describe for Book. Produce exactly "TITLE by AUTHOR".

## Interface

```rust
impl Describe for Book { fn describe(&self) -> String
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
let b = Book { title: "Rust".into(), author: "Ada".into() }; assert_eq!(b.describe(), "Rust by Ada");
let b = Book { title: "".into(), author: "猫".into() }; assert_eq!(b.describe(), " by 猫");
```

Run `cargo test -p exercises --test ex18`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex18). Optional answer: [reference](../../../../solutions/exercises/ex18.rs). Earlier exercises teach concepts but are not code dependencies.

