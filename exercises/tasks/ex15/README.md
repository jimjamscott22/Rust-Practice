# ex15: Parse a number

Trim surrounding whitespace and parse a u32, returning the standard ParseIntError on invalid input or overflow.

## Interface

```rust
pub fn parse_count(text: &str) -> Result<u32, std::num::ParseIntError>
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(parse_count(" 42 "), Ok(42)); assert_eq!(parse_count("0"), Ok(0));
assert!(parse_count("-1").is_err()); assert!(parse_count("").is_err()); assert!(parse_count("4294967296").is_err());
```

Run `cargo test -p exercises --test ex15`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex15). Optional answer: [reference](../../../../solutions/exercises/ex15.rs). Earlier exercises teach concepts but are not code dependencies.

