mod starter;
use starter::*;
#[test]
fn normal_cases() {
    let c = word_counts("Rust Rust rust!");
    assert_eq!(c.get("Rust"), Some(&2));
    assert_eq!(c.get("rust!"), Some(&1));
    assert_eq!(c.len(), 2);
}
#[test]
fn edge_cases() {
    assert!(word_counts(" \t\n").is_empty());
    assert_eq!(word_counts("猫\n猫").get("猫"), Some(&2));
}
