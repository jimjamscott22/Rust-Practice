mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert_eq!(greet("Ada"), "Hello, Ada!");
    assert_eq!(greet("世界"), "Hello, 世界!");
}
#[test]
fn edge_cases() {
    assert_eq!(greet(""), "Hello, !");
    assert_eq!(greet(" Ada "), "Hello,  Ada !");
}
