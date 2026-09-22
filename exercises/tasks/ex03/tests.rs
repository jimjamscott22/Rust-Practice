mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(classify(-4), "negative");
    assert_eq!(classify(5), "positive");
}
#[test]
fn edge_cases() {
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(i32::MIN), "negative");
}
