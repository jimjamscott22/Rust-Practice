mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(sum_to(5), 15);
    assert_eq!(sum_to(100_000), 5_000_050_000);
}
#[test]
fn edge_cases() {
    assert_eq!(sum_to(0), 0);
    assert_eq!(sum_to(1), 1);
}
