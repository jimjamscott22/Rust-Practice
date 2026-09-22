# ex10: Methods

Implement Rectangle::area using a borrowed receiver and a u64 result. Zero dimensions produce zero.

## Interface

```rust
impl Rectangle { pub fn area(&self) -> u64
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
let r = Rectangle { width: 3, height: 4 }; assert_eq!(r.area(), 12); assert_eq!(r.width, 3);
assert_eq!(Rectangle { width: 0, height: 5 }.area(), 0); assert_eq!(Rectangle { width: u32::MAX, height: 2 }.area(), 8_589_934_590);
```

Run `cargo test -p exercises --test ex10`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex10). Optional answer: [reference](../../../../solutions/exercises/ex10.rs). Earlier exercises teach concepts but are not code dependencies.

