mod starter;
use starter::*;
#[test]
fn normal_cases() {
    let text = String::from("café🦀");
    assert_eq!(char_count(&text), 5);
    assert_eq!(text, "café🦀");
}
#[test]
fn edge_cases() {
    assert_eq!(char_count(""), 0);
}
