mod common;
use std::process::{Command, Output};
fn run(root: &std::path::Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_task-lab"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}
fn ok(root: &std::path::Path, args: &[&str]) -> String {
    let output = run(root, args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .replace("\r\n", "\n")
}
#[test]
fn separate_processes_persist_tasks() {
    let dir = common::Sandbox::new();
    let root = dir.path();
    assert_eq!(ok(root, &["list"]), "No tasks.\n");
    assert!(!root.join("tasks.tsv").exists());
    assert_eq!(ok(root, &["add", "Learn Rust"]), "Added task 1.\n");
    assert_eq!(ok(root, &["add", "Practice"]), "Added task 2.\n");
    assert_eq!(ok(root, &["done", "1"]), "Completed task 1.\n");
    assert_eq!(ok(root, &["list"]), "1 [x] Learn Rust\n2 [ ] Practice\n");
    assert_eq!(ok(root, &["--file", "other.tsv", "list"]), "No tasks.\n");
    assert_eq!(
        ok(root, &["--file", "other.tsv", "add", "Separate"]),
        "Added task 1.\n"
    );
    assert_eq!(
        ok(root, &["--file", "other.tsv", "list"]),
        "1 [ ] Separate\n"
    );
    let before = std::fs::read(root.join("tasks.tsv")).unwrap();
    for args in [
        vec!["done", "99"],
        vec!["add", " "],
        vec!["add", "x\ny"],
        vec!["unknown"],
    ] {
        let out = run(root, &args);
        assert_eq!(out.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&out.stderr).starts_with("Error: "));
        assert_eq!(std::fs::read(root.join("tasks.tsv")).unwrap(), before);
    }
}
#[test]
fn failures_preserve_data() {
    let dir = common::Sandbox::new();
    let root = dir.path();
    let path = root.join("tasks.tsv");
    std::fs::write(&path, "malformed\n").unwrap();
    for args in [vec!["add", "new"], vec!["done", "1"], vec!["list"]] {
        let out = run(root, &args);
        assert_eq!(out.status.code(), Some(1));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "malformed\n");
    }
    std::fs::write(&path, [0xff, 0xfe]).unwrap();
    assert_eq!(run(root, &["add", "new"]).status.code(), Some(1));
    assert_eq!(std::fs::read(&path).unwrap(), vec![0xff, 0xfe]);
    for args in [
        vec!["--file", ".", "list"],
        vec!["--file", "absent/tasks.tsv", "add", "new"],
    ] {
        let out = run(root, &args);
        assert_eq!(out.status.code(), Some(1));
        assert!(!out.stderr.is_empty());
    }
}
