# ex16: Propagate errors

Parse two trimmed u32 inputs and return their sum as u64. Use ? to return the first parse error; valid operands cannot overflow the result.

## Interface

```rust
pub fn sum_strings(left: &str, right: &str) -> Result<u64, std::num::ParseIntError>
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
assert_eq!(sum_strings(" 2", "3 "), Ok(5)); assert_eq!(sum_strings("4294967295", "1"), Ok(4_294_967_296));
assert!(sum_strings("no", "3").is_err()); assert!(sum_strings("2", "-1").is_err());
```

Run `cargo test -p exercises --test ex16`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex16). Optional answer: [reference](../../../../solutions/exercises/ex16.rs). Earlier exercises teach concepts but are not code dependencies.

