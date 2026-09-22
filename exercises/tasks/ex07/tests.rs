mod starter;
use starter::*;
#[test]
fn normal_cases() {
    let mut s = String::from("Rust");
    append(&mut s, " rocks");
    assert_eq!(s, "Rust rocks");
}
#[test]
fn edge_cases() {
    let mut s = String::new();
    append(&mut s, "🦀");
    append(&mut s, "");
    assert_eq!(s, "🦀");
}
