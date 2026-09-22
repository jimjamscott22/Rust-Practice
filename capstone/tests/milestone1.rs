use capstone::model::{Task, TaskList};
#[test]
fn empty_list_and_task_fields() {
    let tasks = TaskList::new();
    assert!(tasks.all().is_empty());
    let task = Task {
        id: 1,
        title: "Learn Rust".into(),
        done: false,
    };
    assert_eq!(task.id, 1);
    assert!(!task.done);
    assert!(TaskList::default().all().is_empty());
}
