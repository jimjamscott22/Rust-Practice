mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(first_even(&[3, -4, 8]), Some(-4));
    assert_eq!(first_even(&[0]), Some(0));
}
#[test]
fn edge_cases() {
    assert_eq!(first_even(&[]), None);
    assert_eq!(first_even(&[1, 3]), None);
}
