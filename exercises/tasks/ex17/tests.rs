mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(even_squares(&[-4, 1, 2, 0]), vec![16, 4, 0]);
}
#[test]
fn edge_cases() {
    assert!(even_squares(&[]).is_empty());
    assert_eq!(even_squares(&[i32::MIN]), vec![4_611_686_018_427_387_904]);
}
