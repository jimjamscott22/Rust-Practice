mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(longer("a", "rust"), "rust");
    assert_eq!(longer("🦀", "ab"), "ab");
}
#[test]
fn edge_cases() {
    assert_eq!(longer("ab", "cd"), "ab");
    assert_eq!(longer("", ""), "");
}
