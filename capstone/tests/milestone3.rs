use capstone::commands::{Command, parse};
fn args(input: &[&str]) -> Vec<String> {
    input.iter().map(|s| (*s).into()).collect()
}
#[test]
fn commands() {
    let o = parse(&args(&["list"])).unwrap();
    assert_eq!(o.file.to_str(), Some("tasks.tsv"));
    assert_eq!(o.command, Command::List);
    let o = parse(&args(&["--file", "my tasks.tsv", "add", "Learn Rust"])).unwrap();
    assert_eq!(o.file.to_str(), Some("my tasks.tsv"));
    assert_eq!(o.command, Command::Add("Learn Rust".into()));
    assert_eq!(
        parse(&args(&["done", "3"])).unwrap().command,
        Command::Done(3)
    );
}
#[test]
fn rejects_bad_arguments() {
    for input in [
        vec![],
        vec!["--file"],
        vec!["--file", "", "list"],
        vec!["add"],
        vec!["add", " "],
        vec!["add", "a\tb"],
        vec!["add", "two", "words"],
        vec!["done", "0"],
        vec!["done", "-1"],
        vec!["done", "4294967296"],
        vec!["list", "extra"],
        vec!["unknown"],
        vec!["list", "--file", "x"],
    ] {
        assert!(parse(&args(&input)).is_err(), "{input:?}");
    }
}
