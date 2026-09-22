mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(sum_strings(" 2", "3 "), Ok(5));
    assert_eq!(sum_strings("4294967295", "1"), Ok(4_294_967_296));
}
#[test]
fn edge_cases() {
    assert!(sum_strings("no", "3").is_err());
    assert!(sum_strings("2", "-1").is_err());
}
