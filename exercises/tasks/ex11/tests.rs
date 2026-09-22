mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(seconds(Wait::Seconds(7)), 7);
    assert_eq!(seconds(Wait::Minutes(2)), 120);
}
#[test]
fn edge_cases() {
    assert_eq!(seconds(Wait::Minutes(0)), 0);
    assert_eq!(seconds(Wait::Minutes(u32::MAX)), u64::from(u32::MAX) * 60);
}
