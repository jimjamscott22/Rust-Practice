mod starter;
use starter::*;
#[test]
fn normal_cases() {
    let u = new_user("Ada".into());
    assert_eq!(u.name, "Ada");
    assert!(u.active);
}
#[test]
fn edge_cases() {
    assert_eq!(new_user(String::new()).name, "");
}
