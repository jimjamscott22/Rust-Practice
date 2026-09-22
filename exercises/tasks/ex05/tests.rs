mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(exclaim(String::from("Rust")), "Rust!");
}
#[test]
fn edge_cases() {
    assert_eq!(exclaim(String::new()), "!");
}
