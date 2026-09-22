use capstone::model::{TaskList, validate_title};
#[test]
fn operations() {
    let mut tasks = TaskList::new();
    assert_eq!(tasks.add("Read".into()).unwrap(), 1);
    assert_eq!(tasks.add("Write".into()).unwrap(), 2);
    tasks.complete(1).unwrap();
    tasks.complete(1).unwrap();
    assert!(tasks.all()[0].done);
    assert!(!tasks.all()[1].done);
    assert_eq!(tasks.add("Practice".into()).unwrap(), 3);
    assert!(tasks.complete(99).is_err());
    assert!(tasks.complete(0).is_err());
    assert_eq!(tasks.all().len(), 3);
}
#[test]
fn invalid_titles_do_not_consume_ids() {
    let mut tasks = TaskList::new();
    for title in ["", "  ", "a\tb", "a\nb", "a\rb"] {
        assert!(validate_title(title).is_err());
        assert!(tasks.add(title.into()).is_err());
    }
    assert_eq!(tasks.add(" 猫 ".into()).unwrap(), 1);
    assert_eq!(tasks.all()[0].title, " 猫 ");
}
