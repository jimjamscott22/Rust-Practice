mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(parse_count(" 42 "), Ok(42));
    assert_eq!(parse_count("0"), Ok(0));
}
#[test]
fn edge_cases() {
    assert!(parse_count("-1").is_err());
    assert!(parse_count("").is_err());
    assert!(parse_count("4294967296").is_err());
}
