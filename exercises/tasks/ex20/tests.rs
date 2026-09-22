mod starter;
use starter::*;
#[test]
fn normal_cases() {
    let r = Record::parse("7\tAda").unwrap();
    assert_eq!(r.id, 7);
    assert_eq!(r.format(), "7\tAda");
    assert_eq!(Record::parse("0\t 猫 ").unwrap().format(), "0\t 猫 ");
}
#[test]
fn edge_cases() {
    for bad in ["", "x\tAda", "1\t ", "1\ta\tb", "1\ta\n", "4294967296\tx"] {
        assert!(Record::parse(bad).is_err(), "{bad:?}");
    }
}
