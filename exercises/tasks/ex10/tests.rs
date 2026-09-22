mod starter;
use starter::*;
#[test]
fn normal_cases() {
    let r = Rectangle {
        width: 3,
        height: 4,
    };
    assert_eq!(r.area(), 12);
    assert_eq!(r.width, 3);
}
#[test]
fn edge_cases() {
    assert_eq!(
        Rectangle {
            width: 0,
            height: 5
        }
        .area(),
        0
    );
    assert_eq!(
        Rectangle {
            width: u32::MAX,
            height: 2
        }
        .area(),
        8_589_934_590
    );
}
