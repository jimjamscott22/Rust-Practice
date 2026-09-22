mod common;
use capstone::{
    model::{Task, TaskList},
    storage::*,
};
#[test]
fn round_trip_and_file_errors() {
    let sandbox = common::Sandbox::new();
    let path = sandbox.path().join("tasks.tsv");
    assert!(load(&path).unwrap().all().is_empty());
    let mut tasks = TaskList::new();
    tasks.add(" 猫 ".into()).unwrap();
    tasks.add("Read".into()).unwrap();
    tasks.complete(1).unwrap();
    assert_eq!(encode(&tasks), "1\t1\t 猫 \n2\t0\tRead\n");
    assert_eq!(decode(&encode(&tasks)).unwrap(), tasks);
    save(&path, &tasks).unwrap();
    assert_eq!(load(&path).unwrap(), tasks);
    tasks.add("Next".into()).unwrap();
    save(&path, &tasks).unwrap();
    assert_eq!(load(&path).unwrap().all().len(), 3);
    assert!(load(sandbox.path()).is_err());
    assert!(save(sandbox.path(), &tasks).is_err());
    assert!(save(&sandbox.path().join("absent/tasks.tsv"), &tasks).is_err());
    assert_eq!(load(&path).unwrap(), tasks);
}
#[test]
fn validates_loaded_data_and_ids() {
    for bad in [
        "\n",
        "x\t0\ttitle\n",
        "0\t0\ttitle\n",
        "1\t2\ttitle\n",
        "1\t0\t \n",
        "1\t0\ta\tb\n",
        "1\t0\ta\n1\t1\tb\n",
        "1\t0\ta\rb\n",
    ] {
        assert!(decode(bad).is_err(), "{bad:?}");
    }
    assert!(decode("").unwrap().all().is_empty());
    assert_eq!(decode("1\t0\ta\r\n").unwrap().all()[0].title, "a");
    let mut tasks = decode("7\t0\tb\n2\t1\ta\n").unwrap();
    assert_eq!(tasks.add("c".into()).unwrap(), 8);
    let mut full = TaskList::from_tasks(vec![Task {
        id: u32::MAX,
        title: "last".into(),
        done: false,
    }])
    .unwrap();
    assert!(full.add("overflow".into()).is_err());
    assert_eq!(full.all().len(), 1);
}
