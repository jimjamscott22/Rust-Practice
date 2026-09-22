mod starter;
use starter::*;
#[test]
fn normal_cases() {
    let b = Book {
        title: "Rust".into(),
        author: "Ada".into(),
    };
    assert_eq!(b.describe(), "Rust by Ada");
}
#[test]
fn edge_cases() {
    let b = Book {
        title: "".into(),
        author: "猫".into(),
    };
    assert_eq!(b.describe(), " by 猫");
}
