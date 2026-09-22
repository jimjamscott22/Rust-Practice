# ex20: Records

Implement Record::parse and format. Wire format is unsigned u32 ID, one tab, and a nonblank name. Reject additional tabs and CR/LF anywhere. Preserve name whitespace; ID uses standard u32 parsing. Return String errors; their wording is not tested.

## Interface

```rust
impl Record { pub fn parse(line: &str) -> Result<Self, String>
// Also implement pub fn format(&self) -> String
```

Edit only `starter.rs`. Supporting types, if any, are supplied there.

## Acceptance and examples

The following assertions illustrate the required behavior (full tests are in tests.rs):

```rust
let r = Record::parse("7\tAda").unwrap(); assert_eq!(r.id, 7); assert_eq!(r.format(), "7\tAda"); assert_eq!(Record::parse("0\t 猫 ").unwrap().format(), "0\t 猫 ");
for bad in ["", "x\tAda", "1\t ", "1\ta\tb", "1\ta\n", "4294967296\tx"] { assert!(Record::parse(bad).is_err(), "{bad:?}"); }
```

Run `cargo test -p exercises --test ex20`. Pass both tests and follow the learning constraints above. Tests cannot enforce your implementation technique; compare the reference explanation after solving.

Hints: [progressive hints](../../../../docs/hints.md#ex20). Optional answer: [reference](../../../../solutions/exercises/ex20.rs). Earlier exercises teach concepts but are not code dependencies.

