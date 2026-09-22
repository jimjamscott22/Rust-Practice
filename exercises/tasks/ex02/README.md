# ex02: Temperature conversion

Convert Celsius to Fahrenheit using f64. Compare floating-point results with a tolerance.

## Interface

```rust
pub fn fahrenheit(celsius: f64) -> f64
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert!((fahrenheit(100.0) - 212.0).abs() < 1e-9);
assert!((fahrenheit(-40.0) + 40.0).abs() < 1e-9); assert!((fahrenheit(0.5) - 32.9).abs() < 1e-9);
```

Run `cargo test -p exercises --test ex02`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex02). Optional answer: [reference](../../../../solutions/exercises/ex02.rs). Earlier exercises teach concepts but are not code dependencies.

