mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(smallest(&[3, -1, 5]), Some(&-1));
    let words = [String::from("z"), String::from("a")];
    assert_eq!(smallest(&words), Some(&words[1]));
}
#[test]
fn edge_cases() {
    assert_eq!(smallest::<i32>(&[]), None);
    assert_eq!(smallest(&[7]), Some(&7));
}
