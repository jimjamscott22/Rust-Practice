mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(unique(&[3, 1, 3, 2, 1]), vec![3, 1, 2]);
}
#[test]
fn edge_cases() {
    assert_eq!(unique(&[]), Vec::<i32>::new());
    assert_eq!(unique(&[-1, -1, 0]), vec![-1, 0]);
}
